use crate::dynamic::loader::{ConfigError, ConfigLoader};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Background task for polling Apollo configuration
pub struct ReloadTask {
    loader: Arc<dyn ConfigLoader>,
    poll_interval: Duration,
    current_version: Arc<AtomicU64>,
}

impl ReloadTask {
    /// Create a new reload task
    pub fn new(loader: Arc<dyn ConfigLoader>, poll_interval_secs: u64) -> Self {
        Self {
            loader,
            poll_interval: Duration::from_secs(poll_interval_secs),
            current_version: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Create a new reload task with initial version
    pub fn with_version(loader: Arc<dyn ConfigLoader>, poll_interval_secs: u64, initial_version: u64) -> Self {
        Self {
            loader,
            poll_interval: Duration::from_secs(poll_interval_secs),
            current_version: Arc::new(AtomicU64::new(initial_version)),
        }
    }

    /// Start the background polling task
    /// Returns a handle that can be used to stop the task
    pub fn start<F>(self, on_reload: F) -> ReloadTaskHandle
    where
        F: Fn() + Send + Sync + 'static,
    {
        let stopped = Arc::new(AtomicBool::new(false));
        let stopped_clone = Arc::clone(&stopped);

        let loader = self.loader;
        let poll_interval = self.poll_interval;
        let current_version = self.current_version;
        let on_reload = Arc::new(on_reload);

        tokio::spawn(async move {
            loop {
                if stopped_clone.load(Ordering::Acquire) {
                    break;
                }

                sleep(poll_interval).await;

                // Run the blocking check in a blocking thread pool
                let loader_clone = Arc::clone(&loader);
                let version_clone = Arc::clone(&current_version);
                let on_reload_clone = Arc::clone(&on_reload);

                let result = tokio::task::spawn_blocking(move || {
                    check_and_reload(&loader_clone, &version_clone, on_reload_clone.as_ref())
                })
                .await;

                match result {
                    Ok(Ok(())) => {},
                    Ok(Err(e)) => log::warn!("Reload check failed: {:?}", e),
                    Err(e) => log::error!("Reload task panicked: {:?}", e),
                }
            }
        });

        ReloadTaskHandle { stopped }
    }
}

/// Check if configuration has changed and trigger reload
fn check_and_reload<F>(
    loader: &Arc<dyn ConfigLoader>,
    current_version: &Arc<AtomicU64>,
    on_reload: &F,
) -> Result<(), ConfigError>
where
    F: Fn() + Send + Sync,
{
    // Check if loader is available
    if !loader.is_available() {
        return Ok(());
    }

    // Load the latest rules
    let new_rules = loader.load()?;
    let stored_version = current_version.load(Ordering::Acquire);

    // Only trigger reload if version has changed
    if new_rules.version != stored_version {
        log::info!(
            "Configuration version changed: {} -> {}",
            stored_version,
            new_rules.version
        );
        on_reload();
        current_version.store(new_rules.version, Ordering::Release);
    }

    Ok(())
}

/// Handle to control the reload task
pub struct ReloadTaskHandle {
    stopped: Arc<AtomicBool>,
}

impl ReloadTaskHandle {
    /// Stop the reload task
    pub fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
    }

    /// Check if the task has been stopped
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::Acquire)
    }
}

impl Drop for ReloadTaskHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic::rules::{DynamicRuleSet, RuleSetMetadata};
    use std::sync::atomic::AtomicUsize;

    struct MockLoader {
        version: u64,
        available: bool,
    }

    impl ConfigLoader for MockLoader {
        fn load(&self) -> Result<crate::dynamic::rules::DynamicRuleSet, ConfigError> {
            if !self.available {
                return Err(ConfigError::NotConfigured("Mock not available".to_string()));
            }
            Ok(DynamicRuleSet {
                version: self.version,
                metadata: RuleSetMetadata {
                    name: "test".to_string(),
                    locale: "en".to_string(),
                    description: None,
                },
                rules: vec![],
            })
        }

        fn is_available(&self) -> bool {
            self.available
        }

        fn source_name(&self) -> &str {
            "mock"
        }
    }

    #[test]
    fn test_reload_task_handle_stop() {
        let handle = ReloadTaskHandle {
            stopped: Arc::new(AtomicBool::new(false)),
        };

        assert!(!handle.is_stopped());
        handle.stop();
        assert!(handle.is_stopped());
    }

    #[test]
    fn test_reload_task_handle_drop() {
        let stopped = Arc::new(AtomicBool::new(false));
        let stopped_clone = Arc::clone(&stopped);

        {
            let _handle = ReloadTaskHandle { stopped };
            assert!(!stopped_clone.load(Ordering::Acquire));
        }
        // Handle dropped, should call stop()
        assert!(stopped_clone.load(Ordering::Acquire));
    }

    #[tokio::test]
    async fn test_reload_task_lifecycle() {
        let loader = Arc::new(MockLoader {
            version: 1,
            available: true,
        });

        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = Arc::clone(&call_count);

        let task = ReloadTask::new(loader, 1); // 1 second poll interval
        let handle = task.start(move || {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Wait a bit to ensure task is running
        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(!handle.is_stopped());

        // Stop the task
        handle.stop();
        assert!(handle.is_stopped());

        // Wait to ensure task has stopped
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Since the task was stopped immediately, reload should not have been called
        // (first poll happens after poll_interval)
        let count = call_count.load(Ordering::SeqCst);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_check_and_reload_version_unchanged() {
        let loader: Arc<dyn ConfigLoader> = Arc::new(MockLoader {
            version: 1,
            available: true,
        });
        let current_version = Arc::new(AtomicU64::new(1));
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = Arc::clone(&call_count);

        let result = check_and_reload(&loader, &current_version, &|| {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(result.is_ok());
        // Version unchanged, reload should not be called
        assert_eq!(call_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_check_and_reload_version_changed() {
        let loader: Arc<dyn ConfigLoader> = Arc::new(MockLoader {
            version: 2,
            available: true,
        });
        let current_version = Arc::new(AtomicU64::new(1));
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = Arc::clone(&call_count);

        let result = check_and_reload(&loader, &current_version, &|| {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(result.is_ok());
        // Version changed, reload should be called once
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        // Version should be updated
        assert_eq!(current_version.load(Ordering::Acquire), 2);
    }

    #[test]
    fn test_check_and_reload_loader_unavailable() {
        let loader: Arc<dyn ConfigLoader> = Arc::new(MockLoader {
            version: 1,
            available: false,
        });
        let current_version = Arc::new(AtomicU64::new(1));
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = Arc::clone(&call_count);

        let result = check_and_reload(&loader, &current_version, &|| {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(result.is_ok());
        // Loader unavailable, reload should not be called
        assert_eq!(call_count.load(Ordering::SeqCst), 0);
    }
}

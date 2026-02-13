use crate::dynamic::{ConfigError, ConfigLoader};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Background task for polling Apollo configuration
pub struct ReloadTask {
    loader: Arc<dyn ConfigLoader>,
    poll_interval: Duration,
}

impl ReloadTask {
    /// Create a new reload task
    pub fn new(loader: Arc<dyn ConfigLoader>, poll_interval_secs: u64) -> Self {
        Self {
            loader,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    /// Start the background polling task
    /// Returns a handle that can be used to stop the task
    pub fn start<F>(self, on_reload: F) -> ReloadTaskHandle
    where
        F: Fn() + Send + Sync + 'static,
    {
        let handle = ReloadTaskHandle::new();

        let loader = Arc::new(self.loader);
        let poll_interval = self.poll_interval;
        let on_reload = Arc::new(on_reload);

        // Spawn background task
        tokio::spawn(async move {
            loop {
                sleep(poll_interval).await;

                if let Err(e) = self.check_and_reload(&loader, &on_reload) {
                    log::warn!("Reload check failed: {:?}", e);
                }

                if handle.is_stopped() {
                    break;
                }
            }
        });

        handle
    }

    fn check_and_reload(
        &self,
        loader: &Arc<dyn ConfigLoader>,
        on_reload: &Arc<dyn Fn() + Send + Sync>,
    ) -> Result<(), ConfigError> {
        // Check if configuration has changed
        // In a real implementation, this would use Apollo's notification API
        // For now, we trigger a reload
        on_reload();
        Ok(())
    }
}

/// Handle to control the reload task
pub struct ReloadTaskHandle {
    stopped: std::sync::atomic::AtomicBool,
}

impl ReloadTaskHandle {
    fn new() -> Self {
        Self {
            stopped: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Stop the reload task
    pub fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn is_stopped(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::SeqCst)
    }
}

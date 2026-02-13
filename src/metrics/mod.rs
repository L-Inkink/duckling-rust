//! Performance metrics module
//!
//! Provides utilities for tracking parsing performance and resource usage.

pub mod timing;

use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::collections::HashMap;

/// Timing scope for measuring execution time
pub struct TimingScope<'a> {
    name: String,
    start: Instant,
    metrics: &'a Metrics,
}

impl<'a> TimingScope<'a> {
    pub fn new(name: &str, metrics: &'a Metrics) -> Self {
        Self {
            name: name.to_string(),
            start: Instant::now(),
            metrics,
        }
    }
}

impl<'a> Drop for TimingScope<'a> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.metrics.record_timing(&self.name, elapsed);
    }
}

/// Performance metrics collector
pub struct Metrics {
    /// Total number of parse requests
    total_requests: AtomicU64,
    /// Total number of successful parses
    successful_parses: AtomicU64,
    /// Total number of failed parses
    failed_parses: AtomicU64,
    /// Timing data: name -> (total_duration_ns, count)
    timings: RwLock<HashMap<String, (AtomicU64, AtomicU64)>>,
    /// Peak memory usage estimate (in bytes)
    peak_memory: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_parses: AtomicU64::new(0),
            failed_parses: AtomicU64::new(0),
            timings: RwLock::new(HashMap::new()),
            peak_memory: AtomicU64::new(0),
        }
    }

    /// Record a parse request
    pub fn record_request(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a successful parse
    pub fn record_success(&self) {
        self.successful_parses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a failed parse
    pub fn record_failure(&self) {
        self.failed_parses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record timing for an operation
    pub fn record_timing(&self, name: &str, duration: Duration) {
        let mut timings = self.timings.write().unwrap();
        let entry = timings
            .entry(name.to_string())
            .or_insert_with(|| (AtomicU64::new(0), AtomicU64::new(0)));
        entry.0.fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        entry.1.fetch_add(1, Ordering::Relaxed);
    }

    /// Update peak memory estimate
    pub fn update_memory(&self, bytes: u64) {
        let current = self.peak_memory.load(Ordering::Relaxed);
        if bytes > current {
            self.peak_memory.store(bytes, Ordering::Relaxed);
        }
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let timings = self.timings.read().unwrap();
        let timing_data: HashMap<String, TimingData> = timings
            .iter()
            .map(|(k, (total, count))| {
                let total_ns = total.load(Ordering::Relaxed);
                let cnt = count.load(Ordering::Relaxed) as u64;
                (
                    k.clone(),
                    TimingData {
                        total_ns,
                        count: cnt,
                        avg_ns: if cnt > 0 { total_ns / cnt } else { 0 },
                    },
                )
            })
            .collect();

        MetricsSnapshot {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            successful_parses: self.successful_parses.load(Ordering::Relaxed),
            failed_parses: self.failed_parses.load(Ordering::Relaxed),
            timings: timing_data,
            peak_memory_bytes: self.peak_memory.load(Ordering::Relaxed),
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_parses.store(0, Ordering::Relaxed);
        self.failed_parses.store(0, Ordering::Relaxed);
        self.peak_memory.store(0, Ordering::Relaxed);
        if let Ok(mut timings) = self.timings.write() {
            timings.clear();
        }
    }

    /// Create a timing scope for measuring
    pub fn scope(&self, name: &str) -> TimingScope<'_> {
        TimingScope::new(name, self)
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of metrics at a point in time
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub successful_parses: u64,
    pub failed_parses: u64,
    pub timings: HashMap<String, TimingData>,
    pub peak_memory_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct TimingData {
    pub total_ns: u64,
    pub count: u64,
    pub avg_ns: u64,
}

impl MetricsSnapshot {
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 100.0;
        }
        (self.successful_parses as f64 / self.total_requests as f64) * 100.0
    }

    /// Get average parse time in microseconds
    pub fn avg_parse_time_us(&self) -> f64 {
        self.timings
            .get("parse")
            .map(|t| t.avg_ns as f64 / 1000.0)
            .unwrap_or(0.0)
    }
}

/// Global metrics instance
pub static METRICS: std::sync::LazyLock<Metrics> = std::sync::LazyLock::new(Metrics::new);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_snapshot() {
        let snapshot = METRICS.snapshot();
        assert_eq!(snapshot.total_requests, 0);
    }

    #[test]
    fn test_timing_scope() {
        METRICS.reset();
        {
            let _scope = METRICS.scope("test_op");
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
        let snapshot = METRICS.snapshot();
        assert!(snapshot.timings.contains_key("test_op"));
    }
}

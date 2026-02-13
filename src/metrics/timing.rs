//! Timing utilities for module instrumentation
//!
//! Provides easy-to-use timing wrappers for measuring module performance.

pub use crate::metrics::{METRICS, Metrics, MetricsSnapshot, TimingScope};

/// Convenience function to time an operation
#[macro_export]
macro_rules! time_scope {
    ($name:expr) => {
        let _scope = $crate::metrics::METRICS.scope($name);
    };
}

/// Convenience macro to time a block of code
#[macro_export]
macro_rules! time_block {
    ($name:expr, $block:block) => {
        {
            let _scope = $crate::metrics::METRICS.scope($name);
            $block
        }
    };
}

pub use crate::metrics::TimingData;

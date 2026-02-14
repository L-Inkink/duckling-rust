//! HTTP Server module for rustling
//!
//! Provides REST API for:
//! - Text parsing
//! - Configuration management
//! - Health checks

pub mod app;
pub mod handlers;
pub mod reload;
pub mod state;

pub use app::ServerBuilder;
pub use reload::{ReloadTask, ReloadTaskHandle};
pub use state::AppState;

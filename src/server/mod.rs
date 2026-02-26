//! HTTP Server module for rustling
//!
//! Provides REST API for:
//! - Text parsing
//! - Configuration management
//! - Health checks

pub mod app;
pub mod docs;
pub mod handlers;
pub mod reload;
pub mod state;

#[cfg(feature = "grpc")]
pub mod grpc;

pub use app::ServerBuilder;
pub use docs::ApiDoc;
pub use reload::{ReloadTask, ReloadTaskHandle};
pub use state::AppState;

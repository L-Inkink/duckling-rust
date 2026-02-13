//! HTTP Server module for rustling
//!
//! Provides REST API for:
//! - Text parsing
//! - Configuration management
//! - Health checks

pub mod handlers;
pub mod state;

pub use state::AppState;

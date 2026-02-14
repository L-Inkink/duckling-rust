//! Example: Running the HTTP Server
//!
//! This example demonstrates how to start the rustling HTTP server with
//! static rules and optional Apollo hot-reload support.
//!
//! ## Usage
//!
//! Basic usage (static rules only):
//! ```bash
//! cargo run --example http_server
//! ```
//!
//! With hot-reload enabled:
//! ```bash
//! RELOAD_API_KEY=your-secret-key cargo run --example http_server
//! ```
//!
//! Custom bind address:
//! ```bash
//! BIND_ADDRESS=0.0.0.0:3000 cargo run --example http_server
//! ```
//!
//! ## API Endpoints
//!
//! - `GET /health` - Health check
//! - `GET /config/status` - Configuration status
//! - `POST /parse` - Parse text (max 10KB)
//! - `POST /config/reload` - Reload configuration (requires API key)
//!
//! ## Example Requests
//!
//! Health check:
//! ```bash
//! curl http://localhost:8080/health
//! ```
//!
//! Parse text:
//! ```bash
//! curl -X POST http://localhost:8080/parse \
//!   -H "Content-Type: application/json" \
//!   -d '{"text": "I need 5 minutes"}'
//! ```
//!
//! Check config status:
//! ```bash
//! curl http://localhost:8080/config/status
//! ```
//!
//! Reload config (requires API key):
//! ```bash
//! curl -X POST http://localhost:8080/config/reload \
//!   -H "X-API-Key: your-secret-key"
//! ```
//!
//! ## Environment Variables
//!
//! - `BIND_ADDRESS` - Server bind address (default: 127.0.0.1:8080)
//! - `RELOAD_API_KEY` - API key for config reload endpoint (required for reload)
//! - `RUST_LOG` - Log level (e.g., info, debug, warn)
//!
//! ## Logging
//!
//! Enable debug logging:
//! ```bash
//! RUST_LOG=debug cargo run --example http_server
//! ```

use actix_web::{middleware, App, HttpServer};
use rustling::server::ServerBuilder;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Read configuration from environment
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let reload_key = env::var("RELOAD_API_KEY").ok();

    // Print startup info
    log::info!("Starting rustling HTTP server");
    log::info!("Bind address: {}", bind_address);
    log::info!(
        "Reload API key: {}",
        if reload_key.is_some() {
            "configured ✓"
        } else {
            "not set (reload endpoint will be unavailable)"
        }
    );

    log::info!("Server configuration:");
    log::info!("  - Static rules: enabled");
    log::info!("  - Dynamic rules: not configured");
    log::info!("  - Payload limit: 64KB");
    log::info!("  - Text limit: 10KB");

    // Start HTTP server
    log::info!("🚀 Server starting at http://{}", bind_address);
    log::info!("📚 API endpoints:");
    log::info!("  GET  /health         - Health check");
    log::info!("  GET  /config/status  - Configuration status");
    log::info!("  POST /parse          - Parse text");
    log::info!("  POST /config/reload  - Reload configuration (requires API key)");

    HttpServer::new(|| {
        // Create server builder and configure routes
        let builder = ServerBuilder::new();

        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Configure routes
            .configure(builder.configure())
    })
    .bind(&bind_address)?
    .run()
    .await
}

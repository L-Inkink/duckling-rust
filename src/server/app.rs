use actix_web::web;

use super::handlers::{config_reload, config_status, health, parse};
use super::AppState;

/// Server builder for configuring and creating the HTTP server
pub struct ServerBuilder {
    bind_address: String,
    #[allow(dead_code)]
    state: AppState,
}

impl ServerBuilder {
    /// Create a new server builder
    pub fn new() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".to_string(),
            state: AppState::static_only(),
        }
    }

    /// Set the bind address
    pub fn bind(mut self, address: impl Into<String>) -> Self {
        self.bind_address = address.into();
        self
    }

    /// Configure routes for the application
    /// This returns a closure suitable for use with HttpServer::new():
    ///
    /// ```ignore
    /// let builder = ServerBuilder::new().bind("127.0.0.1:8080");
    /// HttpServer::new(builder.configure())
    ///     .bind(builder.bind_address())?
    ///     .run()
    /// ```
    pub fn configure(&self) -> impl Fn(&mut web::ServiceConfig) {
        let state = self.state.clone();
        move |cfg: &mut web::ServiceConfig| {
            cfg.app_data(web::Data::new(state.clone()));
            // Configure JSON payload limit to prevent large request attacks
            cfg.app_data(web::JsonConfig::default().limit(65_536)); // 64KB max
            cfg.service(
                web::resource("/health")
                    .route(web::get().to(health))
            );
            cfg.service(
                web::resource("/parse")
                    .route(web::post().to(parse))
            );
            cfg.service(
                web::resource("/config/status")
                    .route(web::get().to(config_status))
            );
            cfg.service(
                web::resource("/config/reload")
                    .route(web::post().to(config_reload))
            );
        }
    }

    /// Get the bind address
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

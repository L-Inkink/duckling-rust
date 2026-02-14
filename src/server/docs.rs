//! OpenAPI documentation for rustling HTTP server

use utoipa::OpenApi;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

use super::handlers::{
    BatchParseRequest, BatchParseResponse, BatchParseResult,
    ConfigStatusResponse, HealthResponse, ParseRequest,
    ParseResponse, ParseResult,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::handlers::parse,
        super::handlers::parse_batch,
        super::handlers::health,
        super::handlers::config_status,
        super::handlers::config_reload,
    ),
    components(
        schemas(
            ParseRequest,
            ParseResponse,
            ParseResult,
            BatchParseRequest,
            BatchParseResponse,
            BatchParseResult,
            HealthResponse,
            ConfigStatusResponse,
        )
    ),
    tags(
        (name = "Parse", description = "Text parsing endpoints"),
        (name = "Health", description = "Health check endpoints"),
        (name = "Configuration", description = "Configuration management endpoints"),
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Rustling HTTP API",
        version = env!("CARGO_PKG_VERSION"),
        description = "REST API for the rustling natural language parsing library",
        contact(
            name = "API Support",
            url = "https://github.com/snipsco/rustling"
        )
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
            )
        }
    }
}

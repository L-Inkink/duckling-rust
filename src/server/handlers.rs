use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResponse {
    pub results: Vec<ParseResult>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResult {
    pub value: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub char_start: usize,
    pub char_end: usize,
}

/// Parse text and return all matches
pub async fn parse(
    state: web::Data<AppState>,
    req: web::Json<ParseRequest>,
) -> Result<impl Responder, actix_web::Error> {
    // Validate input length to prevent memory exhaustion attacks
    const MAX_TEXT_LEN: usize = 10_000;
    if req.text.len() > MAX_TEXT_LEN {
        return Err(actix_web::error::ErrorBadRequest(
            format!("Text exceeds maximum length of {} bytes", MAX_TEXT_LEN)
        ));
    }

    let text = req.text.clone();
    let state_clone = state.clone();

    // Move both normalization and parsing to blocking thread pool
    // (both are CPU-bound operations)
    let results = web::block(move || {
        // Normalize text for fuzzy matching
        let normalized = state_clone.pattern_normalizer.normalize(&text);

        // Apply rules (requires read lock on rule_set)
        // Note: rule_set.apply_all returns Rc<Node> which is not Send,
        // so we must complete parsing and extract results within this block
        let rule_set_guard = state_clone.rule_set.read()
            .map_err(|_| "Failed to acquire read lock on rule_set")?;

        let nodes = rule_set_guard.apply_all(&normalized)
            .map_err(|_| "Failed to parse input text")?;

        // Extract results while still in the blocking thread
        let results: Vec<ParseResult> = nodes
            .iter()
            .map(|n| {
                let byte_range = n.root_node.byte_range;
                let char_range = byte_range.char_range(&text);
                ParseResult {
                    value: format!("{:?}", n.value),
                    byte_start: byte_range.0,
                    byte_end: byte_range.1,
                    char_start: char_range.0,
                    char_end: char_range.1,
                }
            })
            .collect();

        Ok::<_, String>(results)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Parse operation failed"))?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let count = results.len();
    Ok(HttpResponse::Ok().json(ParseResponse { results, count }))
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub dynamic_enabled: bool,
}

/// Get health status
pub async fn health(state: web::Data<AppState>) -> impl Responder {
    let dynamic_enabled = state.dynamic_enabled;

    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        dynamic_enabled,
    })
}

/// Configuration status response
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigStatusResponse {
    pub dynamic_enabled: bool,
    pub current_version: u64,
    pub source: String,
}

/// Get configuration status
pub async fn config_status(
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let state_clone = state.clone();
    let (current_version, has_dynamic) = web::block(move || {
        let config = state_clone
            .config_manager
            .lock()
            .map_err(|e| format!("Mutex poisoned: {}", e))?;
        Ok::<_, String>((config.current_version(), config.has_dynamic_rules()))
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get config"))?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(ConfigStatusResponse {
        dynamic_enabled: state.dynamic_enabled,
        current_version,
        source: if has_dynamic {
            "apollo".to_string()
        } else {
            "static".to_string()
        },
    }))
}

/// Trigger configuration reload (requires API key authentication)
pub async fn config_reload(
    state: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    // Authenticate using API key from environment variable
    let expected_key = std::env::var("RELOAD_API_KEY")
        .map_err(|_| actix_web::error::ErrorForbidden("Reload endpoint not configured"))?;

    let provided_key = req
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided_key != expected_key {
        log::warn!("Unauthorized reload attempt");
        return Err(actix_web::error::ErrorForbidden("Invalid API key"));
    }

    let state_clone = state.clone();

    let result = web::block(move || state_clone.reload_rules())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to reload config"))?;

    match result {
        Ok(rules) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "reloaded",
            "version": rules.version,
            "rule_count": rules.rules.len(),
        }))),
        Err(e) => {
            log::error!("Config reload failed: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Configuration reload failed",
            })))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_endpoint() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/health", web::get().to(health))
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "healthy");
        assert!(!body.dynamic_enabled);
    }

    #[actix_web::test]
    async fn test_parse_endpoint_success() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse", web::post().to(parse))
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(ParseRequest {
                text: "42".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: ParseResponse = test::read_body_json(resp).await;
        assert!(body.count > 0);
    }

    #[actix_web::test]
    async fn test_parse_endpoint_text_too_long() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse", web::post().to(parse))
        )
        .await;

        let long_text = "a".repeat(10_001); // Exceeds MAX_TEXT_LEN
        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(ParseRequest { text: long_text })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400); // Bad Request
    }

    #[actix_web::test]
    async fn test_config_status_endpoint() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/config/status", web::get().to(config_status))
        )
        .await;

        let req = test::TestRequest::get().uri("/config/status").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: ConfigStatusResponse = test::read_body_json(resp).await;
        assert!(!body.dynamic_enabled);
        assert_eq!(body.source, "static");
    }

    #[actix_web::test]
    async fn test_config_reload_unauthorized() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/config/reload", web::post().to(config_reload))
        )
        .await;

        // No API key
        let req = test::TestRequest::post()
            .uri("/config/reload")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403); // Forbidden
    }

    #[actix_web::test]
    async fn test_config_reload_invalid_api_key() {
        std::env::set_var("RELOAD_API_KEY", "correct-key");

        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/config/reload", web::post().to(config_reload))
        )
        .await;

        // Wrong API key
        let req = test::TestRequest::post()
            .uri("/config/reload")
            .insert_header(("X-API-Key", "wrong-key"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403); // Forbidden

        std::env::remove_var("RELOAD_API_KEY");
    }
}

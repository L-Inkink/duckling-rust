use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParseRequest {
    #[schema(example = "I need 5 minutes")]
    pub text: String,
    /// BCP-47 locale code, e.g. "en", "fr", "zh".
    /// Missing or unsupported locale returns empty results.
    #[schema(example = "en")]
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParseResponse {
    /// Parse results
    pub results: Vec<ParseResult>,
    /// Total number of results
    #[schema(example = 1)]
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchParseRequest {
    /// List of texts to parse (max 100 items, 10KB each)
    #[schema(example = json!(["5 minutes", "3 hours", "tomorrow"]))]
    pub texts: Vec<String>,
    /// BCP-47 locale code. Missing or unsupported returns empty results for all items.
    #[schema(example = "fr")]
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchParseResponse {
    /// Batch parse results
    pub results: Vec<BatchParseResult>,
    /// Total number of results across all texts
    #[schema(example = 3)]
    pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchParseResult {
    /// Index of the text in the request
    #[schema(example = 0)]
    pub index: usize,
    /// Original input text
    #[schema(example = "5 minutes")]
    pub text: String,
    /// Parse results for this text
    pub results: Vec<ParseResult>,
    /// Number of results for this text
    #[schema(example = 1)]
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParseResult {
    /// Parsed value representation
    #[schema(example = "Duration(5, Minute)")]
    pub value: String,
    /// Byte start position
    #[schema(example = 0)]
    pub byte_start: usize,
    /// Byte end position
    #[schema(example = 9)]
    pub byte_end: usize,
    /// Character start position
    #[schema(example = 0)]
    pub char_start: usize,
    /// Character end position
    #[schema(example = 9)]
    pub char_end: usize,
}

/// Parse text and return all matches
#[utoipa::path(
    post,
    path = "/parse",
    request_body = ParseRequest,
    responses(
        (status = 200, description = "Parse successful", body = ParseResponse),
        (status = 400, description = "Invalid request (text too long)")
    ),
    tag = "Parse"
)]
pub async fn parse(
    state: web::Data<AppState>,
    req: web::Json<ParseRequest>,
    http_req: actix_web::HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    const MAX_TEXT_LEN: usize = 10_000;
    if req.text.len() > MAX_TEXT_LEN {
        return Err(actix_web::error::ErrorBadRequest(
            format!("Text exceeds maximum length of {} bytes", MAX_TEXT_LEN)
        ));
    }

    let request_id = http_req
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let rule_set = match &req.locale {
        None => {
            log::warn!("[req={}] locale missing in request, returning empty results", request_id);
            return Ok(empty_response(&request_id));
        }
        Some(locale) => match state.locales.get(locale) {
            None => {
                log::warn!("[req={}] unsupported locale {:?}, returning empty results", request_id, locale);
                return Ok(empty_response(&request_id));
            }
            Some(rs) => Arc::clone(rs),
        }
    };

    let text = req.text.clone();
    let state_clone = state.clone();
    let request_id_clone = request_id.clone();

    let results = web::block(move || {
        let normalized = state_clone.pattern_normalizer.normalize(&text);

        let nodes = rule_set.apply_all(&normalized)
            .map_err(|_| "Failed to parse input text")?;

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

        log::info!("[req={}] matched={} results", request_id_clone, results.len());
        Ok::<_, String>(results)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Parse operation failed"))?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let count = results.len();
    Ok(HttpResponse::Ok()
        .insert_header(("X-Request-ID", request_id.as_str()))
        .json(ParseResponse { results, count }))
}

/// Build an empty 200 response with X-Request-ID header.
fn empty_response(request_id: &str) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("X-Request-ID", request_id))
        .json(ParseResponse { results: vec![], count: 0 })
}

/// Parse multiple texts in batch
#[utoipa::path(
    post,
    path = "/parse/batch",
    request_body = BatchParseRequest,
    responses(
        (status = 200, description = "Batch parse successful", body = BatchParseResponse),
        (status = 400, description = "Invalid request (batch too large or text too long)")
    ),
    tag = "Parse"
)]
pub async fn parse_batch(
    state: web::Data<AppState>,
    req: web::Json<BatchParseRequest>,
    http_req: actix_web::HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    const MAX_BATCH_SIZE: usize = 100;
    if req.texts.len() > MAX_BATCH_SIZE {
        return Err(actix_web::error::ErrorBadRequest(
            format!("Batch size exceeds maximum of {} items", MAX_BATCH_SIZE)
        ));
    }

    const MAX_TEXT_LEN: usize = 10_000;
    for (idx, text) in req.texts.iter().enumerate() {
        if text.len() > MAX_TEXT_LEN {
            return Err(actix_web::error::ErrorBadRequest(
                format!("Text at index {} exceeds maximum length of {} bytes", idx, MAX_TEXT_LEN)
            ));
        }
    }

    let request_id = http_req
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Resolve locale → rule set (same logic as single parse)
    let rule_set = match &req.locale {
        None => {
            log::warn!("[req={}] locale missing in batch request, returning empty results", request_id);
            let empty: Vec<BatchParseResult> = req.texts.iter().enumerate()
                .map(|(i, t)| BatchParseResult { index: i, text: t.clone(), results: vec![], count: 0 })
                .collect();
            return Ok(HttpResponse::Ok()
                .insert_header(("X-Request-ID", request_id.as_str()))
                .json(BatchParseResponse { results: empty, total_count: 0 }));
        }
        Some(locale) => match state.locales.get(locale) {
            None => {
                log::warn!("[req={}] unsupported locale {:?} in batch request", request_id, locale);
                let empty: Vec<BatchParseResult> = req.texts.iter().enumerate()
                    .map(|(i, t)| BatchParseResult { index: i, text: t.clone(), results: vec![], count: 0 })
                    .collect();
                return Ok(HttpResponse::Ok()
                    .insert_header(("X-Request-ID", request_id.as_str()))
                    .json(BatchParseResponse { results: empty, total_count: 0 }));
            }
            Some(rs) => Arc::clone(rs),
        }
    };

    let texts = req.texts.clone();
    let state_clone = state.clone();
    let request_id_clone = request_id.clone();

    let batch_results = web::block(move || {
        let mut results = Vec::with_capacity(texts.len());

        for (index, text) in texts.iter().enumerate() {
            let normalized = state_clone.pattern_normalizer.normalize(text);

            let nodes = rule_set.apply_all(&normalized)
                .map_err(|_| format!("Failed to parse text at index {}", index))?;

            let parse_results: Vec<ParseResult> = nodes
                .iter()
                .map(|n| {
                    let byte_range = n.root_node.byte_range;
                    let char_range = byte_range.char_range(text);
                    ParseResult {
                        value: format!("{:?}", n.value),
                        byte_start: byte_range.0,
                        byte_end: byte_range.1,
                        char_start: char_range.0,
                        char_end: char_range.1,
                    }
                })
                .collect();

            let count = parse_results.len();
            results.push(BatchParseResult {
                index,
                text: text.clone(),
                results: parse_results,
                count,
            });
        }

        log::info!("[req={}] batch matched {} total results across {} texts",
            request_id_clone,
            results.iter().map(|r| r.count).sum::<usize>(),
            results.len());
        Ok::<_, String>(results)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Batch parse operation failed"))?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let total_count = batch_results.iter().map(|r| r.count).sum();

    Ok(HttpResponse::Ok()
        .insert_header(("X-Request-ID", request_id.as_str()))
        .json(BatchParseResponse {
            results: batch_results,
            total_count,
        }))
}

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Health status
    #[schema(example = "healthy")]
    pub status: String,
    /// Server version
    #[schema(example = "0.10.0")]
    pub version: String,
    /// Whether dynamic rules are enabled
    #[schema(example = false)]
    pub dynamic_enabled: bool,
}

/// Get health status
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is healthy", body = HealthResponse)
    ),
    tag = "Health"
)]
pub async fn health(state: web::Data<AppState>) -> impl Responder {
    let dynamic_enabled = state.dynamic_enabled;

    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        dynamic_enabled,
    })
}

/// Configuration status response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConfigStatusResponse {
    /// Whether dynamic rules are enabled
    #[schema(example = false)]
    pub dynamic_enabled: bool,
    /// Current configuration version
    #[schema(example = 0)]
    pub current_version: u64,
    /// Configuration source
    #[schema(example = "static")]
    pub source: String,
}

/// Get configuration status
#[utoipa::path(
    get,
    path = "/config/status",
    responses(
        (status = 200, description = "Configuration status retrieved", body = ConfigStatusResponse),
        (status = 500, description = "Failed to get configuration")
    ),
    tag = "Configuration"
)]
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
#[utoipa::path(
    post,
    path = "/config/reload",
    responses(
        (status = 200, description = "Configuration reloaded successfully"),
        (status = 403, description = "Invalid or missing API key"),
        (status = 500, description = "Reload failed")
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Configuration"
)]
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
                locale: Some("en".to_string()),
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
            .set_json(ParseRequest { text: long_text, locale: Some("en".to_string()) })
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

    #[actix_web::test]
    async fn test_parse_batch_endpoint_success() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse/batch", web::post().to(parse_batch))
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/parse/batch")
            .set_json(BatchParseRequest {
                texts: vec!["42".to_string(), "5 minutes".to_string()],
                locale: Some("en".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: BatchParseResponse = test::read_body_json(resp).await;
        assert_eq!(body.results.len(), 2);
        assert!(body.total_count > 0);
    }

    #[actix_web::test]
    async fn test_parse_batch_too_many_items() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse/batch", web::post().to(parse_batch))
        )
        .await;

        let large_batch: Vec<String> = (0..101).map(|i| i.to_string()).collect();
        let req = test::TestRequest::post()
            .uri("/parse/batch")
            .set_json(BatchParseRequest { texts: large_batch, locale: Some("en".to_string()) })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400); // Bad Request
    }

    #[actix_web::test]
    async fn test_parse_batch_text_too_long() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse/batch", web::post().to(parse_batch))
        )
        .await;

        let long_text = "a".repeat(10_001);
        let req = test::TestRequest::post()
            .uri("/parse/batch")
            .set_json(BatchParseRequest {
                texts: vec!["valid".to_string(), long_text],
                locale: Some("en".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400); // Bad Request
    }

    #[actix_web::test]
    async fn test_parse_with_locale_fr() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse", web::post().to(parse))
        ).await;

        let req = test::TestRequest::post()
            .uri("/parse")
            .insert_header(("X-Request-ID", "test-123"))
            .set_json(ParseRequest {
                text: "42".to_string(),
                locale: Some("fr".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(
            resp.headers().get("X-Request-ID").unwrap(),
            "test-123"
        );
    }

    #[actix_web::test]
    async fn test_parse_missing_locale_returns_empty() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse", web::post().to(parse))
        ).await;

        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(serde_json::json!({"text": "tomorrow"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: ParseResponse = test::read_body_json(resp).await;
        assert_eq!(body.count, 0);
    }

    #[actix_web::test]
    async fn test_parse_unsupported_locale_returns_empty() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse", web::post().to(parse))
        ).await;

        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(ParseRequest {
                text: "tomorrow".to_string(),
                locale: Some("xx".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: ParseResponse = test::read_body_json(resp).await;
        assert_eq!(body.count, 0);
    }

    #[actix_web::test]
    async fn test_parse_batch_with_locale() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse/batch", web::post().to(parse_batch))
        ).await;

        let req = test::TestRequest::post()
            .uri("/parse/batch")
            .insert_header(("X-Request-ID", "batch-001"))
            .set_json(BatchParseRequest {
                texts: vec!["42".to_string(), "5 minutes".to_string()],
                locale: Some("fr".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("X-Request-ID").unwrap(), "batch-001");
    }

    #[actix_web::test]
    async fn test_parse_batch_missing_locale_returns_empty() {
        let state = AppState::static_only();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/parse/batch", web::post().to(parse_batch))
        ).await;

        let req = test::TestRequest::post()
            .uri("/parse/batch")
            .set_json(serde_json::json!({"texts": ["tomorrow"]}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: BatchParseResponse = test::read_body_json(resp).await;
        assert_eq!(body.total_count, 0);
    }
}

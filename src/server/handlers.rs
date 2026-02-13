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
    let text = req.text.clone();
    let text_for_block = text.clone();
    let state_clone = state.clone();

    // Normalize text for fuzzy matching (CPU-bound, run in blocking thread)
    let normalized = web::block(move || state_clone.pattern_normalizer.normalize(&text_for_block)).await?;

    // Apply rules
    // Note: rule_set.apply_all returns Rc<Node> which is not Send,
    // so we handle errors inline
    let nodes = state.rule_set.apply_all(&normalized).map_err(|_| {
        actix_web::error::ErrorBadRequest("Failed to parse input text")
    })?;

    // Note: latent filtering is not available since crate::values::Value
    // does not implement the Value trait from crate::lib
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

    let count = results.len();
    Ok(HttpResponse::Ok().json(ParseResponse { results, count }))
}

/// Health check response
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct ConfigStatusResponse {
    pub dynamic_enabled: bool,
    pub current_version: u64,
    pub source: String,
}

/// Get configuration status
pub async fn config_status(state: web::Data<AppState>) -> impl Responder {
    let config = state.config_manager.lock().unwrap();

    HttpResponse::Ok().json(ConfigStatusResponse {
        dynamic_enabled: state.dynamic_enabled,
        current_version: config.current_version(),
        source: if config.has_dynamic_rules() {
            "apollo".to_string()
        } else {
            "static".to_string()
        },
    })
}

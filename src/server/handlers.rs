use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseRequest {
    pub text: String,
    #[serde(default)]
    pub include_latent: bool,
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
) -> impl Responder {
    let text = &req.text;

    // Normalize text for fuzzy matching
    let normalized = state.pattern_normalizer.normalize(text);

    // Apply rules
    match state.rule_set.apply_all(&normalized) {
        Ok(nodes) => {
            // Note: latent filtering is not available since crate::values::Value
            // does not implement the Value trait from crate::lib
            let results: Vec<ParseResult> = nodes
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

            let count = results.len();
            HttpResponse::Ok().json(ParseResponse { results, count })
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Parse error: {:?}", e)
        })),
    }
}

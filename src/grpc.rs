//! gRPC Server module for rustling
//!
//! Provides gRPC API for:
//! - Text parsing
//! - Batch parsing
//! - Health checks
//!
//! This module is only compiled when the "grpc" feature is enabled.

use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::fuzzy::PatternNormalizer;
use crate::locale::LocaleRegistry;
use crate::grpc_proto::{
    ParseRequest, ParseResponse, BatchParseRequest, BatchParseResponse,
    BatchParseResult, ParseResult, HealthRequest, HealthResponse,
    parser_server::{self, ParserServer},
};

/// Application state for gRPC server
#[derive(Clone)]
pub struct GrpcAppState {
    /// Core rule sets indexed by locale
    pub locales: Arc<LocaleRegistry>,
    /// Pattern normalizer for fuzzy matching
    pub pattern_normalizer: Arc<PatternNormalizer>,
}

impl GrpcAppState {
    /// Create new gRPC application state
    pub fn new() -> Self {
        Self {
            locales: Arc::new(LocaleRegistry::build_all()),
            pattern_normalizer: Arc::new(PatternNormalizer::new()),
        }
    }
}

impl Default for GrpcAppState {
    fn default() -> Self {
        Self::new()
    }
}

/// gRPC Parser service implementation
#[derive(Clone)]
pub struct ParserService {
    state: GrpcAppState,
}

impl ParserService {
    /// Create new parser service
    pub fn new(state: GrpcAppState) -> Self {
        Self { state }
    }

    /// Wrap in a tonic ParserServer ready for serving
    pub fn into_server(self) -> ParserServer<Self> {
        ParserServer::new(self)
    }
}

/// Implement the tonic-generated gRPC service trait
#[tonic::async_trait]
impl parser_server::Parser for ParserService {
    async fn parse(
        &self,
        request: Request<ParseRequest>,
    ) -> Result<Response<ParseResponse>, Status> {
        let req = request.into_inner();

        if req.text.is_empty() {
            return Ok(Response::new(ParseResponse {
                results: vec![],
                count: 0,
            }));
        }

        let locale = if req.locale.is_empty() {
            "en".to_string()
        } else {
            req.locale.clone()
        };

        // Use unified parse API for clean JSON output (handles locale validation internally)
        let parse_output = crate::parse::Parser::new().parse(&req.text, Some(&locale));

        let results: Vec<ParseResult> = parse_output
            .results
            .into_iter()
            .map(|pv| {
                let value = serde_json::to_string(&pv.value).unwrap_or_default();
                ParseResult {
                    value,
                    byte_start: pv.byte_start as u32,
                    byte_end: pv.byte_end as u32,
                    char_start: pv.char_start as u32,
                    char_end: pv.char_end as u32,
                }
            })
            .collect();

        let count = results.len() as u32;
        Ok(Response::new(ParseResponse { results, count }))
    }

    async fn parse_batch(
        &self,
        request: Request<BatchParseRequest>,
    ) -> Result<Response<BatchParseResponse>, Status> {
        let req = request.into_inner();

        if req.texts.len() > 100 {
            return Err(Status::invalid_argument(
                "Batch size exceeds maximum of 100 items",
            ));
        }

        let locale = if req.locale.is_empty() {
            "en".to_string()
        } else {
            req.locale.clone()
        };

        let mut batch_results = Vec::with_capacity(req.texts.len());
        let mut total_count = 0u32;

        for (index, text) in req.texts.iter().enumerate() {
            if text.is_empty() {
                batch_results.push(BatchParseResult {
                    index: index as u32,
                    text: text.clone(),
                    results: vec![],
                    count: 0,
                });
                continue;
            }

            // Use unified parse API for clean JSON output
            let parse_output = crate::parse::Parser::new().parse(text, Some(&locale));

            let results: Vec<ParseResult> = parse_output
                .results
                .into_iter()
                .map(|pv| {
                    let value = serde_json::to_string(&pv.value).unwrap_or_default();
                    ParseResult {
                        value,
                        byte_start: pv.byte_start as u32,
                        byte_end: pv.byte_end as u32,
                        char_start: pv.char_start as u32,
                        char_end: pv.char_end as u32,
                    }
                })
                .collect();

            let count = results.len() as u32;
            total_count += count;

            batch_results.push(BatchParseResult {
                index: index as u32,
                text: text.clone(),
                results,
                count,
            });
        }

        Ok(Response::new(BatchParseResponse {
            results: batch_results,
            total_count,
        }))
    }

    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            healthy: true,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc_proto::parser_server::Parser;

    #[test]
    fn test_grpc_state_creation() {
        let state = GrpcAppState::new();
        assert!(state.locales.get("en").is_some());
        assert!(state.locales.get("fr").is_some());
    }

    #[tokio::test]
    async fn test_parse_empty_text() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(ParseRequest {
            text: "".to_string(),
            locale: "en".to_string(),
        });

        let response = service.parse(request).await.unwrap();
        let response = response.into_inner();

        assert_eq!(response.count, 0);
        assert!(response.results.is_empty());
    }

    #[tokio::test]
    async fn test_parse_integer() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(ParseRequest {
            text: "42".to_string(),
            locale: "en".to_string(),
        });

        let response = service.parse(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.count > 0);
    }

    #[tokio::test]
    async fn test_parse_duration() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(ParseRequest {
            text: "5 minutes".to_string(),
            locale: "en".to_string(),
        });

        let response = service.parse(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.count > 0);
    }

    #[tokio::test]
    async fn test_parse_batch() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(BatchParseRequest {
            texts: vec!["42".to_string(), "5 minutes".to_string()],
            locale: "en".to_string(),
        });

        let response = service.parse_batch(request).await.unwrap();
        let response = response.into_inner();

        assert_eq!(response.results.len(), 2);
        assert!(response.total_count > 0);
    }

    #[tokio::test]
    async fn test_parse_unsupported_locale() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(ParseRequest {
            text: "42".to_string(),
            locale: "xx".to_string(),
        });

        let response = service.parse(request).await.unwrap();
        let response = response.into_inner();

        assert_eq!(response.count, 0);
    }

    #[tokio::test]
    async fn test_health() {
        let service = ParserService::new(GrpcAppState::new());

        let request = Request::new(HealthRequest {});

        let response = service.health(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.healthy);
        assert!(!response.version.is_empty());
    }

    #[test]
    fn test_into_server() {
        let service = ParserService::new(GrpcAppState::new());
        let _server = service.into_server();
        // Should compile and create a ParserServer successfully
    }
}

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

        let rule_set = match self.state.locales.get(&locale) {
            Some(rs) => rs,
            None => {
                log::warn!("Unsupported locale: {}", locale);
                return Ok(Response::new(ParseResponse {
                    results: vec![],
                    count: 0,
                }));
            }
        };

        let normalized = self.state.pattern_normalizer.normalize(&req.text);

        let nodes = rule_set
            .apply_all(&normalized)
            .map_err(|e| Status::internal(format!("Parse error: {:?}", e)))?;

        let results: Vec<ParseResult> = nodes
            .iter()
            .map(|n| {
                let byte_range = n.root_node.byte_range;
                let char_range = byte_range.char_range(&req.text);
                ParseResult {
                    value: format!("{:?}", n.value),
                    byte_start: byte_range.0 as u32,
                    byte_end: byte_range.1 as u32,
                    char_start: char_range.0 as u32,
                    char_end: char_range.1 as u32,
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

        let rule_set = match self.state.locales.get(&locale) {
            Some(rs) => rs,
            None => {
                let results: Vec<BatchParseResult> = req
                    .texts
                    .iter()
                    .enumerate()
                    .map(|(idx, text)| BatchParseResult {
                        index: idx as u32,
                        text: text.clone(),
                        results: vec![],
                        count: 0,
                    })
                    .collect();
                return Ok(Response::new(BatchParseResponse {
                    results,
                    total_count: 0,
                }));
            }
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

            let normalized = self.state.pattern_normalizer.normalize(text);

            let nodes = match rule_set.apply_all(&normalized) {
                Ok(nodes) => nodes,
                Err(e) => {
                    return Err(Status::internal(format!(
                        "Parse error at index {}: {:?}",
                        index, e
                    )));
                }
            };

            let results: Vec<ParseResult> = nodes
                .iter()
                .map(|n| {
                    let byte_range = n.root_node.byte_range;
                    let char_range = byte_range.char_range(text);
                    ParseResult {
                        value: format!("{:?}", n.value),
                        byte_start: byte_range.0 as u32,
                        byte_end: byte_range.1 as u32,
                        char_start: char_range.0 as u32,
                        char_end: char_range.1 as u32,
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

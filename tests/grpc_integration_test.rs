//! gRPC end-to-end integration tests
//!
//! Run with:
//!   cargo test --features grpc --test grpc_integration_test
//!
//! These tests start a real gRPC server on a random port and connect
//! a real gRPC client to verify the full request/response stack.

#[cfg(feature = "grpc")]
mod grpc_tests {
    use rustling::grpc_proto::{
        parser_client::ParserClient,
        parser_server::ParserServer,
        BatchParseRequest, HealthRequest, ParseRequest,
    };
    use rustling::server::grpc::{GrpcAppState, ParserService};
    use std::time::Duration;
    use tokio::net::TcpListener;
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::transport::{Channel, Server};

    /// Start a test gRPC server on a random port.
    /// Returns the URI to connect to (e.g. "http://127.0.0.1:54321").
    async fn start_test_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();

        let service = ParserService::new(GrpcAppState::new());

        tokio::spawn(async move {
            Server::builder()
                .add_service(ParserServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .expect("gRPC server failed");
        });

        // Give the server a moment to start accepting connections
        tokio::time::sleep(Duration::from_millis(50)).await;

        format!("http://127.0.0.1:{}", port)
    }

    async fn make_client(addr: &str) -> ParserClient<Channel> {
        ParserClient::connect(addr.to_string())
            .await
            .expect("Failed to connect gRPC client")
    }

    // ── Parse (single text) ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_e2e_parse_integer() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse(ParseRequest {
                text: "42".to_string(),
                locale: "en".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert!(body.count > 0, "Expected at least one parse result for '42'");
        assert_eq!(body.results.len(), body.count as usize);
    }

    #[tokio::test]
    async fn test_e2e_parse_duration() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse(ParseRequest {
                text: "5 minutes".to_string(),
                locale: "en".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert!(body.count > 0, "Expected a duration parse result");
    }

    #[tokio::test]
    async fn test_e2e_parse_empty_text() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse(ParseRequest {
                text: "".to_string(),
                locale: "en".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert_eq!(body.count, 0);
        assert!(body.results.is_empty());
    }

    #[tokio::test]
    async fn test_e2e_parse_default_locale() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        // Empty locale → server falls back to "en"
        let response = client
            .parse(ParseRequest {
                text: "100".to_string(),
                locale: "".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert!(body.count > 0);
    }

    #[tokio::test]
    async fn test_e2e_parse_unsupported_locale() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse(ParseRequest {
                text: "42".to_string(),
                locale: "xx".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert_eq!(body.count, 0, "Expected empty results for unknown locale");
    }

    #[tokio::test]
    async fn test_e2e_parse_result_fields() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse(ParseRequest {
                text: "42".to_string(),
                locale: "en".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert!(body.count > 0);

        let first = &body.results[0];
        // All position fields must be set
        assert!(first.byte_end > first.byte_start, "byte_end should be > byte_start");
        assert!(first.char_end > first.char_start, "char_end should be > char_start");
        assert!(!first.value.is_empty(), "value field must not be empty");
    }

    // ── ParseBatch ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_e2e_batch_parse() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse_batch(BatchParseRequest {
                texts: vec![
                    "42".to_string(),
                    "5 minutes".to_string(),
                    "".to_string(),
                ],
                locale: "en".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert_eq!(body.results.len(), 3, "Should have 3 batch results");

        // First result: "42" → count > 0
        assert!(body.results[0].count > 0, "Expected result for '42'");
        // Second result: "5 minutes" → count > 0
        assert!(body.results[1].count > 0, "Expected result for '5 minutes'");
        // Third result: empty text → count == 0
        assert_eq!(body.results[2].count, 0, "Empty text should have 0 results");

        // Indices must be preserved in order
        for (i, r) in body.results.iter().enumerate() {
            assert_eq!(r.index, i as u32, "Batch result index mismatch");
        }
    }

    #[tokio::test]
    async fn test_e2e_batch_parse_unsupported_locale() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .parse_batch(BatchParseRequest {
                texts: vec!["42".to_string()],
                locale: "xx".to_string(),
            })
            .await
            .expect("RPC failed");

        let body = response.into_inner();
        assert_eq!(body.total_count, 0);
    }

    #[tokio::test]
    async fn test_e2e_batch_too_large() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        // 101 items exceeds the 100-item maximum
        let texts: Vec<String> = (0..101).map(|i| i.to_string()).collect();

        let result = client
            .parse_batch(BatchParseRequest {
                texts,
                locale: "en".to_string(),
            })
            .await;

        assert!(result.is_err(), "Should reject oversized batch");
        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }

    // ── Health ──────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_e2e_health_check() {
        let addr = start_test_server().await;
        let mut client = make_client(&addr).await;

        let response = client
            .health(HealthRequest {})
            .await
            .expect("Health RPC failed");

        let body = response.into_inner();
        assert!(body.healthy, "Server should report healthy");
        assert!(!body.version.is_empty(), "Version must not be empty");
    }
}

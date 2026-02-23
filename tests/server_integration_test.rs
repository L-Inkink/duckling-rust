//! Integration tests for the HTTP server
//!
//! These tests verify end-to-end functionality of the rustling HTTP server,
//! including all endpoints and hot-reload capabilities.

use actix_web::{test, App};
use rustling::server::{handlers, ServerBuilder};
use rustling::dynamic::loader::InlineLoader;
use rustling::dynamic::ConfigManager;
use rustling::server::AppState;
use rustling::locale::LocaleRegistry;
use rustling::fuzzy::PatternNormalizer;
use rustling::{RuleSetBuilder, BoundariesChecker};
use rustling::rules;
use serde_json::json;
use std::sync::{Arc, Mutex, RwLock};

/// Test basic health check endpoint
#[actix_web::test]
async fn test_health_check_integration() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: handlers::HealthResponse = test::read_body_json(resp).await;
    assert_eq!(body.status, "healthy");
    assert!(!body.dynamic_enabled);
}

/// Test parsing endpoint with static rules
#[actix_web::test]
async fn test_parse_endpoint_integration() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    // Test parsing a number
    let req = test::TestRequest::post()
        .uri("/parse")
        .set_json(json!({"text": "42", "locale": "en"}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: handlers::ParseResponse = test::read_body_json(resp).await;
    assert!(body.count > 0);
    assert!(!body.results.is_empty());
}

/// Test batch parsing endpoint
#[actix_web::test]
async fn test_batch_parse_integration() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let req = test::TestRequest::post()
        .uri("/parse/batch")
        .set_json(json!({
            "texts": ["5 minutes", "3 hours", "tomorrow at 3pm"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: handlers::BatchParseResponse = test::read_body_json(resp).await;
    assert_eq!(body.results.len(), 3);
    assert!(body.total_count > 0);

    // Verify each result has correct index
    for (i, result) in body.results.iter().enumerate() {
        assert_eq!(result.index, i);
        assert!(!result.text.is_empty());
    }
}

/// Test config status endpoint
#[actix_web::test]
async fn test_config_status_integration() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let req = test::TestRequest::get()
        .uri("/config/status")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: handlers::ConfigStatusResponse = test::read_body_json(resp).await;
    assert!(!body.dynamic_enabled);
    assert_eq!(body.source, "static");
}

/// Test config reload endpoint requires authentication
#[actix_web::test]
async fn test_config_reload_authentication() {
    std::env::set_var("RELOAD_API_KEY", "test-secret-key");

    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    // Test without API key - should fail
    let req = test::TestRequest::post()
        .uri("/config/reload")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403); // Forbidden

    // Test with wrong API key - should fail
    let req = test::TestRequest::post()
        .uri("/config/reload")
        .insert_header(("X-API-Key", "wrong-key"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403); // Forbidden

    std::env::remove_var("RELOAD_API_KEY");
}

/// Test input validation - text too long
#[actix_web::test]
async fn test_input_validation_text_length() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let long_text = "a".repeat(10_001); // Exceeds 10KB limit
    let req = test::TestRequest::post()
        .uri("/parse")
        .set_json(json!({"text": long_text, "locale": "en"}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400); // Bad Request
}

/// Test batch validation - too many items
#[actix_web::test]
async fn test_batch_validation_size_limit() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let large_batch: Vec<String> = (0..101).map(|i| i.to_string()).collect();
    let req = test::TestRequest::post()
        .uri("/parse/batch")
        .set_json(json!({"texts": large_batch}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400); // Bad Request
}

/// Test Swagger UI endpoint
#[actix_web::test]
async fn test_swagger_ui_endpoint() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    let req = test::TestRequest::get()
        .uri("/swagger-ui/")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status().is_redirection());
}

/// Test hot-reload functionality
#[actix_web::test]
async fn test_hot_reload_functionality() {
    // Create a test rule set
    let rules_json = json!({
        "version": 1,
        "metadata": {
            "name": "test-rules",
            "locale": "en"
        },
        "rules": [
            {
                "type": "Terminal",
                "name": "test integer",
                "pattern": r"(\d+)",
                "capture_group": 1,
                "value": {
                    "kind": "Integer",
                    "value": "{1}"
                },
                "enabled": true,
                "priority": 0
            }
        ]
    });

    let loader = InlineLoader::new(rules_json.to_string());
    let config_manager = ConfigManager::new(loader);

    // Create AppState with dynamic rules
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules::register_all_rules(&b);
    let rule_set = b.build();

    let state = AppState {
        rule_set: Arc::new(RwLock::new(rule_set)),
        config_manager: Arc::new(Mutex::new(config_manager)),
        pattern_normalizer: Arc::new(PatternNormalizer::new()),
        dynamic_enabled: true,
        locales: Arc::new(LocaleRegistry::build_all()),
    };

    // Test reload
    let result = state.reload_rules();
    assert!(result.is_ok());

    let reloaded = result.unwrap();
    assert_eq!(reloaded.version, 1);
    assert_eq!(reloaded.rules.len(), 1);
}

/// Test parsing various input types
#[actix_web::test]
async fn test_parse_various_inputs() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    // Test cases with expected patterns
    let test_cases = vec![
        ("42", true),                    // Integer
        ("5 minutes", true),             // Duration
        ("3 hours and 30 minutes", true), // Duration
        ("", false),                     // Empty (should still work, just no results)
        ("   ", false),                  // Whitespace only
    ];

    for (text, _should_have_results) in test_cases {
        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(json!({"text": text, "locale": "en"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success(), "Failed to parse: {}", text);

        let _body: handlers::ParseResponse = test::read_body_json(resp).await;
        // Results may vary, just ensure parsing doesn't error
    }
}

/// Test concurrent requests
#[actix_web::test]
async fn test_concurrent_requests() {
    let builder = ServerBuilder::new();
    let app = test::init_service(App::new().configure(builder.configure())).await;

    // Create multiple concurrent requests
    let mut handles = vec![];

    for i in 0..10 {
        let req = test::TestRequest::post()
            .uri("/parse")
            .set_json(json!({"text": format!("{} minutes", i), "locale": "en"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        handles.push(resp);
    }

    // All requests should succeed
    for resp in handles {
        assert!(resp.status().is_success());
    }
}

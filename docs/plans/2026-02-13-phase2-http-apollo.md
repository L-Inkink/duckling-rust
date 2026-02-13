# Phase 2: HTTP 服务器 + Apollo 热重载实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标：** 构建 HTTP 服务器提供 REST API 接口用于文本解析，集成 Apollo 配置中心实现动态规则热重载

**架构：** 基于 Actix-web 的 HTTP 服务器，配合后台任务轮询 Apollo 配置，提供解析和配置管理的 REST 端点

**技术栈：** Actix-web 4.x, Tokio 异步运行时, Apollo 客户端（已实现）, serde_json

---

## 第一周：HTTP 服务器基础

### Task 1: 添加 Actix-web 依赖

**文件：**
- 修改: `Cargo.toml`

**Step 1: 添加 actix-web 和 tokio 依赖**

修改: `Cargo.toml`

```toml
[dependencies]
# 在现有依赖后添加
actix-web = "4.4"
actix-rt = "2.9"
tokio = { version = "1", features = ["full"] }
```

**Step 2: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 3: 提交**

```bash
git add Cargo.toml
git commit -m "chore: add actix-web and tokio dependencies for HTTP server"
```

---

### Task 2: 创建 HTTP 服务器模块结构

**文件：**
- 创建: `src/server/mod.rs`
- 创建: `src/server/handlers.rs`
- 创建: `src/server/state.rs`
- 修改: `src/lib.rs`

**Step 1: 创建 server 模块目录**

```bash
mkdir -p src/server
```

**Step 2: 创建 server 模块文件**

创建: `src/server/mod.rs`

```rust
//! HTTP Server module for rustling
//!
//! Provides REST API for:
//! - Text parsing
//! - Configuration management
//! - Health checks

pub mod handlers;
pub mod state;

pub use state::AppState;
```

**Step 3: 创建应用状态**

创建: `src/server/state.rs`

```rust
use crate::dynamic::{ConfigManager, DynamicRuleSet};
use crate::fuzzy::PatternNormalizer;
use crate::rules;
use crate::values::Value;
use rustling_core::{BoundariesChecker, RuleSet as CoreRuleSet};

/// Application state shared across HTTP handlers
pub struct AppState {
    /// Core rule set for parsing
    pub rule_set: CoreRuleSet<Value>,
    /// Dynamic rule configuration manager
    pub config_manager: std::sync::Mutex<ConfigManager>,
    /// Pattern normalizer for fuzzy matching
    pub pattern_normalizer: PatternNormalizer,
    /// Whether dynamic rules are enabled
    pub dynamic_enabled: bool,
}

impl AppState {
    /// Create new application state with static rules only
    pub fn static_only() -> Self {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules::register_all_rules(&b);
        let rule_set = b.build();

        Self {
            rule_set,
            config_manager: std::sync::Mutex::new(ConfigManager::static_only()),
            pattern_normalizer: PatternNormalizer::new(),
            dynamic_enabled: false,
        }
    }
}
```

**Step 4: 导出 server 模块**

修改: `src/lib.rs` - 在第25行后添加

```rust
pub mod server;
```

**Step 5: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 6: 提交**

```bash
git add src/server/ src/lib.rs
git commit -m "feat: add HTTP server module structure"
```

---

### Task 3: 实现解析处理器

**文件：**
- 修改: `src/server/handlers.rs`

**Step 1: 编写失败的测试**

创建: `tests/server_parse_test.rs`

```rust
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ParseRequest {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct ParseResponse {
    results: Vec<ParsedResult>,
}

#[derive(Serialize, Deserialize)]
struct ParsedResult {
    value: String,
    byte_start: usize,
    byte_end: usize,
}

#[actix_web::test]
async fn test_parse_endpoint() {
    // Test will be implemented after handler is ready
}
```

**Step 2: 运行测试验证失败**

运行: `cargo test test_parse_endpoint`
预期: 测试运行（初始为空）

**Step 3: 实现解析处理器**

修改: `src/server/handlers.rs`

```rust
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
            let results: Vec<ParseResult> = nodes
                .iter()
                .filter(|n| req.include_latent || !n.latent)
                .map(|n| ParseResult {
                    value: format!("{:?}", n.value),
                    byte_start: n.byte_range.start,
                    byte_end: n.byte_range.end,
                    char_start: n.char_range.start,
                    char_end: n.char_range.end,
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
```

**Step 4: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 5: 提交**

```bash
git add src/server/handlers.rs tests/server_parse_test.rs
git commit -m "feat: implement parse HTTP handler"
```

---

### Task 4: 实现健康检查和配置端点

**文件：**
- 修改: `src/server/handlers.rs`

**Step 1: 添加健康检查处理器**

修改: `src/server/handlers.rs` - 在 parse 函数后添加

```rust
use crate::dynamic::ConfigError;

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
```

**Step 2: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 3: 提交**

```bash
git add src/server/handlers.rs
git commit -m "feat: add health check and config status endpoints"
```

---

### Task 5: 创建 HTTP 服务器构建器/App

**文件：**
- 创建: `src/server/app.rs`

**Step 1: 编写失败的测试**

创建: `tests/server_app_test.rs`

```rust
#[test]
fn test_server_builder() {
    use crate::server::App;

    let app = App::new()
        .with_static_rules();

    // Verify app builds
    assert!(true);
}
```

**Step 2: 实现服务器构建器**

创建: `src/server/app.rs`

```rust
use actix_web::{web, App as ActixApp, HttpServer};

use super::handlers::{config_status, health, parse};
use super::AppState;

/// Server builder for configuring and creating the HTTP server
pub struct ServerBuilder {
    bind_address: String,
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

    /// Build and return the ActixApp
    pub fn build(self) -> ActixApp<AppState> {
        ActixApp::new()
            .app_data(web::Data::new(self.state))
            .route("/health", web::get().to(health))
            .route("/parse", web::post().to(parse))
            .route("/config/status", web::get().to(config_status))
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
```

**Step 3: 更新 mod.rs 导出 App**

修改: `src/server/mod.rs`

```rust
pub mod handlers;
pub mod state;
pub mod app;

pub use state::AppState;
pub use app::ServerBuilder;
```

**Step 4: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 5: 提交**

```bash
git add src/server/app.rs src/server/mod.rs
git commit -m "feat: add HTTP server builder"
```

---

## 第二周：Apollo 集成 + 热重载

### Task 6: 实现 Apollo 热重载后台任务

**文件：**
- 创建: `src/server/reload.rs`
- 修改: `src/server/state.rs`

**Step 1: 编写失败的测试**

创建: `tests/server_reload_test.rs`

```rust
#[test]
fn test_reload_task() {
    // Test will verify hot reload functionality
}
```

**Step 2: 实现重载任务**

创建: `src/server/reload.rs`

```rust
use crate::dynamic::{ConfigError, ConfigLoader};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Background task for polling Apollo configuration
pub struct ReloadTask {
    loader: Arc<dyn ConfigLoader>,
    poll_interval: Duration,
}

impl ReloadTask {
    /// Create a new reload task
    pub fn new(loader: Arc<dyn ConfigLoader>, poll_interval_secs: u64) -> Self {
        Self {
            loader,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    /// Start the background polling task
    /// Returns a handle that can be used to stop the task
    pub fn start<F>(self, on_reload: F) -> ReloadTaskHandle
    where
        F: Fn() + Send + Sync + 'static,
    {
        let handle = ReloadTaskHandle::new();

        let loader = Arc::new(self.loader);
        let poll_interval = self.poll_interval;
        let on_reload = Arc::new(on_reload);

        // Spawn background task
        tokio::spawn(async move {
            loop {
                sleep(poll_interval).await;

                if let Err(e) = self.check_and_reload(&loader, &on_reload) {
                    log::warn!("Reload check failed: {:?}", e);
                }

                if handle.is_stopped() {
                    break;
                }
            }
        });

        handle
    }

    fn check_and_reload(
        &self,
        loader: &Arc<dyn ConfigLoader>,
        on_reload: &Arc<dyn Fn() + Send + Sync>,
    ) -> Result<(), ConfigError> {
        // Check if configuration has changed
        // In a real implementation, this would use Apollo's notification API
        // For now, we trigger a reload
        on_reload();
        Ok(())
    }
}

/// Handle to control the reload task
pub struct ReloadTaskHandle {
    stopped: std::sync::atomic::AtomicBool,
}

impl ReloadTaskHandle {
    fn new() -> Self {
        Self {
            stopped: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Stop the reload task
    pub fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn is_stopped(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::SeqCst)
    }
}
```

**Step 3: 更新状态以支持热重载**

修改: `src/server/state.rs`

```rust
use crate::dynamic::{ConfigManager, DynamicRuleSet};
use crate::fuzzy::PatternNormalizer;
use crate::rules;
use crate::values::Value;
use rustling_core::{BoundariesChecker, RuleSet as CoreRuleSet};

/// Application state shared across HTTP handlers
pub struct AppState {
    /// Core rule set for parsing
    pub rule_set: CoreRuleSet<Value>,
    /// Dynamic rule configuration manager
    pub config_manager: std::sync::Mutex<ConfigManager>,
    /// Pattern normalizer for fuzzy matching
    pub pattern_normalizer: PatternNormalizer,
    /// Whether dynamic rules are enabled
    pub dynamic_enabled: bool,
}

impl AppState {
    /// Create new application state with static rules only
    pub fn static_only() -> Self {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules::register_all_rules(&b);
        let rule_set = b.build();

        Self {
            rule_set,
            config_manager: std::sync::Mutex::new(ConfigManager::static_only()),
            pattern_normalizer: PatternNormalizer::new(),
            dynamic_enabled: false,
        }
    }

    /// Reload dynamic rules from configuration
    pub fn reload_rules(&self) -> Result<DynamicRuleSet, String> {
        let mut config = self.config_manager.lock().unwrap();
        config.load_rules().map_err(|e| format!("{:?}", e))
    }
}
```

**Step 4: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 5: 提交**

```bash
git add src/server/reload.rs src/server/state.rs
git commit -m "feat: implement Apollo hot reload background task"
```

---

### Task 7: 添加配置重载端点

**文件：**
- 修改: `src/server/handlers.rs`

**Step 1: 添加重载处理器**

修改: `src/server/handlers.rs` - 在 config_status 后添加

```rust
/// Reload configuration request
#[derive(Debug, Deserialize)]
pub struct ReloadRequest {
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

fn default_poll_interval() -> u64 {
    60 // Default 60 seconds
}

/// Trigger configuration reload
pub async fn config_reload(
    state: web::Data<AppState>,
) -> impl Responder {
    match state.reload_rules() {
        Ok(rules) => HttpResponse::Ok().json(serde_json::json!({
            "status": "reloaded",
            "version": rules.version,
            "rule_count": rules.rules.len(),
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e,
        })),
    }
}
```

**Step 2: 更新 app.rs 添加路由**

修改: `src/server/app.rs`

```rust
use super::handlers::{config_reload, config_status, health, parse};

// In build() method, add route:
.route("/config/reload", web::post().to(config_reload))
```

**Step 3: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 4: 提交**

```bash
git add src/server/handlers.rs src/server/app.rs
git commit -m "feat: add config reload endpoint"
```

---

### Task 8: 添加主入口

**文件：**
- 创建: `examples/http_server.rs`

**Step 1: 创建 HTTP 服务器示例**

创建: `examples/http_server.rs`

```rust
//! HTTP Server example for rustling
//!
//! Run with: cargo run --example http_server
//!
//! Endpoints:
//! - POST /parse - Parse text
//! - GET /health - Health check
//! - GET /config/status - Configuration status
//! - POST /config/reload - Reload configuration

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use duckling_rust::server::{AppState, ServerBuilder};

async fn parse_handler(
    state: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> impl Responder {
    let text = req.get("text").and_then(|v| v.as_str()).unwrap_or("");

    // Normalize and parse
    let normalized = state.pattern_normalizer.normalize(text);
    match state.rule_set.apply_all(&normalized) {
        Ok(nodes) => {
            let results: Vec<_> = nodes.iter().map(|n| {
                serde_json::json!({
                    "value": format!("{:?}", n.value),
                    "byte_range": [n.byte_range.start, n.byte_range.end],
                })
            }).collect();

            HttpResponse::Ok().json(serde_json::json!({
                "results": results,
                "count": results.len(),
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("{:?}", e)
        })),
    }
}

async fn health_handler(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    let bind_addr = "127.0.0.1:8080";
    println!("Starting server at http://{}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::static_only()))
            .route("/parse", web::post().to(parse_handler))
            .route("/health", web::get().to(health_handler))
    })
    .bind(bind_addr)?
    .run()
    .await
}
```

**Step 2: 验证编译**

运行: `cargo check --example http_server`
预期: 编译成功

**Step 3: 提交**

```bash
git add examples/http_server.rs
git commit -m "feat: add HTTP server example"
```

---

## 第三周：高级功能

### Task 9: 添加批量解析端点

**文件：**
- 修改: `src/server/handlers.rs`

**Step 1: 添加批量解析处理器**

修改: `src/server/handlers.rs`

```rust
/// Batch parse request
#[derive(Debug, Deserialize)]
pub struct BatchParseRequest {
    pub texts: Vec<String>,
    #[serde(default)]
    pub include_latent: bool,
}

/// Batch parse response
#[derive(Debug, Serialize)]
pub struct BatchParseResponse {
    pub results: Vec<BatchParseResult>,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct BatchParseResult {
    pub text: String,
    pub matches: Vec<ParseResult>,
}

/// Parse multiple texts in batch
pub async fn parse_batch(
    state: web::Data<AppState>,
    req: web::Json<BatchParseRequest>,
) -> impl Responder {
    let results: Vec<BatchParseResult> = req
        .texts
        .iter()
        .map(|text| {
            let normalized = state.pattern_normalizer.normalize(text);
            let matches = state
                .rule_set
                .apply_all(&normalized)
                .unwrap_or_default()
                .into_iter()
                .filter(|n| req.include_latent || !n.latent)
                .map(|n| ParseResult {
                    value: format!("{:?}", n.value),
                    byte_start: n.byte_range.start,
                    byte_end: n.byte_range.end,
                    char_start: n.char_range.start,
                    char_end: n.char_range.end,
                })
                .collect();

            BatchParseResult {
                text: text.clone(),
                matches,
            }
        })
        .collect();

    let total_count = results.iter().map(|r| r.matches.len()).sum();

    HttpResponse::Ok().json(BatchParseResponse {
        results,
        total_count,
    })
}
```

**Step 2: 更新 app.rs 添加路由**

修改: `src/server/app.rs`

```rust
use super::handlers::{parse, parse_batch, health, config_status, config_reload};

// Add route:
.route("/parse/batch", web::post().to(parse_batch))
```

**Step 3: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 4: 提交**

```bash
git add src/server/handlers.rs src/server/app.rs
git commit -m "feat: add batch parse endpoint"
```

---

### Task 10: 添加 OpenAPI/Swagger 文档

**文件：**
- 修改: `Cargo.toml`
- 创建: `src/server/docs.rs`

**Step 1: 添加 swagger 依赖**

修改: `Cargo.toml`

```toml
[dependencies]
actix-web-swagger = "3.0"
```

**Step 2: 添加 Swagger 文档**

创建: `src/server/docs.rs`

```rust
use actix_web::swagger::SwaggerUi;
use actix_web::{web, HttpResponse};

/// Get OpenAPI 3.0 specification
pub async fn get_openapi() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(swagger_spec())
}

fn swagger_spec() -> serde_json::Value {
    serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Rustling Parser API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Natural language parsing API with dynamic rule support"
        },
        "paths": {
            "/parse": {
                "post": {
                    "summary": "Parse text",
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "text": {"type": "string"},
                                        "include_latent": {"type": "boolean"}
                                    },
                                    "required": ["text"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Parse results",
                            "content": {
                                "application/json": {}
                            }
                        }
                    }
                }
            },
            "/health": {
                "get": {
                    "summary": "Health check",
                    "responses": {
                        "200": {
                            "description": "Service is healthy"
                        }
                    }
                }
            }
        }
    })
}
```

**Step 3: 验证编译**

运行: `cargo check`
预期: 编译成功

**Step 4: 提交**

```bash
git add Cargo.toml src/server/docs.rs
git commit -m "feat: add OpenAPI documentation"
```

---

### Task 11: 最终集成测试

**文件：**
- 创建: `tests/server_integration_test.rs`

**Step 1: 编写集成测试**

创建: `tests/server_integration_test.rs`

```rust
#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn test_parse_integration() {
        use crate::rules;
        use crate::values::Value;
        use rustling_core::{BoundariesChecker, RuleSetBuilder};

        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::_word(),
        );
separated_alphanumeric        rules::register_all_rules(&b);
        let rule_set = b.build();

        // Test parsing
        let result = rule_set.apply_all("5 minutes");
        assert!(result.is_ok());

        let nodes = result.unwrap();
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_pattern_normalizer() {
        use crate::fuzzy::PatternNormalizer;

        let normalizer = PatternNormalizer::new();

        // Test normalization
        assert_eq!(normalizer.normalize("明天早上"), "明天早上");
        assert_eq!(normalizer.normalize("明日早上"), "明天早上");
    }
}
```

**Step 2: 运行测试**

运行: `cargo test`
预期: 所有测试通过

**Step 3: 运行 clippy**

运行: `cargo clippy`
预期: 无警告

**Step 4: 提交**

```bash
git add tests/server_integration_test.rs
git commit -m "test: add server integration tests"
```

---

## 总结

本计划实现 Phase 2：

| 周 | 任务 | 描述 |
|----|------|------|
| 第一周 | 1-5 | HTTP 服务器基础，解析端点，健康检查 |
| 第二周 | 6-8 | Apollo 热重载，配置端点，主入口 |
| 第三周 | 9-11 | 批量解析，OpenAPI 文档，集成测试 |

**总计：11 个任务**

**验证：**
- 所有测试通过
- Clippy 无警告
- 文档完整

---

**计划保存：** `docs/plans/2026-02-13-phase2-http-apollo.md`
**预计时长：** 3 周（2月13日 - 3月5日）
**验收标准：** 所有测试通过 + Clippy 干净 + 完整 API 文档

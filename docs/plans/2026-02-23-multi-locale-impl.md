# Multi-Locale Routing Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `LocaleRegistry` so `POST /parse` can route to the correct language rule set via `{"locale": "fr"}`, with `X-Request-ID` tracing throughout.

**Architecture:** A new `src/locale/registry.rs` builds all 28 language rule sets at startup and stores them in a `HashMap`. `AppState` gains a `locales` field. The handler reads `X-Request-ID` from the request header, checks the `locale` field, looks up the registry, and logs a warning (returning empty results) if locale is absent or unsupported.

**Tech Stack:** Rust, actix-web, rustling_core::RuleSet, chrono, log

---

### Task 1: Normalize `en/time.rs` signature

`en/time.rs` currently has `pub fn rules(b: &RuleSetBuilder<Value>)` — missing the `context` parameter that all other 27 languages have. Unify the signature so the registry can call all languages uniformly.

**Files:**
- Modify: `languages/en/time.rs:23`

**Step 1: Write failing compile test**

Add to the bottom of `languages/en/time.rs`:

```rust
#[cfg(test)]
mod sig_test {
    use super::*;
    use std::sync::Arc;
    use rustling_core::time::TimeContext;
    use rustling_core::{BoundariesChecker, RuleSetBuilder};
    use crate::values::Value;

    #[test]
    fn test_rules_accepts_context() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        // This must compile: calling rules() with a context argument
        rules(&b, Some(Arc::new(TimeContext::default())));
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test -p rustling languages::en::time::sig_test 2>&1 | head -20
```
Expected: compile error — `rules` takes 1 argument but 2 were supplied.

**Step 3: Update the function signature**

In `languages/en/time.rs`, change line 23:

```rust
// Before:
pub fn rules(b: &RuleSetBuilder<Value>) {

// After:
pub fn rules(b: &RuleSetBuilder<Value>, _context: Option<Arc<TimeContext>>) {
```

Also add the missing import at the top of the file if not present:

```rust
use std::sync::Arc;
use rustling_core::time::TimeContext;
```

**Step 4: Run tests to verify they pass**

```bash
cargo test 2>&1 | tail -5
```
Expected: `test result: ok`

**Step 5: Commit**

```bash
git add languages/en/time.rs
git commit -m "fix: normalize en/time rules() signature to accept context param"
```

---

### Task 2: Create `src/locale/` module

**Files:**
- Create: `src/locale/mod.rs`
- Create: `src/locale/registry.rs`
- Modify: `src/lib.rs`

**Step 1: Write failing test for LocaleRegistry**

Create `src/locale/registry.rs` with only the test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_supports_known_locales() {
        let registry = LocaleRegistry::build_all();
        assert!(registry.get("en").is_some());
        assert!(registry.get("fr").is_some());
        assert!(registry.get("zh").is_some());
        assert!(registry.get("xx").is_none());
    }

    #[test]
    fn test_supported_locales_count() {
        let registry = LocaleRegistry::build_all();
        let locales = registry.supported_locales();
        assert!(locales.len() >= 28, "expected 28 locales, got {}", locales.len());
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test locale::registry 2>&1 | head -20
```
Expected: compile error — `LocaleRegistry` not found.

**Step 3: Implement `LocaleRegistry`**

Write the full `src/locale/registry.rs`:

```rust
use std::collections::HashMap;
use std::sync::Arc;

use rustling_core::time::TimeContext;
use rustling_core::{BoundariesChecker, RuleSet, RuleSetBuilder};

use crate::languages;
use crate::rules;
use crate::values::Value;

pub struct LocaleRegistry {
    rule_sets: HashMap<String, Arc<RuleSet<Value>>>,
}

impl LocaleRegistry {
    /// Build rule sets for all 28 supported locales.
    /// Called once at server startup.
    pub fn build_all() -> Self {
        let mut map = HashMap::new();

        // Each entry: (locale code, registration closure)
        // All language time::rules() share the same signature:
        //   fn(&RuleSetBuilder<Value>, Option<Arc<TimeContext>>)
        let language_rules: &[(&str, fn(&RuleSetBuilder<Value>, Option<Arc<TimeContext>>))] = &[
            ("ar", languages::ar::time::rules),
            ("bg", languages::bg::time::rules),
            ("ca", languages::ca::time::rules),
            ("da", languages::da::time::rules),
            ("de", languages::de::time::rules),
            ("el", languages::el::time::rules),
            ("en", languages::en::time::rules),
            ("es", languages::es::time::rules),
            ("fr", languages::fr::time::rules),
            ("ga", languages::ga::time::rules),
            ("he", languages::he::time::rules),
            ("hr", languages::hr::time::rules),
            ("hu", languages::hu::time::rules),
            ("it", languages::it::time::rules),
            ("ja", languages::ja::time::rules),
            ("ka", languages::ka::time::rules),
            ("ko", languages::ko::time::rules),
            ("nb", languages::nb::time::rules),
            ("nl", languages::nl::time::rules),
            ("pl", languages::pl::time::rules),
            ("pt", languages::pt::time::rules),
            ("ro", languages::ro::time::rules),
            ("ru", languages::ru::time::rules),
            ("sv", languages::sv::time::rules),
            ("tr", languages::tr::time::rules),
            ("uk", languages::uk::time::rules),
            ("vi", languages::vi::time::rules),
            ("zh", languages::zh::time::rules),
        ];

        let ctx = Arc::new(TimeContext::default());

        for (code, register_fn) in language_rules {
            let b = RuleSetBuilder::new(
                BoundariesChecker::detailed(),
                BoundariesChecker::separated_alphanumeric_word(),
            );
            // Register shared base rules (integers, durations)
            rules::integer::rules(&b);
            rules::duration::rules(&b);
            // Register language-specific time rules
            register_fn(&b, Some(Arc::clone(&ctx)));
            map.insert(code.to_string(), Arc::new(b.build()));
        }

        Self { rule_sets: map }
    }

    /// Look up a rule set by locale code (e.g. "fr", "zh").
    /// Returns None if the locale is not supported.
    pub fn get(&self, locale: &str) -> Option<&Arc<RuleSet<Value>>> {
        self.rule_sets.get(locale)
    }

    /// List all supported locale codes.
    pub fn supported_locales(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.rule_sets.keys().map(|s| s.as_str()).collect();
        v.sort();
        v
    }
}
```

Create `src/locale/mod.rs`:

```rust
pub mod registry;
pub use registry::LocaleRegistry;
```

Add to `src/lib.rs` (after existing `pub mod dynamic;`):

```rust
pub mod locale;
```

**Step 4: Run tests to verify they pass**

```bash
cargo test locale::registry 2>&1 | tail -10
```
Expected: `test result: ok. 2 passed`

**Step 5: Commit**

```bash
git add src/locale/ src/lib.rs
git commit -m "feat: add LocaleRegistry with all 28 language rule sets"
```

---

### Task 3: Add `locales` field to `AppState`

**Files:**
- Modify: `src/server/state.rs`

**Step 1: Write failing test**

Add to the test block in `src/server/state.rs`:

```rust
#[test]
fn test_app_state_has_locale_registry() {
    let state = AppState::static_only();
    // fr should be in the registry
    assert!(state.locales.get("fr").is_some());
    // unknown locale should not be
    assert!(state.locales.get("xx").is_none());
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test server::state::tests::test_app_state_has_locale_registry 2>&1 | head -20
```
Expected: compile error — no field `locales` on `AppState`.

**Step 3: Add `locales` to `AppState`**

In `src/server/state.rs`, add the import and field:

```rust
// Add import at top:
use crate::locale::LocaleRegistry;

// Add field to struct:
pub struct AppState {
    pub locales: Arc<LocaleRegistry>,           // ← add this line
    pub rule_set: Arc<RwLock<CoreRuleSet<Value>>>,
    pub config_manager: Arc<Mutex<ConfigManager>>,
    pub pattern_normalizer: Arc<PatternNormalizer>,
    pub dynamic_enabled: bool,
}
```

Update `static_only()` to build the registry:

```rust
pub fn static_only() -> Self {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules::register_all_rules(&b);
    let rule_set = b.build();

    Self {
        locales: Arc::new(LocaleRegistry::build_all()),   // ← add this line
        rule_set: Arc::new(RwLock::new(rule_set)),
        config_manager: Arc::new(Mutex::new(ConfigManager::static_only())),
        pattern_normalizer: Arc::new(PatternNormalizer::new()),
        dynamic_enabled: false,
    }
}
```

Also update the two test helper constructors in the test block that build `AppState` directly — add `locales: Arc::new(LocaleRegistry::build_all())` to each.

**Step 4: Run tests to verify they pass**

```bash
cargo test server::state 2>&1 | tail -10
```
Expected: `test result: ok`

**Step 5: Commit**

```bash
git add src/server/state.rs
git commit -m "feat: add LocaleRegistry to AppState"
```

---

### Task 4: Update `ParseRequest` and `parse` handler

**Files:**
- Modify: `src/server/handlers.rs`

**Step 1: Write failing tests**

Add to the test block in `src/server/handlers.rs`:

```rust
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
            text: "demain".to_string(),
            locale: Some("fr".to_string()),
        })
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    // X-Request-ID must be echoed back
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
```

**Step 2: Run tests to verify they fail**

```bash
cargo test server::handlers::tests::test_parse_with_locale_fr 2>&1 | head -20
```
Expected: compile error — `ParseRequest` has no `locale` field.

**Step 3: Update `ParseRequest`**

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParseRequest {
    #[schema(example = "I need 5 minutes")]
    pub text: String,
    /// BCP-47 locale code, e.g. "en", "fr", "zh".
    /// Missing or unsupported locale returns empty results.
    #[schema(example = "en")]
    pub locale: Option<String>,
}
```

**Step 4: Rewrite the `parse` handler**

Replace the existing `parse` function body with:

```rust
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

    // Extract X-Request-ID for log tracing
    let request_id = http_req
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Resolve locale → rule set
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
```

Also add `use std::sync::Arc;` at the top of `handlers.rs` if not already present.

**Step 5: Run tests to verify they pass**

```bash
cargo test server::handlers 2>&1 | tail -10
```
Expected: `test result: ok` — all handler tests pass.

**Step 6: Commit**

```bash
git add src/server/handlers.rs
git commit -m "feat: add locale routing and X-Request-ID tracing to parse handler"
```

---

### Task 5: Update `/parse/batch` to support locale

The batch endpoint must behave consistently with the single-parse endpoint.

**Files:**
- Modify: `src/server/handlers.rs` (BatchParseRequest + parse_batch handler)

**Step 1: Write failing test**

Add to test block:

```rust
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
            texts: vec!["demain".to_string(), "hier".to_string()],
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
```

**Step 2: Run tests to verify they fail**

```bash
cargo test server::handlers::tests::test_parse_batch_with_locale 2>&1 | head -15
```
Expected: compile error — `BatchParseRequest` has no `locale` field.

**Step 3: Update `BatchParseRequest` and handler**

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BatchParseRequest {
    pub texts: Vec<String>,
    /// BCP-47 locale code. Missing or unsupported returns empty results for all items.
    #[schema(example = "fr")]
    pub locale: Option<String>,
}
```

In `parse_batch`, add the same locale-check logic before the loop:

```rust
pub async fn parse_batch(
    state: web::Data<AppState>,
    req: web::Json<BatchParseRequest>,
    http_req: actix_web::HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    // ... existing size validation ...

    let request_id = http_req
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

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

    // ... rest of batch processing using `rule_set` ...
    // Add .insert_header(("X-Request-ID", request_id.as_str())) to final response
}
```

**Step 4: Run all handler tests**

```bash
cargo test server::handlers 2>&1 | tail -10
```
Expected: `test result: ok` — all tests pass.

**Step 5: Commit**

```bash
git add src/server/handlers.rs
git commit -m "feat: add locale routing and X-Request-ID to batch parse handler"
```

---

### Task 6: Full integration smoke test

Verify the whole chain works end-to-end before declaring done.

**Step 1: Run full test suite**

```bash
cargo test 2>&1 | tail -10
```
Expected: `test result: ok. N passed; 0 failed`

**Step 2: Run Clippy**

```bash
cargo clippy 2>&1 | grep -E "^error" | wc -l
```
Expected: `0`

**Step 3: Verify with a manual curl (optional, if server running)**

```bash
# Start server in background
cargo run &
sleep 2

# French: "demain" (tomorrow)
curl -s -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: smoke-001" \
  -d '{"text": "demain", "locale": "fr"}' | jq .

# Missing locale → empty
curl -s -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "tomorrow"}' | jq .

# Unknown locale → empty
curl -s -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "tomorrow", "locale": "xx"}' | jq .

kill %1
```

**Step 4: Final commit**

```bash
git add -p  # review any remaining changes
git commit -m "feat: complete multi-locale routing with X-Request-ID tracing"
```

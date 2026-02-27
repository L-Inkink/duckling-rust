# Multi-Locale Support Design

**Date**: 2026-02-23
**Status**: Approved
**Author**: Claude Code (brainstorming session)

---

## Overview

Add per-locale rule set routing to the HTTP server so that callers can specify a language (e.g. `"fr"`, `"zh"`, `"de"`) and get results from the corresponding language rule set. All 26+ language rule sets defined in `languages/*/time.rs` are currently unreachable from the server; this design connects them.

---

## Decisions

| Topic | Decision | Rationale |
|-------|----------|-----------|
| Locale传递方式 | body 字段 `"locale"` | 与现有接口结构一致，改动最小 |
| 加载策略 | 启动时全量加载 | 流量会覆盖多语言，懒加载只会产生延迟毛刺 |
| locale 缺省行为 | 返回空结果 + 错误日志 | 避免静默 fallback 到错误语言 |
| 不支持的 locale | 返回空结果 + 错误日志 | 同上 |
| request_id 来源 | 客户端 Header `X-Request-ID` | 支持端到端日志串联（移动端 ↔ 服务端） |
| request_id 返回 | 响应 Header `X-Request-ID` | 不污染 body 结构 |

---

## Architecture

```
X-Request-ID: abc-123
POST /parse {"text": "demain matin", "locale": "fr"}
         │
         ▼
  [Handler] 读取 request_id from Header
         │
         ▼
  [PatternNormalizer] normalize(text)
  "demain matin" → "demain matin"（法语无预设规则，原样透传）
         │
         ▼
  [locale 检查]
  ├─ None    → log::warn! [req={}] locale missing → 返回空结果
  └─ Some(s) → LocaleRegistry.get(s)
                   ├─ None    → log::warn! [req={}] unsupported locale → 返回空结果
                   └─ Some(rule_set) → rule_set.apply_all(normalized)
         │
         ▼
  返回 ParseResponse，Header 携带 X-Request-ID
```

---

## New Module: `src/locale/`

### `src/locale/mod.rs`

```rust
pub mod registry;
pub use registry::LocaleRegistry;
```

### `src/locale/registry.rs`

```rust
pub struct LocaleRegistry {
    rule_sets: HashMap<String, Arc<RuleSet<Value>>>,
}

impl LocaleRegistry {
    /// 启动时构建所有语言规则集（阻塞，约 26 * ~20ms）
    pub fn build_all() -> Self;

    /// 查找语言规则集，None 表示不支持该 locale
    pub fn get(&self, locale: &str) -> Option<&Arc<RuleSet<Value>>>;

    /// 列出所有已注册的 locale
    pub fn supported_locales(&self) -> Vec<&str>;
}
```

`build_all()` 内部为每个语言分别构建独立 `RuleSet`：

```rust
// 每个语言的 rule set 包含：
// 1. 基础规则（integer / duration）
// 2. 该语言的 time rules
let b = RuleSetBuilder::new(...);
rules::integer::rules(&b);
rules::duration::rules(&b);
languages::fr::time::rules(&b, Some(Arc::new(TimeContext::default())));
map.insert("fr", Arc::new(b.build()));
```

支持的语言列表（对应 `languages/` 下已有实现）：

```
ar bg ca da de el en es fr ga he hr hu it ja ka ko nb nl pl pt ro ru sv tr uk vi zh
```

---

## Modified: `ParseRequest`

```rust
pub struct ParseRequest {
    pub text: String,
    pub locale: Option<String>,  // 新增
}
```

---

## Modified: `AppState`

```rust
pub struct AppState {
    pub locales: Arc<LocaleRegistry>,           // 新增
    pub rule_set: Arc<RwLock<RuleSet<Value>>>,  // 保留（动态规则热加载用）
    pub config_manager: Arc<Mutex<ConfigManager>>,
    pub pattern_normalizer: Arc<PatternNormalizer>,
    pub dynamic_enabled: bool,
}
```

`static_only()` 构造时调用 `LocaleRegistry::build_all()`。

---

## Modified: Handler 逻辑

```rust
pub async fn parse(
    state: web::Data<AppState>,
    req: web::Json<ParseRequest>,
    http_req: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    let request_id = http_req.headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let rule_set = match &req.locale {
        None => {
            log::warn!("[req={}] locale missing in request, returning empty results", request_id);
            return Ok(empty_response_with_id(request_id));
        }
        Some(locale) => match state.locales.get(locale) {
            None => {
                log::warn!("[req={}] unsupported locale {:?}, returning empty results", request_id, locale);
                return Ok(empty_response_with_id(request_id));
            }
            Some(rs) => Arc::clone(rs),
        }
    };

    // normalize + apply_all + 返回结果，响应 Header 带回 X-Request-ID
}
```

---

## Log Format

```
WARN [req=abc-123] locale missing in request, returning empty results
WARN [req=abc-123] unsupported locale "xx", returning empty results
INFO [req=abc-123] locale=fr matched=2 elapsed=3ms
```

---

## Response

**成功：**
```http
HTTP/1.1 200 OK
X-Request-ID: abc-123
Content-Type: application/json

{"results": [...], "count": 2}
```

**locale 缺失 / 不支持：**
```http
HTTP/1.1 200 OK
X-Request-ID: abc-123
Content-Type: application/json

{"results": [], "count": 0}
```

---

## Files Changed

| 文件 | 变更类型 |
|------|---------|
| `src/locale/mod.rs` | 新建 |
| `src/locale/registry.rs` | 新建 |
| `src/lib.rs` | 新增 `pub mod locale;` |
| `src/server/handlers.rs` | 修改 ParseRequest，handler 逻辑 |
| `src/server/state.rs` | AppState 新增 `locales` 字段 |

---

## Out of Scope

- locale fallback 链（`fr-CA` → `fr`）：预留接口，Phase 4 实现
- 动态规则的 locale 路由：下一阶段设计
- `/parse/batch` 的 locale 支持：同步跟进

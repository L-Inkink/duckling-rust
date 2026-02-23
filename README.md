# Rustling

[![Build Status](https://travis-ci.org/snipsco/rustling.svg?branch=master)](https://travis-ci.org/snipsco/rustling)

Rust port of [Facebook Duckling](https://github.com/facebookincubator/duckling) - a Haskell library for parsing natural language into structured data.

## Features

- **Multi-language Time Parsing** - Supports 28 languages including English, Chinese, French, German, Spanish, Japanese, Korean, and more
- **Numeral Parsing** - Extract numbers from text in multiple languages
- **Duration Parsing** - Recognize time durations (e.g., "5 minutes", "2 hours")
- **Fuzzy Matching** - SmartMatcher with three-layer pipeline:
  - Pattern normalization (abbreviations, typos)
  - Levenshtein distance matching
  - Optional fastText embeddings (feature-gated)
- **Dynamic Rules** - JSON-configurable rules with hot-reload support
- **HTTP Server** - REST API with OpenAPI/Swagger documentation

## Quick Start

```bash
# Build
cargo build

# Run server (requires server feature)
cargo run --features server

# Parse a time expression
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: req-001" \
  -d '{"text": "tomorrow", "locale": "en"}'

# Multi-language support
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "demain", "locale": "fr"}'   # French: "tomorrow"

curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "明天", "locale": "zh"}'       # Chinese: "tomorrow"
```

## Supported Languages

| Code | Language | Status |
|------|----------|--------|
| en | English | ✅ Complete |
| zh | Chinese | ✅ Complete |
| fr | French | ✅ Complete |
| de | German | ✅ Complete |
| es | Spanish | ✅ Complete |
| it | Italian | ✅ Complete |
| ja | Japanese | ✅ Complete |
| ko | Korean | ✅ Complete |
| ar, bg, ca, da, el, ga, he, hr, hu, ka, nb, nl, pl, pt, ro, ru, sv, tr, uk, vi | Other languages | ✅ Complete |

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/parse` | POST | Parse text with locale |
| `/parse/batch` | POST | Batch parse multiple texts |
| `/health` | GET | Health check |
| `/config/status` | GET | Configuration status |
| `/config/reload` | POST | Reload dynamic rules |
| `/api-docs/openapi.json` | GET | OpenAPI schema |
| `/swagger-ui/` | GET | Swagger UI |

## Features

| Feature | Flag | Description |
|---------|------|-------------|
| Server | `--features server` | HTTP API server |
| Apollo | `--features apollo` | Dynamic rules from Apollo config center |
| fastText | `--features fasttext` | ML-based fuzzy matching |
| Migration | `--features migration-tools` | Code generation tools |

## Examples

### Parse Time

```bash
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{
    "text": "in 3 hours",
    "locale": "en"
  }'
```

### Batch Parse

```bash
curl -X POST http://localhost:8080/parse/batch \
  -H "Content-Type: application/json" \
  -d '{
    "texts": ["tomorrow", "next week", "3 days ago"],
    "locale": "en"
  }'
```

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --features server

# Code quality check
cargo clippy

# Run benchmarks
cargo bench
```

## Architecture

```
Input Text
    │
    ▼
┌─────────────────────────────┐
│   PatternNormalizer         │  ← Fuzzy layer 1
│   (abbreviations, typos)   │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   LocaleRegistry           │  ← 28 languages
│   (per-locale rules)       │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   RuleSet::apply_all()     │  ← Core engine
│   (regex pattern matching)  │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   Parse Results           │
│   (time, duration, etc.)  │
└─────────────────────────────┘
```

## License

Apache 2.0 / MIT - see [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT)

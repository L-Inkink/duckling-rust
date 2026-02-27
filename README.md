# Rustling

Rust port of [Facebook Duckling](https://github.com/facebookincubator/duckling) — a Haskell library for parsing natural language into structured data. Extended with HTTP/gRPC server, FFI library, dynamic rules, and 28-language support.

## Features

- **Multi-language Parsing** — 28 languages (EN, ZH, FR, DE, JA, KO, and more), routed via `LocaleRegistry`
- **Multiple Dimensions** — Time, Numeral, Duration, Distance
- **Dual-mode Deployment** — Online (HTTP REST + gRPC) and offline (C FFI / Android JNI)
- **Fuzzy Matching** — SmartMatcher three-layer pipeline:
  - Layer 1: PatternNormalizer (abbreviations, typos)
  - Layer 2: Levenshtein distance (200+ time phrase dictionary)
  - Layer 3: fastText embeddings (optional, `--features fasttext`)
- **Dynamic Rules** — JSON-configurable rules with Apollo hot-reload
- **High Performance** — FFI parsing: 6.6µs; batch: 9.4µs/item; Docker image: 34.3MB

## Quick Start

```bash
# Build
cargo build

# Run HTTP server
cargo run --features server

# Run with Docker
docker-compose up

# Parse a time expression
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: req-001" \
  -d '{"text": "tomorrow at 3pm", "locale": "en"}'

# Multi-language
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "demain matin", "locale": "fr"}'   # French

curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "明天上午9点", "locale": "zh"}'       # Chinese
```

## Architecture

```
Input Text
    │
    ▼
┌──────────────────────────────────────────┐
│  SmartMatcher (Fuzzy Pre-processing)     │
│  PatternNormalizer → Levenshtein         │
│  → FastTextExpander (optional)           │
└──────────────────────────────────────────┘
    │ normalized text
    ▼
┌──────────────────────────────────────────┐
│  LocaleRegistry (28 languages)           │
│  locale → RuleSet lookup (O(1))          │
└──────────────────────────────────────────┘
    │ matched RuleSet
    ▼
┌──────────────────────────────────────────┐
│  Core Engine (rustling-core)             │
│  Terminal rules → Stash → Composition    │
│  → Saturation + Dynamic rules (JSON)     │
└──────────────────────────────────────────┘
    │ ParsedNode[]
    ▼
┌──────────────────────────────────────────┐
│  ML Ranking (rustling-ml, optional)      │
│  Naive Bayes → sorted ParserMatch[]      │
└──────────────────────────────────────────┘
    │ structured results
    ▼
┌─────────────┬─────────────┬─────────────┐
│ HTTP Server │ gRPC Server │  C FFI lib  │
│ (Actix-web) │   (tonic)   │  (.so/.a)   │
│  REST API   │ Protobuf RPC│ Android JNI │
└─────────────┴─────────────┴─────────────┘
```

## Supported Languages

| Code | Language | Code | Language |
|------|----------|------|----------|
| en | English | ko | Korean |
| zh | Chinese | ar | Arabic |
| fr | French | bg, ca, da, el | Bulgarian, Catalan, Danish, Greek |
| de | German | ga, he, hr, hu | Irish, Hebrew, Croatian, Hungarian |
| es | Spanish | ka, nb, nl, pl | Georgian, Norwegian, Dutch, Polish |
| it | Italian | pt, ro, ru, sv | Portuguese, Romanian, Russian, Swedish |
| ja | Japanese | tr, uk, vi | Turkish, Ukrainian, Vietnamese |

## HTTP API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/parse` | POST | Parse text (`{"text": "...", "locale": "en"}`) |
| `/parse/batch` | POST | Batch parse (up to 100 items) |
| `/health` | GET | Health check |
| `/config/status` | GET | Configuration status |
| `/config/reload` | POST | Reload dynamic rules (API key required) |
| `/api-docs/openapi.json` | GET | OpenAPI schema |
| `/swagger-ui/` | GET | Swagger UI |

## gRPC Services

```protobuf
service Parser {
  rpc Parse(ParseRequest) returns (ParseResponse);
  rpc ParseBatch(BatchParseRequest) returns (BatchParseResponse);
  rpc Health(HealthRequest) returns (HealthResponse);
}
```

Enable with `--features grpc`.

### Run gRPC Server

```bash
# Build with gRPC support
cargo build --release --features grpc

# Run server (default port 50051)
cargo run --release --features grpc --bin grpc_server

# Or specify custom address
cargo run --release --features grpc --bin grpc_server -- --addr 0.0.0.0:9000
```

### Test with grpcurl

```bash
# Parse single text
grpcurl -plaintext -d '{"text":"42","locale":"en"}' localhost:50051 duckling.Parser/Parse

# Batch parse
grpcurl -plaintext -d '{"texts":["42","five minutes"],"locale":"en"}' localhost:50051 duckling.Parser/ParseBatch

# Health check
grpcurl -plaintext localhost:50051 duckling.Parser/Health
```

## FFI (Offline / Android)

```c
// Parse text, returns JSON string (caller must free)
char* rustling_parse(const char* text, const char* locale);
void  rustling_free_string(char* s);
void  rustling_free_error(char* s);
char* rustling_version(void);
char* rustling_supported_locales(void);
uint32_t rustling_locale_supported(const char* locale);
void rustling_init(void);
```

### Build FFI Library

```bash
cargo build --release --lib
# Output: target/release/librustling.so (2.7MB) + librustling.a (37MB)
```

### C Example

```bash
# Build and run the C example
./scripts/build_ffi_example.sh --run
```

See [include/rustling.h](include/rustling.h) for the complete C API header.

## Compile Features

| Feature | Flag | Description |
|---------|------|-------------|
| HTTP Server | `--features server` | Actix-web REST API |
| Apollo | `--features apollo` | Dynamic rules from Apollo config center |
| gRPC | `--features grpc` | tonic gRPC server |
| fastText | `--features fasttext` | ML-based fuzzy matching |
| Migration tools | `--features migration-tools` | Haskell → Rust code generation |

## Development

```bash
# Run tests
cargo test

# Check code quality (0 warnings enforced)
cargo clippy

# Run with debug logging
RUST_LOG=debug cargo run --features server

# Run benchmarks
cargo bench

# Build FFI library
cargo build --release --lib

# Build gRPC server
cargo build --release --features grpc
```

## Performance

| Operation | Latency | Throughput |
|-----------|---------|------------|
| FFI: parse integer | 6.6µs | — |
| FFI: parse duration | 9.6µs | — |
| FFI: batch (4 items) | 37.7µs (9.4µs/item) | — |
| gRPC: parse (P50) | **234µs** | — |
| gRPC: parse (P99) | **393µs** | — |
| gRPC: peak throughput | — | **164K req/s** |
| gRPC: sustained (5s) | — | **225K req/s** |
| Core: levenshtein | 0.4µs | — |
| Core: pattern normalize | 0.09µs | — |
| HTTP: single parse | ~25ms | — |
| Docker image size | 34.3MB | — |

See [docs/reports/GRPC_BENCH_REPORT.md](docs/reports/GRPC_BENCH_REPORT.md) for detailed benchmark.

## Documentation

| Doc | Description |
|-----|-------------|
| [docs/architecture/ARCHITECTURE.md](docs/architecture/ARCHITECTURE.md) | System architecture (dual-mode 5-layer) |
| [docs/architecture/MIGRATION_GUIDE.md](docs/architecture/MIGRATION_GUIDE.md) | Haskell → Rust rule migration |
| [docs/guides/DOCKER.md](docs/guides/DOCKER.md) | Docker deployment guide |
| [docs/reports/BENCHMARKS.md](docs/reports/BENCHMARKS.md) | Core performance benchmarks |
| [docs/reports/GRPC_BENCH_REPORT.md](docs/reports/GRPC_BENCH_REPORT.md) | gRPC performance benchmark (164K+ RPS) |
| [docs/plans/PROJECT_ROADMAP_V3.md](docs/plans/PROJECT_ROADMAP_V3.md) | Project roadmap |

## License

Apache 2.0 / MIT — see [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT)

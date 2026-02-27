# Release Notes - v0.30.0

**Release Date**: 2026-02-27

---

## Highlights

This major release brings **dual-mode deployment** (online gRPC + offline FFI), **28-language support**, and **production-ready performance**. The project has evolved from a pure parsing engine into a complete NLP solution with HTTP/gRPC servers, FFI library for Android, and comprehensive benchmarking tools.

---

## New Features

### 🌐 Multi-Language Support (28 Languages)
- **LocaleRegistry**: O(1) language routing with all 28 languages loaded at startup
- **Time dimension**: Full support for EN, ZH, FR, DE, ES, IT, JA, KO, and more
- **Multi-locale batch parsing**: Process texts in different languages in a single request
- **X-Request-ID tracing**: Full request ID propagation across all layers

### ⚡ Dual-Mode Deployment

#### gRPC Server (Online Mode)
- **High-performance gRPC API** using tonic + prost
- **P50 latency: 234µs** (target was 5-10ms — 20x+ better)
- **Peak throughput: 164K req/s** (target was 1K — 164x better)
- **Sustained throughput: 225K req/s** (5-second window)
- Service methods: `Parse`, `ParseBatch`, `Health`
- Run with: `cargo run --features grpc --bin grpc_server`

#### C FFI Library (Offline Mode)
- **6.6µs parse latency** (target 1-10µs ✓)
- Complete C API with header file (`include/rustling.h`)
- Example C program (`examples/ffi_example.c`)
- Builds to `.so` (2.7MB) and `.a` (37MB static)
- Android JNI ready

### 🔧 Core Engine Enhancements

- **SmartMatcher**: Three-layer fuzzy matching pipeline
  - Layer 1: PatternNormalizer (15+ templates, abbrev/typo normalization)
  - Layer 2: LevenshteinMatcher (200+ time phrase dictionary, 0.85 threshold)
  - Layer 3: FastTextExpander (optional, via `--features fasttext`)

- **Dynamic Rules Engine**: JSON-configurable rules with Apollo hot-reload
  - `validate_ruleset()`, `list_rules()`, `list_enabled_rules()`
  - FileLoader / InlineLoader / ApolloLoader (feature-gated)

- **Unified Parse API**: `Parser::parse()` / `parse_batch()` with clean JSON output
  - `ParserConfig` for customization
  - Consistent output format across all interfaces

- **Performance Monitoring**: `AtomicU64` counters + `TimingScope` RAII timer

### 🐳 Production Deployment

- **Docker**: Multi-stage build, 34.3MB image (target <50MB ✓)
- **Docker Compose**: Production + development override configs
- **Non-root user**, health checks, log rotation, auto-restart

---

## Tests & Quality

| Metric | Value |
|--------|-------|
| Test count | **309+ tests** |
| Test pass rate | **100%** |
| Clippy warnings | **0** |
| Unsafe code | Audited, LOW risk |

---

## Breaking Changes

None. This release is fully backward compatible.

---

## Migration from v0.20.0

### If using HTTP Server
```bash
# No changes needed - API remains the same
cargo run --features server
```

### If using gRPC (new in v0.30.0)
```bash
# Build with gRPC feature
cargo build --features grpc

# Run gRPC server
cargo run --features grpc --bin grpc_server

# Or use Docker
docker-compose up
```

### If using FFI (new in v0.30.0)
```bash
# Build library
cargo build --release --lib

# Use in C programs
gcc -o my_app my_app.c -I./include -L./target/release -lrustling
```

---

## Contributors

- L-Inkink (maintainer)
- Claude Code (AI assistant)

---

## Acknowledgments

- [Facebook Duckling](https://github.com/facebookincubator/duckling) - Original Haskell implementation
- [Rustling](https://github.com/sonos/rustling) - Core parsing engine
- Contributors to the 28-language rule implementations

---

## Links

- **Documentation**: [docs/](docs/)
- **Roadmap**: [docs/plans/PROJECT_ROADMAP_V3.md](docs/plans/PROJECT_ROADMAP_V3.md)
- **gRPC Benchmark**: [docs/reports/GRPC_BENCH_REPORT.md](docs/reports/GRPC_BENCH_REPORT.md)
- **GitHub**: https://github.com/L-Inkink/duckling-rust

---

## Internal Changes

### Code Generation
- Multi-language code generation framework (`tools/codegen/`)
- Phase 2.4: Auto-generate Time rules for all 28 languages
- Haskell→Rust rule migration tools

### Documentation Reorganization
- `docs/architecture/` - System architecture docs
- `docs/guides/` - Deployment guides
- `docs/reports/` - Benchmark and audit reports
- `docs/plans/` - Project roadmap and phase tracking

### Performance Optimizations
- PatternNormalizer caching
- LocaleRegistry using `Arc<RwLock<>>` for thread-safe access
- Batch parsing with optimized per-item cost (55.9µs @ batch=8)

# 发布日志 - v0.30.0

**发布日期**: 2026-02-27

---

## 亮点

本版本带来**双模式部署**（在线 gRPC + 离线 FFI）、**28 种语言支持**和**生产级性能**。项目已从纯解析引擎演进为完整的 NLP 解决方案，包含 HTTP/gRPC 服务器、用于 Android 的 FFI 库和全面的基准测试工具。

---

## 新功能

### 🌐 多语言支持（28 种语言）
- **LocaleRegistry**: 启动时加载全部 28 种语言，O(1) 路由
- **时间维度**: 全面支持 EN、ZH、FR、DE、ES、IT、JA、KO 等
- **多语言批量解析**: 单次请求处理不同语言的文本
- **X-Request-ID 追踪**: 全链路请求 ID 透传

### ⚡ 双模式部署

#### gRPC 服务器（在线模式）
- 基于 tonic + prost 的高性能 gRPC API
- **P50 延迟: 234µs**（目标 5-10ms — 提升 20 倍以上）
- **峰值吞吐量: 164K req/s**（目标 1K — 提升 164 倍）
- **持续吞吐量: 225K req/s**（5 秒窗口）
- 服务方法: `Parse`、`ParseBatch`、`Health`
- 运行: `cargo run --features grpc --bin grpc_server`

#### C FFI 库（离线模式）
- **6.6µs 解析延迟**（目标 1-10µs ✓）
- 完整 C API，含头文件（`include/rustling.h`）
- C 示例程序（`examples/ffi_example.c`）
- 构建为 `.so`（2.7MB）和 `.a`（37MB 静态库）
- 支持 Android JNI

### 🔧 核心引擎增强

- **SmartMatcher**: 三层模糊匹配流水线
  - 第一层: PatternNormalizer（15+ 模板，缩写/拼写错误规范化）
  - 第二层: LevenshteinMatcher（200+ 时间短语词典，0.85 阈值）
  - 第三层: FastTextExpander（可选，通过 `--features fasttext`）

- **动态规则引擎**: JSON 可配置规则，支持 Apollo 热重载
  - `validate_ruleset()`、`list_rules()`、`list_enabled_rules()`
  - FileLoader / InlineLoader / ApolloLoader（功能门控）

- **统一解析 API**: `Parser::parse()` / `parse_batch()`，输出清洁 JSON
  - `ParserConfig` 可定制配置
  - 所有接口一致的输出格式

- **性能监控**: `AtomicU64` 计数器 + `TimingScope` RAII 计时器

### 🐳 生产部署

- **Docker**: 多阶段构建，34.3MB 镜像（目标 <50MB ✓）
- **Docker Compose**: 生产配置 + 开发覆盖配置
- **非 root 用户**、健康检查、日志轮转、自动重启

---

## 测试与质量

| 指标 | 数值 |
|------|------|
| 测试数量 | **309+ 测试** |
| 测试通过率 | **100%** |
| Clippy 警告 | **0** |
| unsafe 代码 | 已审计，低风险 |

---

## 破坏性变更

无。此版本完全向后兼容。

---

## 从 v0.20.0 迁移

### 如果使用 HTTP 服务器
```bash
# 无需更改 - API 保持不变
cargo run --features server
```

### 如果使用 gRPC（v0.30.0 新增）
```bash
# 使用 gRPC 功能构建
cargo build --features grpc

# 运行 gRPC 服务器
cargo run --features grpc --bin grpc_server

# 或使用 Docker
docker-compose up
```

### 如果使用 FFI（v0.30.0 新增）
```bash
# 构建库
cargo build --release --lib

# 在 C 程序中使用
gcc -o my_app my_app.c -I./include -L./target/release -lrustling
```

---

## 贡献者

- L-Inkink（维护者）
- Claude Code（AI 助手）

---

## 鸣谢

- [Facebook Duckling](https://github.com/facebookincubator/duckling) - 原始 Haskell 实现
- [Rustling](https://github.com/sonos/rustling) - 核心解析引擎
- 28 种语言规则实现的贡献者

---

## 相关链接

- **文档**: [docs/](docs/)
- **路线图**: [docs/plans/PROJECT_ROADMAP_V3.md](docs/plans/PROJECT_ROADMAP_V3.md)
- **gRPC 压测报告**: [docs/reports/GRPC_BENCH_REPORT.md](docs/reports/GRPC_BENCH_REPORT.md)
- **GitHub**: https://github.com/L-Inkink/duckling-rust

---

## 内部变更

### 代码生成
- 多语言代码生成框架（`tools/codegen/`）
- Phase 2.4: 为全部 28 种语言自动生成时间规则
- Haskell→Rust 规则迁移工具

### 文档重组
- `docs/architecture/` - 系统架构文档
- `docs/guides/` - 部署指南
- `docs/reports/` - 基准测试和安全审计报告
- `docs/plans/` - 项目路线图和阶段追踪

### 性能优化
- PatternNormalizer 缓存
- LocaleRegistry 使用 `Arc<RwLock<>>` 实现线程安全访问
- 批量解析优化每项成本（batch=8 时 55.9µs/项）

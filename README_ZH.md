# Duckling Rust — 自然语言解析引擎

> 基于 [Rustling](https://github.com/sonos/rustling) 扩展的 Duckling Rust 实现，支持服务端（HTTP/gRPC）与移动端（Android FFI）双模式部署。

[![License](https://img.shields.io/badge/license-Apache%202.0%2FMIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

---

## 📖 项目简介

Duckling Rust 是 Facebook Duckling 自然语言解析库的现代化 Rust 实现。在 Rustling 核心引擎基础上扩展为**双模式五层架构**，支持：

- 从文本中提取时间、数字、时长等结构化数据
- 28 种语言，启动时全量加载，O(1) 路由
- 服务端（HTTP REST + gRPC）与离线（C FFI / Android JNI）统一引擎
- 动态规则 JSON 配置 + Apollo 热重载

---

## 🚀 快速开始

```bash
# 编译
cargo build

# 启动 HTTP 服务器
cargo run --features server

# Docker 启动
docker-compose up

# 解析时间表达
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: req-001" \
  -d '{"text": "明天上午9点", "locale": "zh"}'

# 多语言示例
curl -d '{"text": "demain matin", "locale": "fr"}' \
  http://localhost:8080/parse    # 法语：明天早上

curl -d '{"text": "in 3 hours", "locale": "en"}' \
  http://localhost:8080/parse    # 英语：3小时后
```

---

## 🏗️ 架构

```
输入文本
    │
    ▼
┌───────────────────────────────────────────┐
│  Layer 1: SmartMatcher（模糊预处理）       │
│  PatternNormalizer → Levenshtein          │
│  → FastTextExpander（可选 feature）        │
└───────────────────────────────────────────┘
    │ 规范化文本
    ▼
┌───────────────────────────────────────────┐
│  Layer 2: LocaleRegistry（多语种路由）     │
│  28 语言规则集，启动时全量构建，O(1) 查找  │
└───────────────────────────────────────────┘
    │ 对应语言 RuleSet
    ▼
┌───────────────────────────────────────────┐
│  Layer 3: 解析引擎（rustling-core）        │
│  终结规则 → Stash → 组合 → 饱和           │
│  + 动态规则引擎（JSON/Apollo）             │
└───────────────────────────────────────────┘
    │ ParsedNode[]
    ▼
┌───────────────────────────────────────────┐
│  Layer 4: ML 排序（可选）                  │
│  朴素贝叶斯 → 排序后 ParserMatch[]        │
└───────────────────────────────────────────┘
    │ 结构化结果
    ▼
┌─────────────┬─────────────┬──────────────┐
│  HTTP 服务  │  gRPC 服务  │  C FFI 库   │
│ (Actix-web) │  (tonic)    │  (.so/.a)   │
│  REST API   │ Protobuf RPC│ Android JNI │
└─────────────┴─────────────┴──────────────┘
```

---

## 🌍 支持的语言（28 种）

| 代码 | 语言 | 代码 | 语言 |
|------|------|------|------|
| en | English（英语） | ko | Korean（韩语） |
| zh | Chinese（中文） | ar | Arabic（阿拉伯语） |
| fr | French（法语） | bg, ca, da, el | 保加利亚/加泰罗尼亚/丹麦/希腊 |
| de | German（德语） | ga, he, hr, hu | 爱尔兰/希伯来/克罗地亚/匈牙利 |
| es | Spanish（西班牙语） | ka, nb, nl, pl | 格鲁吉亚/挪威/荷兰/波兰 |
| it | Italian（意大利语） | pt, ro, ru, sv | 葡萄牙/罗马尼亚/俄语/瑞典 |
| ja | Japanese（日语） | tr, uk, vi | 土耳其/乌克兰/越南 |

---

## 📡 HTTP API 端点

| 端点 | 方法 | 说明 |
|------|------|------|
| `/parse` | POST | 解析文本（`{"text": "...", "locale": "zh"}`） |
| `/parse/batch` | POST | 批量解析（最多 100 条） |
| `/health` | GET | 健康检查 |
| `/config/status` | GET | 配置状态 |
| `/config/reload` | POST | 重载动态规则（需 API key） |
| `/api-docs/openapi.json` | GET | OpenAPI Schema |
| `/swagger-ui/` | GET | Swagger UI |

---

## 📡 gRPC 服务

```protobuf
service DucklingParser {
  rpc Parse(ParseRequest) returns (ParseResponse);
  rpc ParseBatch(BatchParseRequest) returns (BatchParseResponse);
  rpc Health(HealthRequest) returns (HealthResponse);
}
```

启用：`cargo build --features grpc`

---

## 📱 FFI 库（离线 / Android）

```c
typedef struct {
    char    *json;    // JSON 结果数组（出错时为 NULL）
    uint32_t count;   // 匹配数量
    char    *error;   // 错误信息（成功时为 NULL）
} RustlingParseResult;

RustlingParseResult rustling_parse(const char* text, const char* locale);
void     rustling_free_result(RustlingParseResult result); // 推荐：同时释放 json 和 error
void     rustling_free_string(char* s);                    // 释放 version/locales 字符串
void     rustling_free_error(char* s);                     // 同 free_string
char*    rustling_version(void);
char*    rustling_supported_locales(void);
uint32_t rustling_locale_supported(const char* locale);
void     rustling_init(void);
```

结果中的 `value` 字段为结构化 JSON 对象（如 `{"Integer":42}`、`{"Duration":{"amount":5,"unit":"Minute"}}`），而非调试字符串。

编译：`cargo build --release --lib` → 生成 `librustling.a`（静态）/ `librustling.so`（动态）

---

## 📦 编译特性

| 功能 | 编译选项 | 说明 |
|------|----------|------|
| HTTP 服务器 | `--features server` | Actix-web REST API |
| Apollo | `--features apollo` | 配置中心动态规则热重载 |
| gRPC | `--features grpc` | tonic Protobuf 服务 |
| fastText | `--features fasttext` | ML 词向量模糊匹配 |
| 迁移工具 | `--features migration-tools` | Haskell → Rust 代码生成 |

---

## 🛠️ 开发命令

```bash
# 运行测试
cargo test

# 代码质量检查（0 警告）
cargo clippy

# 带日志启动服务器
RUST_LOG=debug cargo run --features server

# 性能基准测试
cargo bench

# 编译 FFI 库
cargo build --release --lib

# 编译 gRPC 服务器
cargo build --release --features grpc
```

---

## ⚡ 性能

| 操作 | 延迟 | 吞吐量 |
|------|------|--------|
| FFI: 解析整数 | 6.6µs | — |
| FFI: 解析时长 | 9.6µs | — |
| FFI: 批量 4 条 | 37.7µs（9.4µs/条） | — |
| gRPC: 解析 (P50) | **234µs** | — |
| gRPC: 解析 (P99) | **393µs** | — |
| gRPC: 峰值吞吐 | — | **164K req/s** |
| gRPC: 持续吞吐 (5s) | — | **225K req/s** |
| 核心: Levenshtein 距离 | 0.4µs | — |
| 核心: 模式规范化 | 0.09µs | — |
| HTTP: 单次解析 | ~25ms | — |
| Docker 镜像大小 | **34.3MB** | — |

详细报告见 [gRPC 性能压测报告](docs/reports/GRPC_BENCH_REPORT.md)。

---

## ✅ 当前进度

| 阶段 | 内容 | 状态 |
|------|------|------|
| Phase 0 | Rustling 评估与现代化 | ✅ 完成 |
| Phase 2 | HTTP 服务器 + Apollo 集成 | ✅ 完成 |
| Phase 5-A | Docker 容器化（镜像 34.3MB） | ✅ 完成 |
| Phase 1 | 核心功能增强（309 测试，0 Clippy 警告） | ✅ 完成 |
| 多语种路由 | 28 语言 LocaleRegistry | ✅ 完成 |
| Phase 6-B | 双模式架构（gRPC + FFI，PR #7） | ✅ 完成 |
| **Phase 3** | **Android JNI 集成** | 🚧 进行中 |

---

## 📚 文档

| 文档 | 说明 |
|------|------|
| [架构设计](docs/architecture/ARCHITECTURE.md) | 双模式五层架构详解 |
| [Haskell→Rust 迁移指南](docs/architecture/MIGRATION_GUIDE.md) | 规则迁移方法论 |
| [Docker 部署指南](docs/guides/DOCKER.md) | 容器化部署详细说明 |
| [性能基准报告](docs/reports/BENCHMARKS.md) | 核心性能测试数据 |
| [gRPC 性能压测报告](docs/reports/GRPC_BENCH_REPORT.md) | gRPC 164K+ RPS 压测数据 |
| [项目路线图](docs/plans/PROJECT_ROADMAP_V3.md) | 完整开发规划 |

---

## 📜 许可证

Apache 2.0 / MIT — 见 [LICENSE-APACHE](LICENSE-APACHE) 和 [LICENSE-MIT](LICENSE-MIT)

---

**最后更新**: 2026-02-27 | **版本**: v0.11.0

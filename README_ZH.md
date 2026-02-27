# Duckling Rust - 自然语言解析引擎

> 基于 [Rustling](https://github.com/sonos/rustling) 扩展的 Duckling Rust 重构版本

[![License](https://img.shields.io/badge/license-Apache%202.0%2FMIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

---

## 📖 项目简介

Duckling Rust 是 Facebook Duckling 自然语言解析库的现代化 Rust 实现。本项目基于 Sonos 的 Rustling 进行扩展，增加动态规则加载、模糊匹配、HTTP 服务、多语言支持等企业级功能。

**核心功能**:
- 🔍 **自然语言解析** - 从文本中提取时间、数字、金额等结构化数据
- 🌐 **多语言支持** - 支持 28 种语言（中文、英文、法语、德语、日语、韩语等）
- 🤖 **智能模糊匹配** - SmartMatcher 三层流水线：PatternNormalizer → Levenshtein → fastText
- ⚡ **高性能** - Rust 零成本抽象，比 Haskell 版本更快
- 📱 **跨平台** - 服务端（HTTP API）+ Android（JNI）统一引擎
- 🔄 **动态规则** - JSON 配置 + Apollo 热重载，无需重新编译

---

## 🚀 快速开始

### 编译与运行

```bash
# 编译
cargo build

# 运行服务器
cargo run --features server

# 测试解析
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: req-001" \
  -d '{"text": "tomorrow", "locale": "en"}'
```

### 多语言示例

```bash
# 中文
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "明天上午9点", "locale": "zh"}'

# 法语
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "demain matin", "locale": "fr"}'

# 日语
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "あしたの朝", "locale": "ja"}'
```

---

## 🌍 支持的语言

| 代码 | 语言 | 状态 |
|------|------|------|
| en | English | ✅ 完整 |
| zh | Chinese | ✅ 完整 |
| fr | French | ✅ 完整 |
| de | German | ✅ 完整 |
| es | Spanish | ✅ 完整 |
| it | Italian | ✅ 完整 |
| ja | Japanese | ✅ 完整 |
| ko | Korean | ✅ 完整 |
| ar, bg, ca, da, el, ga, he, hr, hu, ka, nb, nl, pl, pt, ro, ru, sv, tr, uk, vi | 其他 20 种语言 | ✅ 完整 |

---

## 📡 API 端点

| 端点 | 方法 | 说明 |
|------|------|------|
| `/parse` | POST | 解析文本（需指定 locale） |
| `/parse/batch` | POST | 批量解析多个文本 |
| `/health` | GET | 健康检查 |
| `/config/status` | GET | 配置状态 |
| `/config/reload` | POST | 重载动态规则 |
| `/api-docs/openapi.json` | GET | OpenAPI 文档 |
| `/swagger-ui/` | GET | Swagger UI |

---

## 🏗️ 架构

```
输入文本
    │
    ▼
┌─────────────────────────────┐
│   PatternNormalizer        │  ← 模糊匹配层 1
│   （缩写、拼写错误规范化）   │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   LocaleRegistry           │  ← 28 语言规则集
│   （按 locale 路由）       │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   RuleSet::apply_all()    │  ← 核心引擎
│   （正则模式匹配）         │
└─────────────────────────────┘
    │
    ▼
┌─────────────────────────────┐
│   解析结果                  │
│   (时间、时长、数字等)     │
└─────────────────────────────┘
```

---

## 📦 Features

| 功能 | 编译选项 | 说明 |
|------|----------|------|
| 服务器 | `--features server` | HTTP API 服务 |
| Apollo | `--features apollo` | Apollo 配置中心动态规则 |
| fastText | `--features fasttext` | ML 词向量模糊匹配 |
| 迁移工具 | `--features migration-tools` | 代码生成工具 |

---

## 🛠️ 开发命令

```bash
# 运行测试
cargo test

# 代码检查
cargo clippy

# 运行服务器（带日志）
RUST_LOG=debug cargo run --features server

# 性能基准测试
cargo bench
```

---

## 📅 当前进度

### Phase 1-2 已完成 ✅

- ✅ 技术债务清理（Clippy 0 警告）
- ✅ 动态规则引擎完善
- ✅ SmartMatcher 三层流水线
- ✅ fastText 集成验证
- ✅ 多语言路由（28 语言）
- ✅ X-Request-ID 全链路追踪

### Phase 3 进行中 🚧

- Android JNI 集成

---

## 📜 许可证

Apache 2.0 / MIT - 见 [LICENSE-APACHE](LICENSE-APACHE) 和 [LICENSE-MIT](LICENSE-MIT)

---

**最后更新**: 2026-02-24
**版本**: v0.10.0

# 文档索引

本目录包含 duckling-rust 项目的所有技术文档。

---

## 📐 架构设计 (`architecture/`)

| 文档 | 说明 |
|------|------|
| [ARCHITECTURE.md](./architecture/ARCHITECTURE.md) | 系统架构全览：三层架构、模块结构、数据流、类型系统 |
| [MIGRATION_GUIDE.md](./architecture/MIGRATION_GUIDE.md) | Haskell → Rust 迁移指南：类型映射、规则迁移、错误处理 |
| [TIMEZONE_ANALYSIS.md](./architecture/TIMEZONE_ANALYSIS.md) | 时区处理机制对比：Duckling（Haskell）vs Rustling（Rust） |
| [TIMECONTEXT_MIGRATION.md](./architecture/TIMECONTEXT_MIGRATION.md) | TimeContext 重构记录：ZH 时间规则从 `Utc::now()` 迁移到 `TimeContext` |
| [ValueKind-关键突破.md](./architecture/ValueKind-关键突破.md) | composite 规则调试关键：使用 `ValueKind` 作为 `StashIndexable::Index` |

## 📦 部署指南 (`guides/`)

| 文档 | 说明 |
|------|------|
| [DOCKER.md](./guides/DOCKER.md) | Docker 部署完整指南：多阶段构建、Compose 编排、健康检查、故障排查 |

## 📊 测试与安全报告 (`reports/`)

| 文档 | 说明 |
|------|------|
| [BENCHMARKS.md](./reports/BENCHMARKS.md) | 核心性能基准测试结果：解析延迟、吞吐量、FFI vs HTTP 对比 |
| [GRPC_BENCH_REPORT.md](./reports/GRPC_BENCH_REPORT.md) | gRPC 大批量性能压测报告：P50=234µs，峰值吞吐 164K RPS，持续 225K RPS |
| [UNSAFE_CODE_AUDIT.md](./reports/UNSAFE_CODE_AUDIT.md) | unsafe 代码审计：2 个代码块分析，风险评级 LOW |

## 🗺️ 项目规划 (`plans/`)

| 文档 | 说明 |
|------|------|
| [PROJECT_ROADMAP_V3.md](./plans/PROJECT_ROADMAP_V3.md) | **主路线图**：项目愿景、已完成阶段、未来规划（Phase 3-8）、成功指标 |

## 📜 发布记录 (`releases/`)

| 版本 | 日期 | 说明 |
|------|------|------|
| [v0.30.0](./releases/RELEASE_v0.30.0.md) | 2026-02-27 | 双模式部署（gRPC + FFI）、28 语言支持、生产级性能 |
| [v0.30.0 (English)](./releases/RELEASE_v0.30.0_ENGLISH.md) | 2026-02-27 | Dual-mode deployment, 28-language support |
| v0.20.0 | 2026-02-14 | Phase 1 核心功能增强、SmartMatcher、动态规则 |

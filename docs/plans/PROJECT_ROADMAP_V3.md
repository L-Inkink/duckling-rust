# Duckling Rust 项目路线图

**版本**: 3.4
**创建日期**: 2026-02-14
**最后更新**: 2026-02-27
**状态**: Phase 0/1/2/5-A/6-B 已完成，多语种路由已完成，Phase 3 Android 交叉编译 + Python ctypes 已完成，JNI Wrapper 待开发

---

## 📍 项目愿景

将 Haskell Duckling 自然语言解析库迁移到 Rust，实现：

1. **跨平台统一** - 一份 Rust 代码，编译为服务端 + Android 库
2. **动态规则配置** - JSON 配置 + Apollo 热重载，无需重新编译
3. **智能模糊匹配** - 容错拼写错误，fastText 词向量扩展
4. **生产级性能** - <10ms 延迟，>1000 req/s 吞吐
5. **多语种支持** - 28 种语言，统一 LocaleRegistry

---

## 🎯 核心架构（当前）

```
┌────────────────────────────────────────────────────────────┐
│              统一规则配置 (rules/*.json)                    │
│   • 中文/英文/其他语言规则 • Apollo 热重载                  │
└────────────────────────────────────────────────────────────┘
                           ↓ 加载
┌────────────────────────────────────────────────────────────┐
│           Rust 核心引擎 (rustling-core)                     │
│   Pattern匹配 • 饱和解析 • SmartMatcher • ML分类器          │
│   LocaleRegistry (28语言) • 动态规则引擎                    │
└────────────────────────────────────────────────────────────┘
         ↓                    ↓                    ↓
   HTTP 服务端            gRPC 服务端           FFI 库(.so)
   (Actix-web)            (tonic)             (C ABI)
   REST API               高性能RPC            Android/JNI
   Swagger UI             Protobuf             离线解析
```

**关键决策**:
- ✅ **基于 Rustling 扩展** - 而非从零重写（节省 4-6 周）
- ✅ **纯 Rust 架构** - 放弃 Haskell FFI 混合方案
- ✅ **先 HTTP 后 Android** - Phase 2 优先实施（已完成）
- ✅ **可选组件** - fastText、Apollo、gRPC 均可独立启用
- ✅ **双模式 FFI** - 在线 gRPC + 离线 C FFI 统一支撑 Android

---

## 📊 已完成阶段

### Phase 0: Rustling 评估与现代化
**完成时间**: 2026-02-10 | **分支**: `phase0-modernization`

- Fork Rustling 并更新依赖到 Rust 2021 edition
- 代码架构分析：确认饱和解析算法和 ML 模块可复用
- **决策**: 采用扩展方案，节省 4-6 周开发时间

---

### Phase 2: HTTP 服务器 + Apollo 集成
**完成时间**: 2026-02-14 | **分支**: `phase2-http-apollo`

- ✅ 6 个 REST API 端点（/parse, /parse/batch, /health, /config/*, /swagger-ui/）
- ✅ Apollo 配置热重载（AtomicU64 版本检测 + Arc<RwLock<>> 原子交换）
- ✅ API key 认证 + 输入验证（10KB/文本，64KB payload）
- ✅ 41 个测试（30 单元 + 11 集成），100% 通过
- ✅ 代码审查修复 14 个问题（3 Critical + 5 High + 6 Medium）

---

### Phase 5-A: Docker 容器化部署
**完成时间**: 2026-02-14 | **分支**: `phase5-docker-deploy`

- ✅ 多阶段 Dockerfile（Builder rust:slim + Runtime debian:bookworm-slim）
- ✅ Docker Compose 编排（生产 + 开发覆盖配置）
- ✅ 非 root 用户执行、日志轮转、自动重启、健康检查
- ✅ Docker 镜像大小：**34.3MB** ✅（目标 <50MB 已达成）
- ✅ 完整部署文档（[docs/guides/DOCKER.md](../guides/DOCKER.md)）

待完成：
- ⏸️ Kubernetes 部署配置（Phase 5-B）
- ⏸️ CI/CD Pipeline（Phase 5-B）
- ⏸️ Prometheus Metrics / 分布式追踪（Phase 5-C）

---

### Phase 1: 核心功能增强
**完成时间**: 2026-02-23 | **分支**: `phase2-time-implementation`
**测试**: 309 个全部通过，Clippy 0 警告

**技术债务清理**
- 修复全仓库 ~300+ Clippy 警告（包含 26 个语言文件批量修复）
- 修复 zh/time.rs `hour % 1` 逻辑 bug → `hour > 0`
- 修复 en/time.rs 11 处 RangeInclusive 模式

**动态规则引擎** (`src/dynamic/`)
- DynamicRule 数据结构：exact/regex（Terminal）+ template 基础支持
- validate_ruleset()、list_rules()、list_enabled_rules()
- FileLoader / InlineLoader / ApolloLoader（feature-gated）

**SmartMatcher 三层流水线** (`src/fuzzy/`)
- Layer 1: PatternNormalizer — 15+ 模板，中英文缩写规范化
- Layer 2: LevenshteinMatcher — 200+ 英中时间短语词典，阈值 0.85
- Layer 3: FastTextExpander — 可选 feature，finalfusion 加载模型

**性能监控** (`src/metrics/`)
- AtomicU64 线程安全计数器 + TimingScope RAII 计时
- METRICS 全局实例（once_cell::Lazy）

---

### 多语种路由
**完成时间**: 2026-02-23 | **分支**: `phase2-time-implementation`

- ✅ `src/locale/registry.rs` — LocaleRegistry，28 语言规则集启动时全量构建
- ✅ ParseRequest / BatchParseRequest 新增 `locale: Option<String>`
- ✅ X-Request-ID Header 全链路透传（请求 → 日志 → 响应）
- ✅ locale 缺失/不支持 → warn log + 空结果（不静默 fallback）

```bash
# 示例
curl -X POST http://localhost:8080/parse \
  -H "X-Request-ID: req-001" \
  -d '{"text": "demain", "locale": "fr"}'
```

---

### Phase 6-B: 双模式架构 — gRPC + FFI
**完成时间**: 2026-02-25 | **PR**: #7 已合并
**测试**: 194 个全部通过

**gRPC 服务端（在线模式）**
- `proto/duckling.proto` — Parse/ParseBatch/Health 服务定义
- `src/server/grpc.rs` — tonic 完整集成，GrpcAppState + ParserService
- `--features grpc` 编译 0 警告

**FFI 库（离线模式）**
- `src/ffi.rs` — rustling_parse / rustling_free_string / rustling_version / rustling_supported_locales / rustling_init
- `crate-type = ["lib", "staticlib", "cdylib"]`
- 生成：librustling.a (37MB 静态库) / librustling.so (2.7MB 动态库)

**统一解析 API**
- `src/parse.rs` — Parser::parse() / parse_batch() / ParserConfig / ParseOutput

**性能基准** (2026-02-27 实测)
```
核心层:   parse integer 0.4µs | levenshtein 0.4µs | normalize 0.09µs
API层:   parse integer 6.6µs | parse duration 9.6µs | batch(4) 37.7µs (9.4µs/item)
FFI:     6.6µs ✅ (目标 1-10µs)
gRPC:    P50=234µs | P99=393µs | 峰值吞吐=164K RPS | 持续=225K RPS ✅ (目标 5-10ms, >1000 RPS)
HTTP:    ~25ms (待压测)
```

**详细报告**: [GRPC_BENCH_REPORT.md](../reports/GRPC_BENCH_REPORT.md)

**2026-02-27 补充完成：**
- [x] gRPC 端到端集成测试（build.rs + tonic-build 生成 proto 代码，TcpListener 随机端口，11 个测试用例覆盖 Parse/ParseBatch/Health 全流程）
- [x] FFI C 示例程序（include/rustling.h 公开头文件，examples/ffi_example.c 演示完整 C API，scripts/build_ffi_example.sh 一键编译运行）
- [x] gRPC 性能基准测试（tools/grpc_bench.rs，单客户端 P50=234µs，100 并发峰值 164K RPS，持续 225K RPS）

---

## 🗺️ 未来路线图

### 🚧 Phase 3: Android JNI 集成（进行中）
**计划**: 2026-02-27 - 2026-03-07

> **前提条件**: FFI 库已就绪（Phase 6-B 已完成 ✅），C ABI 接口可直接用于 JNI

**核心任务**:
```
✅ Rust Android 交叉编译（2026-02-28 完成）
  - 配置 NDK 环境（cargo-ndk，Homebrew cask）
  - 添加 Android 目标（aarch64-linux-android, armv7-linux-androideabi, x86_64-linux-android）
  - 编译 .so 库（arm64: 3.0MB, armeabi: 2.0MB, x86_64: 3.7MB）
  - 工具链安装脚本（scripts/install_android_toolchain.sh）
  - 自动化构建脚本（scripts/build_android.sh --verify）

✅ Python ctypes 包装（2026-02-28 完成，超出计划范围）
  - python/rustling/ffi.py：ctypes 绑定 FfiParseResult，支持 Android/macOS/Linux/Windows
  - python/rustling/__init__.py：Python 包入口，export parse()
  - 桌面验证测试 4/4 通过
  - 架构文档：docs/guides/ANDROID_PYTHON_INTEGRATION.md

□ JNI Wrapper（复用 src/ffi.rs C ABI）
  - JNI 函数导出（Java_com_rustling_NLPParser_parse 等）
  - Java/Kotlin ↔ Rust 类型转换
  - 内存管理（JNI Global Ref）

□ Kotlin 封装类
  - NLPParser — 核心解析器（加载 .so，调用 JNI）
  - DucklingClient — 在线/离线统一接口（降级策略）
  - ParseResult / ParsedValue 数据类

□ Android 示例 App
  - 规则从 assets 加载
  - 离线解析演示
  - 在线 API 降级演示
```

**验收标准**:
- [x] Android .so 库大小 <10MB（各架构）✅ arm64: 3.0MB, armeabi: 2.0MB, x86_64: 3.7MB
- [x] Python ctypes wrapper 桌面验证通过 ✅
- [ ] Android App 能解析 "twenty three" → 23
- [ ] 解析延迟 <50ms（含 JNI 开销）
- [ ] 无内存泄漏（LeakCanary 验证）
- [ ] 无崩溃（测试 1000 次解析）

---

### Phase 4: 多语种扩展 + ML 优化
**计划**: 2026-03-08 - 2026-03-28

**核心任务**:
```
□ 评估 Rustling ML 模块
  - 朴素贝叶斯分类器分析
  - 特征工程评估，训练数据格式

□ 多语种规则迁移
  - 将已有规则转换为 JSON 格式
  - 扩展语言：ES, FR, DE, JA, KO 等
  - Corpus 测试验证

□ ML 分类器优化
  - 用新语种数据重训练
  - 评估更好的模型（XGBoost, LightGBM）
  - 在线学习机制（用户反馈）

□ 规则冲突解决
  - 多语种规则优先级
  - 地区变体处理（en_US vs en_GB）
```

**验收标准**:
- [ ] 支持 5-10 种语言的核心维度
- [ ] 多语种规则测试通过率 >95%
- [ ] ML 分类器准确率提升
- [ ] 多语种解析延迟 <100ms

---

### Phase 5-B/C: 生产部署完善
**计划**: 2026-03-29 - 2026-04-18

```
□ Kubernetes 部署
  - Deployment（滚动更新）+ Service + ConfigMap + Secret
  - HPA（CPU 60% / Memory 70%）+ Ingress（TLS + Rate limiting）

□ CI/CD Pipeline（GitHub Actions）
  - 自动化测试 → Clippy → cargo audit → Docker build → 推送 Registry

□ Prometheus Metrics
  - /metrics 端点
  - http_requests_total / http_request_duration_seconds / parse_results_total
  - ruleset_version / cache_hits_total

□ 可观测性
  - OpenTelemetry + Jaeger（分布式追踪）
  - 结构化日志（JSON + trace_id）
  - Grafana 仪表板（QPS / P50/P95/P99 / 错误率 / 缓存命中率）
  - 告警规则（错误率>5%, P99>100ms, 内存>80%）
```

**验收标准**:
- [ ] K8s 部署成功，HPA 生效
- [ ] CI/CD 自动化流程完整
- [ ] Prometheus metrics 可查询，Grafana 可视化
- [ ] 服务端 QPS >1000（单实例），7x24 稳定运行

---

### Phase 6: 性能优化
**计划**: 2026-04-19 - 2026-05-09

> 注：Phase 6-B（双模式架构）已提前完成；本 Phase 6 专注于服务端性能调优。

```
□ LRU 解析结果缓存
  - 缓存键：blake3(input_text)，容量：10,000 条，TTL：5 分钟
  - 失效策略：规则集更新时清空

□ PatternNormalizer 缓存（减少重复正则匹配）

□ 热路径优化
  - cargo flamegraph 分析 CPU 热点
  - 减少 String 分配（&str）、Vec clone（Arc）
  - RwLock → ArcSwap 减少锁竞争
  - 正则表达式预编译

□ 并发优化（rayon 并行批量解析）

□ 压力测试（wrk / k6）
  - 稳态：1000 req/s × 10 分钟
  - 峰值：5000 req/s × 1 分钟
  - 长跑：24 小时内存泄漏检测
```

**验收标准**:
- [ ] 缓存命中率 >70%
- [ ] P99 延迟 <50ms
- [ ] 吞吐量 >1000 req/s
- [ ] 24h 压力测试无内存泄漏

---

### Phase 7: 功能增强（长期）

#### 7.1 安全加固
- Rate Limiting（governor：全局 1000/s，单 IP 100/s，单 API key 500/s）
- OAuth2 / JWT + RBAC（Admin/User/Guest）
- CORS、HSTS、CSP 安全 headers

#### 7.2 高级 API
- WebSocket 实时解析（/ws/parse，流式语音场景）
- 异步批量任务（POST /tasks → GET /tasks/{id}/result）
- GraphQL API（可选）

#### 7.3 多语言 SDK
- Python SDK（requests + aiohttp，发布 PyPI）
- TypeScript SDK（Fetch API，Node.js + Browser，发布 npm）
- Go SDK（发布 Go modules）

#### 7.4 管理 Web UI
- 规则列表 / 编辑器（Monaco Editor + JSON schema）
- 在线规则测试 + 版本历史（Git-like diff）
- 审计日志（谁/何时/做了什么，CSV/JSON 导出）

---

### Phase 8: 智能化增强（长期）

#### 8.1 ML 优化
- 朴素贝叶斯排序评估与新语种数据重训练
- 更好的模型（XGBoost, LightGBM, ONNX 压缩）
- fastText 模型量化 + 常用词 embedding 缓存
- 自适应学习（用户反馈 → 规则自动生成 → 人工审核）

#### 8.2 多模态支持
- ASR 集成（Whisper API，实时转写）
- OCR 集成（图像文字提取，版面分析）
- 中英混合解析（"明天 3 PM" 代码切换检测）

---

## 📅 时间表与里程碑

### Q1 2026（已完成）

| 时间 | 里程碑 | 状态 |
|------|--------|------|
| Feb 01-10 | Phase 0 — Rustling 评估与现代化 | ✅ |
| Feb 11-14 | Phase 2 — HTTP 服务器 + Apollo 集成 | ✅ |
| Feb 14 | Phase 5-A — Docker 容器化（镜像 34.3MB） | ✅ |
| Feb 23 | Phase 1 — 核心功能增强（309 测试 0 警告） | ✅ |
| Feb 23 | 多语种路由（28 语言 LocaleRegistry） | ✅ |
| Feb 25 | Phase 6-B — 双模式架构（gRPC + FFI，PR #7） | ✅ |

### Q2 2026（规划中）

| 时间 | 里程碑 | 状态 |
|------|--------|------|
| Feb 28 | Phase 3 — Android 交叉编译 + Python ctypes | ✅ 部分完成 |
| Mar 01 - Mar 07 | Phase 3 续 — JNI Wrapper + Kotlin + 示例 App | 🚧 当前 |
| Mar 08 - Mar 28 | Phase 4 — 多语种扩展 + ML 优化 | ⏳ |
| Mar 29 - Apr 18 | Phase 5-B/C — 生产部署完善 | ⏳ |
| Apr 19 - May 09 | Phase 6 — 性能优化 | ⏳ |

### Q3–Q4 2026（长期规划）

| 时间 | 里程碑 |
|------|--------|
| Jun - Jul | Phase 7 — 功能增强（安全/高级API/SDK） |
| Jul - Aug | Phase 7 续 — 管理 Web UI |
| Aug - Oct | Phase 8 — 智能化增强（ML/多模态） |

---

## 🎯 成功指标

### 功能指标

| 指标 | 目标 | 当前 |
|------|------|------|
| 核心维度支持 | 5+ | 4（Time/Numeral/Duration/Distance） |
| 支持语言数 | 5-10 | **28** ✅ |
| 动态规则类型 | 3+ | 2（exact/regex）+ template 基础 |
| API 端点数 | 8+ | 7（含 gRPC 3 个） |
| 模糊匹配准确率 | >85% | **>85%** ✅（"tomorow"→0.875） |

### 性能指标

| 指标 | 目标 | 当前 |
|------|------|------|
| FFI 解析延迟 | <10µs | **6.6µs** ✅ |
| gRPC 解析延迟 P50 | <10ms | **234µs** ✅ (超越 40x) |
| gRPC 解析延迟 P99 | <50ms | **393µs** ✅ |
| gRPC 吞吐量峰值 | >1000 req/s | **164K r/s** ✅ (超越 164x) |
| gRPC 吞吐量持续 | - | **225K r/s** |
| 解析延迟 P99 | <50ms | 待 HTTP 压测 |
| 吞吐量 | >1000 req/s | 待 HTTP 压测 |
| 缓存命中率 | >70% | 未实现 |
| 内存使用 | <200MB | ~80MB |
| Docker 镜像大小 | <50MB | **34.3MB** ✅ |

### 质量指标

| 指标 | 目标 | 当前 |
|------|------|------|
| Clippy 警告 | 0 | **0** ✅ |
| 测试通过率 | 100% | **100%** ✅ |
| 测试数量 | — | 194（当前主分支） |

---

## 🚨 技术债务

### 已清零（Phase 1 ✅）

| 项目 | 状态 |
|------|------|
| 全仓库 ~300+ Clippy 警告 | ✅ 已清零 (2026-02-23) |
| MSRV 不兼容（LazyLock） | ✅ 改用 once_cell |
| zh/time.rs `hour % 1` bug | ✅ 修复为 `> 0` |
| 26 个语言文件未使用导入 | ✅ 批量修复 |

### 待处理（Phase 3 前）

| 项目 | 影响 | 优先级 |
|------|------|--------|
| API 文档补全（docs/API.md） | 可维护性 | P2 |
| 依赖审计（cargo audit） | 安全性 | P1 |
| ~~gRPC 端到端测试~~（已完成 2026-02-27） | ~~功能验证~~ | ~~P1~~ |
| ~~FFI C 示例程序~~（已完成 2026-02-27） | ~~集成验证~~ | ~~P2~~ |

---

## 🔄 关键决策记录

| # | 决策 | 日期 | 理由 |
|---|------|------|------|
| 1 | 基于 Rustling 扩展，而非从零重写 | 2026-02-10 | 节省 4-6 周，ML 模块已实现 |
| 2 | 纯 Rust 架构，放弃 Haskell FFI 混合 | 2026-02-10 | 降低复杂度，统一技术栈 |
| 3 | Phase 2（HTTP）优先于 Phase 1 实施 | 2026-02-13 | 需要尽快上线，Phase 2 不依赖动态规则 |
| 4 | fastText 作为可选 feature | 2026-02-13 | 模型大（5-50MB），推理有延迟，非必须 |
| 5 | 双模式架构：gRPC（在线）+ FFI（离线） | 2026-02-25 | 统一支撑服务端和 Android 两种部署场景 |

---

## 📚 关键文档索引

| 文档 | 说明 |
|------|------|
| [ARCHITECTURE.md](../architecture/ARCHITECTURE.md) | 系统架构全览（含最新双模式架构） |
| [MIGRATION_GUIDE.md](../architecture/MIGRATION_GUIDE.md) | Haskell → Rust 规则迁移指南 |
| [TIMEZONE_ANALYSIS.md](../architecture/TIMEZONE_ANALYSIS.md) | 时区处理机制分析 |
| [TIMECONTEXT_MIGRATION.md](../architecture/TIMECONTEXT_MIGRATION.md) | TimeContext 重构记录 |
| [ValueKind-关键突破.md](../architecture/ValueKind-关键突破.md) | composite 规则调试关键 |
| [DOCKER.md](../guides/DOCKER.md) | Docker 部署完整指南 |
| [BENCHMARKS.md](../reports/BENCHMARKS.md) | 性能基准测试报告 |
| [GRPC_BENCH_REPORT.md](../reports/GRPC_BENCH_REPORT.md) | gRPC 大批量性能压测报告 |
| [UNSAFE_CODE_AUDIT.md](../reports/UNSAFE_CODE_AUDIT.md) | unsafe 代码安全审计 |
| [tools/migration/README.md](../../tools/migration/README.md) | 代码生成工具链说明 |

---

**版本**: 3.4 | **维护者**: Claude Code | **下次更新**: Phase 3 完成时

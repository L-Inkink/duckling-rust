# Duckling Rust 项目路线图

**版本**: 2.0
**创建日期**: 2026-02-14
**最后更新**: 2026-02-14
**状态**: Phase 2 完成，Phase 3 规划中

---

## 📍 当前位置

### 已完成阶段

- ✅ **Phase 0** (2周) - Rustling 评估与现代化
  - 代码架构分析
  - 依赖更新到 Rust 2021
  - 性能基准测试
  - 决策：采用扩展方案而非重写

- ✅ **Phase 2** (3周) - HTTP 服务器与 Apollo 集成
  - 6个 REST API 端点（/parse, /parse/batch, /health, /config/*, /swagger-ui）
  - Apollo 配置热重载（版本检测 + 原子交换）
  - OpenAPI/Swagger 文档
  - 41 个测试（100% 通过率）
  - 代码质量修复（14个问题全部解决）

### 进行中阶段

- 🚧 **Phase 1** (部分完成) - 核心功能增强
  - ✅ 基础架构（Value 枚举、规则引擎）
  - ⚠️ 待完成：动态规则引擎、模糊匹配、fastText 集成
  - ⚠️ 待完成：性能监测模块

**当前分支**: `phase2-http-apollo` (领先远程 1 个提交)
**技术债务**: 3个 Clippy 警告（MSRV 不兼容）

---

## 🎯 整体目标

将 Rustling 扩展为**生产级的自然语言解析服务**，支持：

1. **动态规则管理** - Apollo 配置中心热更新
2. **智能匹配** - 模糊匹配 + fastText 词向量
3. **高性能** - <10ms P50延迟，>1000 req/s 吞吐
4. **可观测性** - Prometheus metrics，分布式追踪
5. **云原生** - Docker/K8s 部署，自动扩缩容
6. **企业级** - 认证授权，审计日志，多租户

---

## 📅 详细路线图

### Phase 1 (剩余工作) - 核心功能完善

**时间**: 2-3 周
**优先级**: P0 (阻塞其他工作)
**状态**: 部分完成

#### Week 1: 动态规则引擎 (5-7 天)

**目标**: 实现 JSON 规则加载与热更新

```
✅ 已完成:
- Value 枚举定义（Integer, Duration, Time, DateTime）
- 基础规则引擎架构

❌ 待完成:
□ DynamicRule 数据结构
  - JSON Schema 定义
  - 模式类型：exact, regex, template
  - 优先级排序机制

□ JSON 解析器
  - serde_json 集成
  - Schema 验证
  - 错误处理

□ 规则匹配器
  - 精确匹配
  - 正则表达式匹配
  - 模板占位符匹配（{number}, {time}）

□ 规则管理 API
  - 加载规则: load_rules(json_str)
  - 重载规则: reload_rules(json_str)
  - 列出规则: list_rules()

□ 单元测试
  - 10+ 规则类型测试
  - 优先级测试
  - 边界情况测试
```

**交付物**:
- `src/dynamic/rule.rs` - 规则数据结构
- `src/dynamic/loader.rs` - JSON 加载器
- `src/dynamic/matcher.rs` - 规则匹配器
- `src/dynamic/engine.rs` - 动态规则引擎
- `tests/dynamic_rules_test.rs` - 集成测试

#### Week 2: 模糊匹配 (4-5 天)

**目标**: 实现编辑距离纠错与模板正则化

```
□ PatternNormalizer
  - 5-10 个高频模式（如 "明(天|日)(早|晨)" → "明天早上"）
  - 正则替换引擎
  - 可配置模式库

□ LevenshteinMatcher
  - 编辑距离算法（优化版本）
  - 阈值配置（默认 0.85）
  - 候选词库（200-500 个常用短语）

□ SmartMatcher 集成
  - 三层匹配策略
  - 性能优化（缓存）
  - 配置管理
```

**交付物**:
- `src/fuzzy/pattern_normalizer.rs`
- `src/fuzzy/levenshtein.rs`
- `src/fuzzy/smart_matcher.rs`

#### Week 3: fastText + 性能监测 (5-7 天)

**目标**: 词向量扩展与性能分析

```
□ fastText 集成（可选特性）
  - 添加 finalfusion 依赖
  - 模型下载与压缩（<50MB）
  - 相似度搜索（候选词向量预计算）
  - 阈值配置（默认 0.85）

□ PerformanceMetrics
  - 各模块计时（AtomicU64）
  - 报告生成（占比分析）
  - 重置功能

□ 基准测试
  - 100+ 真实输入测试
  - 延迟分布统计
  - 模块耗时分析
```

**交付物**:
- `src/fasttext/expander.rs`
- `src/metrics/mod.rs`
- `benches/fuzzy_bench.rs`

**Phase 1 验收标准**:
- [ ] 动态规则支持 3+ 种模式类型
- [ ] 模糊匹配准确率 >85%
- [ ] 解析延迟 P50 <10ms
- [ ] 测试覆盖率 >80%
- [ ] 文档完整（API + 配置指南）

---

### Phase 3 - 生产就绪

**时间**: 3-4 周
**优先级**: P0 (上线必需)
**依赖**: Phase 1 完成

#### Week 1-2: 容器化 & CI/CD (7-10 天)

**任务清单**:

```
□ Docker 容器化
  - 多阶段 Dockerfile（Builder + Runtime）
  - 镜像优化（目标 <50MB）
  - 健康检查集成
  - Docker Compose 配置

□ Kubernetes 部署
  - Deployment 清单（滚动更新）
  - Service (ClusterIP + LoadBalancer)
  - ConfigMap（配置管理）
  - Secret（API key 管理）
  - HPA（自动扩缩容，CPU 60%/Memory 70%）
  - Ingress（TLS + Rate limiting）

□ CI/CD Pipeline
  - GitHub Actions 工作流
    - 自动化测试（cargo test）
    - 代码质量检查（cargo clippy）
    - 安全审计（cargo audit）
    - Docker 镜像构建
    - 镜像推送到 Registry
  - 版本管理（语义化版本）
  - 自动化部署（测试环境 + 生产环境）
```

**参考 Dockerfile**:
```dockerfile
FROM rust:1.70-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --features server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/http_server /usr/local/bin/
ENV RUST_LOG=info
ENV BIND_ADDRESS=0.0.0.0:8080
EXPOSE 8080
HEALTHCHECK --interval=30s CMD curl -f http://localhost:8080/health || exit 1
CMD ["http_server"]
```

#### Week 3-4: 监控 & 可观测性 (7-10 天)

**任务清单**:

```
□ Prometheus Metrics
  - 添加依赖：prometheus = "0.13"
  - /metrics 端点
  - 核心指标：
    - http_requests_total (counter, by endpoint/status)
    - http_request_duration_seconds (histogram)
    - parse_results_total (counter, by value_type)
    - ruleset_version (gauge)
    - memory_usage_bytes (gauge)

□ 分布式追踪
  - OpenTelemetry 集成
  - Jaeger exporter
  - Span 埋点（HTTP → 解析 → 规则匹配）

□ 日志优化
  - 结构化日志（JSON 格式）
  - trace_id 关联
  - 敏感信息脱敏
  - 动态日志级别

□ Grafana 仪表板
  - 请求吞吐量
  - 延迟分布（P50/P95/P99）
  - 错误率趋势
  - 资源使用（CPU/内存）

□ 告警规则
  - 错误率 > 5%
  - P99 延迟 > 100ms
  - 内存使用 > 80%
  - 热重载失败
```

**Phase 3 验收标准**:
- [ ] Docker 镜像 <50MB
- [ ] K8s 部署成功，HPA 生效
- [ ] CI/CD 自动化流程完整
- [ ] Prometheus metrics 可查询
- [ ] Grafana 仪表板可视化
- [ ] 告警规则触发正常

---

### Phase 4 - 性能优化

**时间**: 2-3 周
**优先级**: P1 (性能关键)
**依赖**: Phase 3 完成

#### Week 1: 缓存实现 (5-7 天)

**目标**: 将热数据延迟降至 <1ms

```
□ LRU 解析结果缓存
  - 依赖：lru = "0.12"
  - 缓存键：input_hash(text)
  - 缓存值：Vec<ParsedNode>
  - 容量：10,000 条（可配置）
  - TTL：5 分钟（可配置）
  - 失效策略：规则集更新时清空

□ PatternNormalizer 缓存
  - 归一化结果缓存
  - 减少重复正则匹配

□ 缓存监控
  - cache_hits_total (counter)
  - cache_misses_total (counter)
  - cache_size_bytes (gauge)
  - 命中率目标：>70%
```

**预期效果**:
- 热数据延迟：<1ms（当前 ~5-10ms）
- 内存增长：~100MB（10K 条缓存）
- 缓存命中率：70-80%

#### Week 2-3: 热路径优化 (7-10 天)

**目标**: P99 延迟 <50ms, 吞吐量 >1000 req/s

```
□ Profiling 分析
  - cargo flamegraph 火焰图
  - 识别 CPU 热点
  - 内存分配分析

□ 优化方向
  - 减少 String 分配（使用 &str）
  - 减少 Vec clone（使用 Arc）
  - 正则表达式预编译
  - RwLock → ArcSwap（减少锁竞争）

□ 并发优化
  - 评估 rayon 并行处理
  - 批量请求并行解析

□ 压力测试
  - wrk / k6 工具
  - 测试场景：
    - 稳态负载（1000 req/s，持续 10 分钟）
    - 峰值负载（5000 req/s，持续 1 分钟）
    - 长时间运行（24 小时，检测内存泄漏）
```

**性能目标**:
```
基准环境：4C8G, Rust 1.70, Linux
- P50 延迟：<10ms ✅
- P99 延迟：<50ms ✅
- 吞吐量：>1000 req/s ✅
- 内存使用：<200MB RSS ✅
- 错误率：<0.01% ✅
- CPU 使用：<60% (稳态)
```

**Phase 4 验收标准**:
- [ ] 缓存命中率 >70%
- [ ] P99 延迟 <50ms
- [ ] 吞吐量 >1000 req/s
- [ ] 24h 压力测试无内存泄漏
- [ ] 性能基准报告完整

---

### Phase 5 - 功能增强

**时间**: 4-6 周
**优先级**: P2 (功能扩展)
**依赖**: Phase 4 完成

#### 5.1 安全加固 (1 周)

```
□ Rate Limiting
  - 依赖：governor / actix-limitation
  - 策略：
    - 全局：1000 req/s
    - 单 IP：100 req/s
    - 单 API key：500 req/s
  - HTTP 429 响应 + Retry-After header

□ 认证增强
  - OAuth2 / JWT 支持
  - RBAC（Admin/User/Guest）
  - 多租户隔离

□ CORS 配置
  - 可配置 allowed_origins
  - 预检请求处理

□ 输入安全
  - 字符集白名单
  - XSS 防护（Content-Security-Policy）
```

#### 5.2 高级 API (2-3 周)

```
□ WebSocket 实时解析
  - /ws/parse 端点
  - 流式处理（语音输入场景）
  - 心跳机制

□ 异步批量任务
  - POST /tasks - 提交任务
  - GET /tasks/{id} - 查询状态
  - GET /tasks/{id}/result - 获取结果
  - WebSocket 进度推送

□ 多语言 SDK
  - Python SDK（requests + 类型提示）
  - TypeScript SDK（fetch + 类型定义）
  - Go SDK（http client + 结构体）
  - 示例代码与文档
```

**Python SDK 示例**:
```python
from rustling import RustlingClient

client = RustlingClient("http://localhost:8080")

# 单次解析
result = client.parse("明早8点")
print(result.results[0].value)  # DateTime(2026-02-15 08:00)

# 批量解析
results = client.parse_batch(["5 minutes", "3 hours"])
```

#### 5.3 管理功能 (2-3 周)

```
□ 规则管理 Web UI
  - React/Vue 前端
  - 功能页面：
    - 规则列表（查看、搜索、过滤）
    - 规则编辑器（JSON + 语法高亮）
    - 规则测试（在线测试）
    - 版本历史

□ 后端 API
  - GET /admin/rules - 列出规则
  - POST /admin/rules - 创建规则
  - PUT /admin/rules/{id} - 更新规则
  - DELETE /admin/rules/{id} - 删除规则

□ 配置版本管理
  - 版本历史（保留最近 N 个）
  - 一键回滚
  - A/B 测试（按百分比路由）

□ 审计日志
  - 操作日志（谁、何时、做了什么）
  - 时间/类型/用户筛选
  - 导出功能
```

**Phase 5 验收标准**:
- [ ] Rate limiting 生效
- [ ] WebSocket 实时解析可用
- [ ] Python/TS SDK 发布
- [ ] Web UI 可访问
- [ ] 审计日志完整

---

### Phase 6 - 智能化增强 (长期)

**时间**: 6-8 周
**优先级**: P3 (研究性质)
**依赖**: Phase 5 完成

#### 6.1 机器学习优化 (3-4 周)

```
□ 朴素贝叶斯排序
  - 训练数据收集（用户反馈）
  - 特征工程
  - 模型训练（smartcore / Python）
  - 在线学习（定期重训练）

□ fastText 优化
  - 模型压缩（量化）
  - 多语言支持（中英联合模型）
  - 缓存常用词 embedding

□ 自适应学习
  - 跟踪用户修正
  - 规则自动生成（人工审核）
  - 个性化解析
```

#### 6.2 多模态支持 (3-4 周)

```
□ 语音输入
  - ASR 集成（Whisper）
  - 实时转写
  - 声学特征利用（语调、停顿）

□ OCR 集成
  - 图像文字提取
  - 版面分析（表格、列表）

□ 多语言混合
  - 中英混合（"明天 3 PM"）
  - 代码切换检测
  - 分段解析
```

---

## 📊 技术债务清单

### 高优先级（立即处理）

| 项目 | 影响 | 预估时间 | 优先级 |
|------|------|---------|--------|
| Clippy 警告修复 | 代码质量 | 2h | P0 |
| Tokio features 瘦身 | 编译时间 | 1h | P1 |
| MSRV 兼容（LazyLock） | 兼容性 | 2h | P1 |

### 中优先级（Phase 3 前）

| 项目 | 影响 | 预估时间 | 优先级 |
|------|------|---------|--------|
| 测试覆盖率提升至 80% | 质量保障 | 1 周 | P1 |
| API 文档补全 | 可维护性 | 2-3 天 | P2 |
| 依赖审计（cargo audit） | 安全性 | 1 天 | P1 |

---

## 🎯 里程碑 & 时间表

### Q1 2026 (当前季度)

- ✅ Phase 0: Rustling 评估（2周）- 已完成
- ✅ Phase 2: HTTP 服务器（3周）- 已完成
- 🚧 Phase 1: 核心功能完善（2-3周）- 进行中
  - 预计完成日期：2026-03-07

### Q2 2026

- **Week 1-4**: Phase 3 生产就绪
  - Week 1-2: Docker + K8s + CI/CD
  - Week 3-4: Prometheus + OpenTelemetry
  - 预计完成：2026-04-05

- **Week 5-7**: Phase 4 性能优化
  - Week 5: LRU 缓存实现
  - Week 6-7: 热路径优化 + 压力测试
  - 预计完成：2026-04-26

### Q3 2026

- **Week 1-4**: Phase 5 功能增强（第一轮）
  - Week 1: 安全加固（Rate limiting, OAuth）
  - Week 2-4: WebSocket + 批量任务
  - 预计完成：2026-05-24

- **Week 5-8**: Phase 5 功能增强（第二轮）
  - Week 5-8: 多语言 SDK + Web UI
  - 预计完成：2026-06-21

### Q4 2026

- **Week 1-8**: Phase 6 智能化增强
  - Week 1-4: ML 优化（NB 排序，fastText 压缩）
  - Week 5-8: 多模态支持（语音，OCR）
  - 预计完成：2026-08-16

---

## 🚀 立即行动计划（下2周）

### Week 1 (2026-02-17 - 2026-02-23)

**目标**: 完成 Phase 1 动态规则引擎

```
□ Day 1-2: 技术债务清理
  - 修复 3 个 Clippy 警告
  - 优化 Tokio features
  - 补充单元测试至 80%

□ Day 3-4: DynamicRule 实现
  - JSON Schema 定义
  - serde_json 解析
  - 规则数据结构

□ Day 5-7: 规则匹配器
  - exact/regex/template 匹配
  - 优先级排序
  - 集成测试
```

### Week 2 (2026-02-24 - 2026-03-02)

**目标**: 完成模糊匹配与 fastText

```
□ Day 1-2: PatternNormalizer
  - 5-10 个高频模式
  - 正则替换引擎

□ Day 3-4: LevenshteinMatcher
  - 编辑距离算法
  - 候选词库（200-500 词）

□ Day 5-7: fastText + 性能监测
  - finalfusion 集成
  - PerformanceMetrics 实现
  - 基准测试
```

**交付物**:
- Phase 1 完整功能
- 文档更新
- 性能报告
- PR 合并到 master

---

## 📈 成功指标

### 技术指标

| 指标 | 目标值 | 当前值 | 截止日期 |
|------|--------|--------|---------|
| 解析延迟 P50 | <10ms | ~5-10ms | Phase 1 |
| 解析延迟 P99 | <50ms | ~20-30ms | Phase 4 |
| 吞吐量 | >1000 req/s | ~500 req/s | Phase 4 |
| 测试覆盖率 | >80% | ~73% | Phase 1 |
| 缓存命中率 | >70% | 0% (未实现) | Phase 4 |
| 内存使用 | <200MB | ~80MB | Phase 4 |
| Docker 镜像大小 | <50MB | 未构建 | Phase 3 |

### 业务指标

| 指标 | 目标值 | 截止日期 |
|------|--------|---------|
| 上线时间 | Q2 2026 | Phase 3 完成 |
| API 可用性 | 99.9% | Phase 3 完成 |
| 动态规则数量 | 50+ | Phase 5 完成 |
| SDK 下载量 | 1000+/月 | Phase 5 后 3 个月 |

---

## 🔗 相关文档

### 已完成文档

- [Phase 0 评估报告](../../PHASE0_评估报告.md) - Rustling 分析
- [执行摘要](../../执行摘要.md) - 高层决策
- [Phase 1 设计](./2026-02-13-phase1-design.md) - 核心功能设计
- [Phase 2 计划](./2026-02-13-phase2-http-apollo.md) - HTTP 服务器
- [Phase 2 状态](./PHASE2_STATUS.md) - 完成报告
- [代码审查](./CODE_REVIEW_2026-02-14.md) - 质量修复
- [优化方向](./FUTURE_IMPROVEMENTS.md) - 32个优化任务

### 待创建文档

- [ ] `PHASE1_STATUS.md` - Phase 1 进度跟踪
- [ ] `PHASE3_PLAN.md` - 生产部署详细计划
- [ ] `PERFORMANCE_REPORT.md` - 性能基准报告
- [ ] `SECURITY_AUDIT.md` - 安全审计报告
- [ ] `API_DOCUMENTATION.md` - 完整 API 文档

---

## 💡 关键决策记录

### 决策 1: 采用扩展方案而非重写

**日期**: 2026-02-10
**决策**: 基于 Rustling 扩展，而非从零重写
**理由**:
- 节省 4-6 周开发时间
- 降低技术风险
- ML 模块已完整实现

### 决策 2: 混合架构（静态 + 动态规则）

**日期**: 2026-02-13
**决策**: 静态规则（Rust）+ 动态规则（Apollo）
**理由**:
- 核心规则性能优先（静态编译）
- 业务规则灵活配置（动态加载）
- Apollo 可选启用，不增加强依赖

### 决策 3: Phase 1 和 Phase 2 调整顺序

**日期**: 2026-02-14
**决策**: 先实施 Phase 2 (HTTP 服务器)，后补完 Phase 1
**理由**:
- Phase 2 更紧急（需要尽快上线）
- Phase 2 不依赖 Phase 1 的动态规则
- Phase 1 可以基于 Phase 2 的反馈优化

### 决策 4: fastText 作为可选特性

**日期**: 2026-02-13
**决策**: fastText 默认禁用，通过 feature flag 启用
**理由**:
- 模型较大（5-50MB）
- 推理有延迟（~5ms）
- 非核心功能，部分场景不需要

---

## 🤝 团队协作

### 当前角色

- **架构设计**: Claude Code
- **代码实现**: Claude Code + 人工审核
- **测试验证**: 自动化测试 + 人工验证
- **文档维护**: Claude Code

### 沟通机制

- **进度更新**: 每周 `PHASE*_STATUS.md` 更新
- **技术决策**: `PROJECT_ROADMAP.md` 决策记录章节
- **代码审查**: PR review + `CODE_REVIEW_*.md`
- **问题跟踪**: GitHub Issues（未来）

---

## 📝 更新日志

### v2.0 (2026-02-14)

- 整合 Phase 0/1/2 的规划文档
- 整合 FUTURE_IMPROVEMENTS.md 的 32 个任务
- 重新组织为 6 个 Phase
- 添加详细的时间表和里程碑
- 添加技术债务清单
- 添加成功指标跟踪

---

**文档维护者**: Claude Code
**最后更新**: 2026-02-14
**版本**: 2.0
**下次更新**: Phase 1 完成时

# Duckling Rust 项目路线图

**版本**: 3.0
**创建日期**: 2026-02-14
**最后更新**: 2026-02-14
**状态**: Phase 0-2 已完成，Phase 5-A (Docker) 已完成，Phase 1 待补完

---

## 📍 项目愿景

将 Haskell Duckling 自然语言解析库迁移到 Rust，实现：

1. **跨平台统一** - 一份 Rust 代码，编译为服务端 + Android 库
2. **动态规则配置** - JSON 配置 + Apollo 热重载，无需重新编译
3. **智能模糊匹配** - 容错拼写错误，fastText 词向量扩展
4. **生产级性能** - <10ms 延迟，>1000 req/s 吞吐
5. **多语种支持** - 复用已有的多语种规则扩展经验

---

## 🎯 核心架构

```
┌──────────────────────────────────────────────────────┐
│        统一规则配置 (rules/*.json)                    │
│   • 中文/英文/其他语言规则                            │
│   • 服务端和移动端共享相同JSON配置                    │
│   • 支持从Apollo等配置中心动态加载                    │
└──────────────────────────────────────────────────────┘
                         ↓ 加载
┌──────────────────────────────────────────────────────┐
│         Rust 核心引擎 (rustling-core)                 │
│   • Pattern匹配 • 饱和解析 • 模糊匹配 • ML分类器     │
│   • 基于Rustling扩展，统一实现                        │
└──────────────────────────────────────────────────────┘
         ↓                              ↓
    编译为二进制                   编译为.so库
         ↓                              ↓
┌──────────────────┐            ┌──────────────────┐
│  Rust HTTP服务   │            │  Android客户端    │
│  (Actix-web)     │            │  (JNI调用.so)    │
│  • 在线解析API   │            │  • 离线本地解析   │
│  • Apollo规则    │            │  • 在线降级       │
└──────────────────┘            └──────────────────┘
```

**关键决策**:
- ✅ **基于 Rustling 扩展** - 而非从零重写（节省 4-6 周）
- ✅ **纯 Rust 架构** - 放弃 Haskell FFI 混合方案
- ✅ **先HTTP后Android** - Phase 2 优先实施（已完成）
- ✅ **可选组件** - fastText、Apollo 均可独立启用

---

## 📊 当前进度

### ✅ 已完成阶段

#### Phase 0: Rustling 评估与现代化 (2周)
**完成时间**: 2026-02-10
**主要成果**:
- Fork Rustling 项目到 `/Users/link/Project/duckling-rust`
- 更新依赖到 Rust 2021 edition
- 代码架构分析和评估报告
- **决策**: 采用扩展方案，节省 4-6 周开发时间

**文档输出**:
- [PHASE0_EVALUATION.md](../../PHASE0_EVALUATION.md) - 英文评估报告
- [PHASE0_评估报告.md](../../PHASE0_评估报告.md) - 中文评估报告
- [执行摘要.md](../../执行摘要.md) - 高层决策文档

---

#### Phase 2: HTTP 服务器 + Apollo 集成 (3周)
**完成时间**: 2026-02-14
**主要成果**:
- ✅ 6个 REST API 端点
  - POST /parse - 单次解析（最大 10KB）
  - POST /parse/batch - 批量解析（最多 100 项）
  - GET /health - 健康检查
  - GET /config/status - 配置状态
  - POST /config/reload - 配置重载（API key 认证）
  - GET /swagger-ui/ - 交互式 API 文档

- ✅ Apollo 配置热重载
  - 版本检测机制（AtomicU64）
  - Arc<RwLock<>> 原子交换
  - 后台任务轮询（可配置间隔）

- ✅ 代码质量修复
  - 修复 14 个问题（3 Critical + 5 High + 6 Medium）
  - API key 认证
  - 输入验证（10KB/文本，64KB payload）
  - 异步/阻塞分离（web::block）

- ✅ 测试覆盖
  - 41 个测试（30 单元 + 11 集成）
  - 100% 通过率
  - 覆盖所有端点和热重载功能

**文档输出**:
- [2026-02-13-phase2-http-apollo.md](./2026-02-13-phase2-http-apollo.md) - Phase 2 计划
- [PHASE2_STATUS.md](./PHASE2_STATUS.md) - 完成报告
- [CODE_REVIEW_2026-02-14.md](./CODE_REVIEW_2026-02-14.md) - 代码审查修复
- [examples/http_server.rs](../../examples/http_server.rs) - 可运行示例

**当前分支**: `phase2-http-apollo` (领先远程 1 个提交)

---

#### Phase 5-A: Docker 容器化部署 (部分完成)
**完成时间**: 2026-02-14
**主要成果**:
- ✅ Docker 多阶段构建配置
  - Builder 阶段：rust:1.70-slim（编译优化）
  - Runtime 阶段：debian:bookworm-slim（最小化镜像）
  - 非 root 用户执行（rustling:1000）
  - 二进制文件 strip 优化
  - 内置健康检查（/health 端点，30s 间隔）

- ✅ Docker Compose 编排
  - Production 配置（资源限制：2 CPU / 512MB 内存）
  - Development 覆盖配置（调试日志 + 预设 API key）
  - 日志轮转（最大 10MB，保留 3 个文件）
  - 自动重启策略
  - 健康检查集成

- ✅ 部署文档
  - 快速启动指南
  - 环境变量配置说明
  - 健康检查和监控
  - 生产环境最佳实践
  - API 端点参考
  - 故障排查指南

**文件清单**:
- `Dockerfile` - 多阶段构建配置（82 行）
- `docker-compose.yml` - 生产编排配置（62 行）
- `docker-compose.dev.yml` - 开发环境覆盖（32 行）
- `.dockerignore` - 构建上下文优化（51 行）
- `.env.example` - 环境变量模板（19 行）
- `docs/DOCKER.md` - 部署文档（410 行）

**当前分支**: `phase5-docker-deploy` (已提交)
**镜像大小目标**: ~80MB（优化目标 <50MB）

**待完成**:
- ⏸️ Kubernetes 部署配置（计划 Phase 5-B）
- ⏸️ CI/CD Pipeline（计划 Phase 5-B）
- ⏸️ Prometheus Metrics（计划 Phase 5-C）
- ⏸️ 分布式追踪（计划 Phase 5-C）

---

### 🚧 部分完成阶段

#### Phase 1: 核心功能增强 (预计 2-3周，已完成 20%)
**状态**: 基础架构已建立，核心功能待实现

**已完成**:
- ✅ Value 枚举定义（Integer, Duration, Time, DateTime, TimeRange）
- ✅ 基础规则引擎架构（基于 Rustling）
- ✅ 项目结构搭建（src/dynamic/, src/fuzzy/, src/server/）

**待完成** (按优先级):

##### 1. 动态规则引擎 (P0, 5-7天)
```
□ DynamicRule 数据结构
  - JSON Schema 定义
  - 模式类型：exact, regex, template
  - 优先级排序机制
  - 启用/禁用控制

□ JSON 解析器与验证
  - serde_json 集成
  - Schema 验证器
  - 错误处理和降级

□ 规则匹配器
  - 精确字符串匹配
  - 正则表达式匹配
  - 模板占位符匹配 ({number}, {time})
  - 约束验证（min/max, type checking）

□ 规则管理 API
  - load_rules(json_str) - 加载规则
  - reload_rules(json_str) - 热重载
  - list_rules() - 列出规则
  - validate_rule(rule) - 验证规则
```

**JSON Schema 示例**:
```json
{
  "version": "1.0.0",
  "metadata": {
    "name": "custom-rules",
    "locale": "en_US"
  },
  "rules": [
    {
      "id": "morning_rush",
      "type": "Terminal",
      "name": "morning rush hour",
      "pattern": "早高峰",
      "capture_group": 0,
      "value": {
        "kind": "TimeRange",
        "start_hour": 7,
        "end_hour": 9
      },
      "enabled": true,
      "priority": 100
    },
    {
      "id": "at_oclock",
      "type": "Template",
      "name": "at X o'clock",
      "pattern": "at {number} o'clock",
      "constraints": {
        "number": {"type": "integer", "min": 1, "max": 24}
      },
      "value": {
        "kind": "Time",
        "hour": "{number}",
        "minute": 0
      },
      "enabled": true,
      "priority": 90
    }
  ]
}
```

##### 2. 模糊匹配模块 (P0, 4-5天)
```
□ PatternNormalizer
  - 5-10 个高频模板（如 "明(天|日)(早|晨)" → "明天早上"）
  - 正则替换引擎
  - 可配置模式库
  - 缓存归一化结果

□ LevenshteinMatcher
  - 编辑距离算法（优化版）
  - 阈值配置（默认 0.85）
  - 候选词库（200-500 个常用时间短语）
  - 滑动窗口匹配

□ SmartMatcher 集成
  - 三层匹配策略
    1. Pattern normalization (必需)
    2. Levenshtein fuzzy match (必需)
    3. fastText expansion (可选)
  - 性能优化（LRU 缓存）
  - 配置管理
```

**模糊匹配示例**:
```rust
pub struct SmartMatcher {
    pattern_normalizer: PatternNormalizer,
    levenshtein: FuzzyMatcher,
    fasttext_expander: Option<Arc<FastTextExpander>>,
    config: MatcherConfig,
}

pub struct MatcherConfig {
    pub enable_pattern_norm: bool,     // 默认 true
    pub enable_levenshtein: bool,      // 默认 true
    pub enable_fasttext: bool,         // 默认 false
    pub levenshtein_threshold: f32,    // 默认 0.85
    pub fasttext_threshold: f32,       // 默认 0.85
}

// 使用示例
let matcher = SmartMatcher::new(MatcherConfig::default());
let normalized = matcher.normalize("明早8点");
// "明早" → "明天早上" (pattern normalization)
// "tomorow" → "tomorrow" (levenshtein, similarity=0.875)
```

##### 3. fastText 集成 (P1, 可选，5-7天)
```
□ fastText 模型加载
  - 依赖：finalfusion = "0.17"
  - 下载预训练模型（中文/英文）
  - 模型压缩（量化，<50MB）

□ 词向量相似度搜索
  - 预计算候选词向量（200-500 个）
  - 实时相似度查询（cosine similarity）
  - 阈值过滤（默认 0.85）

□ 缓存优化
  - 缓存常用词 embedding
  - LRU 缓存机制
  - 启动预热
```

**fastText 示例**:
```rust
// 预计算候选词库
let candidates = vec![
    "明天早上", "明天早晨", "明天晚上",
    "今天早上", "今天晚上", "后天",
];

// 运行时查询
let expander = FastTextExpander::new("models/zh_time.bin", candidates)?;
let expansions = expander.expand("明早", 0.85);
// 返回: [("明天早上", 0.92), ("明天早晨", 0.88)]
```

##### 4. 性能监测模块 (P2, 3-4天)
```
□ PerformanceMetrics 实现
  - 各模块计时（AtomicU64）
  - 线程安全计数器
  - 占比分析

□ 报告生成
  - JSON/Text 格式输出
  - 模块耗时占比
  - 平均/P50/P95/P99 延迟

□ 重置和导出
  - metrics_reset() - 重置统计
  - metrics_export() - 导出数据
  - Prometheus 集成（后续）
```

**Phase 1 验收标准**:
- [ ] 动态规则支持 3+ 种模式类型（exact, regex, template）
- [ ] 模糊匹配准确率 >85%（"tomorow" → "tomorrow"）
- [ ] 解析延迟 P50 <10ms
- [ ] 规则热加载：JSON 文件修改后 <1秒生效
- [ ] 测试覆盖率 >80%
- [ ] 文档完整（API 文档 + 配置指南）

---

## 🗺️ 未来路线图

### Phase 3: Android JNI 集成 (1-2周)

**目标**: Android 应用可调用相同的 Rust .so 库

**先决条件检查**:
- [ ] 检查 Rustling 是否已有 FFI 接口
- [ ] 评估现有 FFI 设计是否满足需求

**核心任务**:
```
□ Rust Android 交叉编译
  - 配置 NDK 环境
  - 添加 Android 目标（aarch64, armv7, x86_64）
  - 编译 .so 库

□ C FFI 接口实现（如未有）
  - duckling_parser_new(rules_json)
  - duckling_parse(parser, text, locale, out_tokens)
  - duckling_parser_free(parser)

□ JNI Wrapper（C++）
  - JNI 函数导出
  - Java/Rust 类型转换
  - 内存管理

□ Kotlin 封装类
  - DucklingParser - 核心解析器
  - DucklingClient - 在线/离线统一接口
  - Token/Value 数据类

□ Android 示例 App
  - 规则从 assets 加载
  - 离线解析演示
  - 在线API降级
```

**验收标准**:
- [ ] Android App 能解析 "twenty three" → 23
- [ ] .so 库大小 <10MB（所有架构）
- [ ] 解析延迟 <50ms
- [ ] 无内存泄漏（valgrind 验证）
- [ ] 无崩溃（测试 1000 次）

---

### Phase 4: 多语种扩展 + ML优化 (2-3周)

**目标**: 迁移已有多语种规则，优化 ML 分类器

**核心任务**:
```
□ 评估 Rustling ML 模块
  - 朴素贝叶斯分类器分析
  - 特征工程评估
  - 训练数据格式

□ 多语种规则迁移
  - 将已有规则转换为 JSON 格式
  - 支持语言：ES, FR, DE, JA, KO 等
  - Corpus 测试验证

□ ML 分类器优化
  - 用新语种数据重训练
  - 评估更好的模型（XGBoost, LightGBM）
  - 在线学习机制（用户反馈）

□ 规则冲突解决
  - 多语种规则优先级
  - 地区变体处理（en_US vs en_GB）
  - 模糊匹配跨语言支持
```

**验收标准**:
- [ ] 支持 5-10 种语言的核心维度
- [ ] 多语种规则测试通过率 >95%
- [ ] ML 分类器准确率提升（相比原版）
- [ ] 多语种解析延迟 <100ms

---

### Phase 5: 生产部署 + 监控 (2-3周)

**目标**: 生产环境部署，完整运维体系

#### Week 1-2: 容器化 & CI/CD

```
✅ Docker 容器化 (已完成 2026-02-14)
  - 多阶段 Dockerfile（Builder + Runtime）
  - 镜像优化（当前 ~80MB，目标 <50MB）
  - 健康检查集成
  - Docker Compose 配置（生产 + 开发）
  - 完整部署文档（docs/DOCKER.md）

□ Kubernetes 部署
  - Deployment（滚动更新策略）
  - Service (ClusterIP + LoadBalancer)
  - ConfigMap（配置管理）
  - Secret（API key 管理）
  - HPA（自动扩缩容，CPU 60%/Memory 70%）
  - Ingress（TLS + Rate limiting）

□ CI/CD Pipeline
  - GitHub Actions 工作流
    - 自动化测试（cargo test）
    - 代码质量（cargo clippy）
    - 安全审计（cargo audit）
    - Docker 镜像构建
    - 镜像推送到 Registry
  - 版本管理（语义化版本）
  - 自动化部署（测试 + 生产环境）
```

#### Week 2-3: 监控 & 可观测性

```
□ Prometheus Metrics
  - /metrics 端点
  - 核心指标：
    - http_requests_total (counter)
    - http_request_duration_seconds (histogram)
    - parse_results_total (counter, by value_type)
    - ruleset_version (gauge)
    - memory_usage_bytes (gauge)
    - cache_hits_total / cache_misses_total

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
  - 缓存命中率

□ 告警规则
  - 错误率 > 5%
  - P99 延迟 > 100ms
  - 内存使用 > 80%
  - 热重载失败
```

**验收标准**:
- [⏸] Docker 镜像 <50MB (当前 ~80MB，待进一步优化)
- [ ] K8s 部署成功，HPA 生效
- [ ] CI/CD 自动化流程完整
- [ ] Prometheus metrics 可查询
- [ ] Grafana 仪表板可视化
- [ ] 告警规则触发正常
- [ ] 服务端 QPS >1000（单实例）
- [ ] 7x24 运行稳定

---

### Phase 6: 性能优化 (2-3周)

**目标**: P99延迟 <50ms, 吞吐量 >1000 req/s

#### Week 1: 缓存实现

```
□ LRU 解析结果缓存
  - 依赖：lru = "0.12"
  - 缓存键：blake3(input_text)
  - 缓存值：Vec<ParsedNode>
  - 容量：10,000 条（可配置）
  - TTL：5 分钟（可配置）
  - 失效策略：规则集更新时清空

□ PatternNormalizer 缓存
  - 归一化结果缓存
  - 减少重复正则匹配
  - 启动预热

□ 缓存监控
  - cache_hits_total (counter)
  - cache_misses_total (counter)
  - cache_hit_ratio (gauge)
  - cache_size_bytes (gauge)
  - 命中率目标：>70%
```

**预期效果**:
- 热数据延迟：<1ms（当前 ~5-10ms）
- 内存增长：~100MB（10K 条缓存）
- 缓存命中率：70-80%

#### Week 2-3: 热路径优化

```
□ Profiling 分析
  - cargo flamegraph - 火焰图
  - perf/valgrind - 性能分析
  - 识别 CPU 热点
  - 内存分配分析

□ 优化方向
  - 减少 String 分配（使用 &str）
  - 减少 Vec clone（使用 Arc）
  - 正则表达式预编译
  - RwLock → ArcSwap（减少锁竞争）
  - 避免不必要的序列化

□ 并发优化
  - 评估 rayon 并行处理
  - 批量请求并行解析
  - 线程池调优

□ 压力测试
  - wrk / k6 / Gatling
  - 测试场景：
    - 稳态负载（1000 req/s，10分钟）
    - 峰值负载（5000 req/s，1分钟）
    - 长时间运行（24小时，内存泄漏检测）
  - 性能报告
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

**验收标准**:
- [ ] 缓存命中率 >70%
- [ ] P99 延迟 <50ms
- [ ] 吞吐量 >1000 req/s
- [ ] 24h 压力测试无内存泄漏
- [ ] 性能基准报告完整
- [ ] Flamegraph 分析文档

---

### Phase 7: 功能增强 (4-6周)

#### 7.1 安全加固 (1周)

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
  - 密钥轮换机制

□ CORS 配置
  - 可配置 allowed_origins
  - 预检请求处理
  - 安全 headers（HSTS, CSP）

□ 输入安全
  - 字符集白名单
  - XSS 防护
  - SQL 注入防护（如使用数据库）
```

#### 7.2 高级 API (2-3周)

```
□ WebSocket 实时解析
  - /ws/parse 端点
  - 流式处理（语音输入场景）
  - 心跳机制
  - 断线重连

□ 异步批量任务
  - POST /tasks - 提交任务
  - GET /tasks/{id} - 查询状态
  - GET /tasks/{id}/result - 获取结果
  - DELETE /tasks/{id} - 取消任务
  - WebSocket 进度推送

□ GraphQL API（可选）
  - Schema 定义
  - Query/Mutation 实现
  - 订阅（实时解析）
```

#### 7.3 多语言 SDK (2-3周)

```
□ Python SDK
  - requests 封装
  - 类型提示（Type hints）
  - 异步支持（aiohttp）
  - 发布到 PyPI

□ TypeScript SDK
  - Fetch API 封装
  - 完整类型定义
  - Node.js + Browser 支持
  - 发布到 npm

□ Go SDK
  - HTTP 客户端封装
  - 结构体定义
  - 示例代码
  - 发布到 Go modules
```

**Python SDK 示例**:
```python
from rustling import RustlingClient

client = RustlingClient("http://localhost:8080", api_key="xxx")

# 单次解析
result = client.parse("明早8点", locale="zh_CN")
print(result.entities[0].value)  # DateTime(2026-02-15 08:00)

# 批量解析
results = client.parse_batch(
    texts=["5 minutes", "3 hours", "tomorrow"],
    locale="en_US"
)

# 异步支持
async with RustlingClient.async_client(url) as client:
    result = await client.parse_async("tomorrow at 3pm")
```

#### 7.4 管理 Web UI (2-3周)

```
□ 前端框架
  - React / Vue / Svelte
  - TypeScript
  - TailwindCSS / Material UI

□ 功能页面
  - 规则列表（查看、搜索、过滤）
  - 规则编辑器（Monaco Editor + JSON schema）
  - 规则测试（在线测试、batch 测试）
  - 版本历史（Git-like diff）
  - 性能监控（实时图表）

□ 后端 API
  - GET /admin/rules - 列出规则
  - POST /admin/rules - 创建规则
  - PUT /admin/rules/{id} - 更新规则
  - DELETE /admin/rules/{id} - 删除规则
  - GET /admin/rules/{id}/history - 版本历史
  - POST /admin/rules/{id}/rollback - 回滚

□ 配置版本管理
  - Git 集成（自动提交）
  - 版本历史（最近 N 个）
  - 一键回滚
  - A/B 测试（按百分比路由）

□ 审计日志
  - 操作日志（谁、何时、做了什么）
  - 时间/类型/用户筛选
  - 导出功能（CSV/JSON）
```

**Phase 7 验收标准**:
- [ ] Rate limiting 生效
- [ ] WebSocket 实时解析可用
- [ ] Python/TS/Go SDK 发布
- [ ] Web UI 可访问，规则可视化编辑
- [ ] 审计日志完整

---

### Phase 8: 智能化增强 (6-8周，长期)

#### 8.1 机器学习优化 (3-4周)

```
□ 朴素贝叶斯排序（Rustling 已有）
  - 评估现有实现
  - 新语种数据训练
  - 在线学习机制

□ 更好的 ML 模型（可选）
  - XGBoost / LightGBM
  - 离线训练 + 在线推理
  - 模型压缩（ONNX）

□ fastText 优化
  - 模型压缩（量化）
  - 多语言联合模型
  - 缓存常用词 embedding

□ 自适应学习
  - 用户反馈收集
  - 规则自动生成（人工审核）
  - 个性化解析
```

#### 8.2 多模态支持 (3-4周)

```
□ 语音输入
  - ASR 集成（Whisper API）
  - 实时转写
  - 声学特征利用（语调、停顿）

□ OCR 集成
  - 图像文字提取
  - 版面分析（表格、列表）
  - 手写识别

□ 多语言混合
  - 中英混合（"明天 3 PM"）
  - 代码切换检测
  - 分段解析
```

---

## 📅 时间表与里程碑

### Q1 2026 (当前季度)

- ✅ Week 1-2 (Feb 01-14): Phase 0 - Rustling 评估
- ✅ Week 3-5 (Feb 15-28): Phase 2 - HTTP 服务器（提前实施）
- 🚧 Week 6-8 (Mar 01-21): **Phase 1 - 核心功能完善** ← 当前阶段
  - Week 1 (Mar 01-07): 动态规则引擎 + 技术债务清理
  - Week 2 (Mar 08-14): 模糊匹配 + LevenshteinMatcher
  - Week 3 (Mar 15-21): fastText + 性能监测

### Q2 2026

- Week 9-10 (Mar 22 - Apr 04): **Phase 3 - Android JNI 集成**
- Week 11-13 (Apr 05 - Apr 25): **Phase 4 - 多语种扩展 + ML优化**
- Week 14-16 (Apr 26 - May 16): **Phase 5 - 生产部署 + 监控**
- Week 17-19 (May 17 - Jun 06): **Phase 6 - 性能优化**

### Q3 2026

- Week 20-25 (Jun 07 - Jul 18): **Phase 7 - 功能增强**
  - Week 20: 安全加固
  - Week 21-23: 高级 API (WebSocket, 批量任务)
  - Week 24-25: 多语言 SDK

### Q4 2026

- Week 26-29 (Jul 19 - Aug 15): **Phase 7 续 - 管理 Web UI**
- Week 30-37 (Aug 16 - Oct 10): **Phase 8 - 智能化增强**
  - Week 30-33: ML 优化
  - Week 34-37: 多模态支持

**总时间**: 约 37 周（8.5 个月）

---

## 🎯 成功指标

### 功能指标

| 指标 | 目标值 | 当前值 | 截止日期 |
|------|--------|--------|---------|
| 核心维度支持 | 5+ | 4 | Phase 4 |
| 支持语言数 | 5-10 | 2 (en, zh) | Phase 4 |
| 动态规则类型 | 3+ (exact/regex/template) | 0 | Phase 1 |
| API 端点数 | 8+ | 6 | Phase 7 |
| 模糊匹配准确率 | >85% | 未实现 | Phase 1 |

### 性能指标

| 指标 | 目标值 | 当前值 | 截止日期 |
|------|--------|--------|---------|
| 解析延迟 P50 | <10ms | ~5-10ms | Phase 1 |
| 解析延迟 P99 | <50ms | ~20-30ms | Phase 6 |
| 吞吐量 | >1000 req/s | ~500 req/s | Phase 6 |
| 缓存命中率 | >70% | 0% (未实现) | Phase 6 |
| 内存使用 | <200MB | ~80MB | Phase 6 |
| Docker 镜像大小 | <50MB | 未构建 | Phase 5 |

### 质量指标

| 指标 | 目标值 | 当前值 | 截止日期 |
|------|--------|--------|---------|
| 测试覆盖率 | >80% | ~73% | Phase 1 |
| Clippy 警告 | 0 | 3 | Phase 1 Week 1 |
| 测试通过率 | 100% | 100% | 持续 |
| 文档完整度 | >90% | ~60% | Phase 5 |
| API 可用性 | 99.9% | - | Phase 5 |

---

## 🚨 技术债务清单

### 高优先级（立即处理，Phase 1 Week 1）

| 项目 | 位置 | 影响 | 预估时间 | 优先级 |
|------|------|------|---------|--------|
| Clippy 警告修复 | fuzzy/levenshtein.rs, metrics/mod.rs | 代码质量 | 2h | P0 |
| MSRV 不兼容（LazyLock） | metrics/mod.rs:181 | 兼容性 | 2h | P0 |
| Tokio features 瘦身 | Cargo.toml | 编译时间 | 1h | P1 |
| 测试覆盖率提升至 80% | 各模块 | 质量保障 | 1 周 | P1 |

### 中优先级（Phase 3 前）

| 项目 | 影响 | 预估时间 | 优先级 |
|------|------|---------|--------|
| API 文档补全 | 可维护性 | 2-3 天 | P2 |
| 依赖审计（cargo audit） | 安全性 | 1 天 | P1 |
| 特性门控优化 | 编译灵活性 | 2 天 | P2 |

**特性门控优化**:
```toml
[features]
default = []
server = ["actix-web", "actix-rt", "tokio", "utoipa", "utoipa-swagger-ui"]
apollo = ["reqwest"]
fasttext = ["finalfusion", "dirs"]
```

---

## 📋 立即行动计划（下 2 周）

### Week 1 (2026-02-17 - 2026-02-23): 技术债务 + 动态规则引擎

**Day 1-2: 技术债务清理**
```
□ 修复 3 个 Clippy 警告
  - fuzzy/levenshtein.rs: needless-range-loop (2处)
  - metrics/mod.rs: unnecessary-cast (1处)

□ 解决 MSRV 兼容性
  - 选项1：升级 MSRV 到 1.80
  - 选项2：使用 once_cell crate

□ Tokio features 优化
  - 从 features = ["full"] 改为精确特性
  - 预期：编译时间减少 ~20%

□ 运行完整测试
  - cargo test
  - cargo clippy
  - cargo audit
```

**Day 3-4: DynamicRule 数据结构**
```
□ 创建 src/dynamic/rule.rs
  - DynamicRule struct
  - DynamicPattern enum (Exact/Regex/Template)
  - OutputTemplate struct
  - Constraint validation

□ 创建 src/dynamic/loader.rs
  - JSON Schema 定义
  - serde_json 解析
  - Schema 验证器
  - 错误处理

□ 单元测试
  - 测试各种规则类型解析
  - 测试约束验证
  - 测试错误情况
```

**Day 5-7: 规则匹配器**
```
□ 创建 src/dynamic/matcher.rs
  - Exact 匹配实现
  - Regex 匹配实现
  - Template 匹配实现（占位符 {number}, {time}）

□ 创建 src/dynamic/engine.rs
  - DynamicRuleEngine struct
  - load_rules(json) 方法
  - apply_rules(text) 方法
  - 优先级排序

□ 集成测试
  - tests/dynamic_rules_test.rs
  - 10+ 规则类型测试
  - 边界情况测试
```

**交付物**:
- src/dynamic/ 模块完整
- 15+ 单元测试
- 集成测试通过
- 技术债务清零

### Week 2 (2026-02-24 - 2026-03-02): 模糊匹配 + fastText

**Day 1-2: PatternNormalizer**
```
□ 创建 src/fuzzy/pattern_normalizer.rs
  - 定义 5-10 个高频模板
  - 正则替换引擎
  - 可配置模式库（JSON）

□ 测试用例
  - "明(天|日)(早|晨)" → "明天早上"
  - "明(天|日)(晚|夜)" → "明天晚上"
  - 英文模式测试
```

**Day 3-4: LevenshteinMatcher**
```
□ 创建 src/fuzzy/levenshtein.rs
  - 编辑距离算法（优化版）
  - 滑动窗口匹配
  - 阈值配置

□ 候选词库
  - 200-500 个常用时间短语
  - 中文/英文分离
  - JSON 配置

□ 测试用例
  - "tomorow" → "tomorrow" (0.875)
  - "yestrday" → "yesterday" (0.875)
  - 中文测试
```

**Day 5-7: SmartMatcher + fastText**
```
□ 创建 src/fuzzy/smart_matcher.rs
  - 集成 PatternNormalizer
  - 集成 LevenshteinMatcher
  - 可选 fastText 集成
  - LRU 缓存

□ fastText (可选)
  - finalfusion 依赖
  - 模型加载（可选 feature）
  - 相似度搜索

□ 性能监测
  - src/metrics/mod.rs
  - 各模块计时
  - 报告生成

□ 基准测试
  - benches/fuzzy_bench.rs
  - 100+ 真实输入测试
  - 延迟分布统计
```

**交付物**:
- src/fuzzy/ 模块完整
- SmartMatcher 可用
- 20+ 单元测试
- 性能报告
- Phase 1 完成 ✅

---

## 📚 关键文档索引

### 规划文档
- [humming-wishing-kahan.md](~/.claude/plans/humming-wishing-kahan.md) - 原始项目记忆
- [PHASE0_EVALUATION.md](../../PHASE0_EVALUATION.md) - Phase 0 评估报告
- [执行摘要.md](../../执行摘要.md) - 高层决策文档
- [Phase 1 设计](./2026-02-13-phase1-design.md) - 核心功能设计
- [Phase 2 计划](./2026-02-13-phase2-http-apollo.md) - HTTP 服务器
- [FUTURE_IMPROVEMENTS.md](./FUTURE_IMPROVEMENTS.md) - 32 个优化任务

### 状态报告
- [PHASE2_STATUS.md](./PHASE2_STATUS.md) - Phase 2 完成报告
- [CODE_REVIEW_2026-02-14.md](./CODE_REVIEW_2026-02-14.md) - 代码质量修复

### 技术文档
- [ARCHITECTURE.md](../../ARCHITECTURE.md) - 架构文档
- [MIGRATION_GUIDE.md](../../MIGRATION_GUIDE.md) - Haskell → Rust 迁移指南
- [BENCHMARKS.md](../../BENCHMARKS.md) - 性能基准

### 参考 Rustling
- **项目地址**: https://github.com/sonos/rustling
- **关键模块**:
  - `core/` - 核心规则引擎
  - `ml/` - ML 模型集成
  - `src/` - 主程序入口

### 参考 Haskell Duckling
- **Engine**: `/Users/link/Project/duckling/Duckling/Engine.hs`
- **Types**: `/Users/link/Project/duckling/Duckling/Types.hs`
- **Rules**: `/Users/link/Project/duckling/Duckling/{Numeral,Time}/*/Rules.hs`

---

## 🔄 决策记录

### 决策 1: 基于 Rustling 扩展
**日期**: 2026-02-10
**决策**: 采用扩展方案，而非从零重写
**理由**: 节省 4-6 周，降低技术风险，ML 模块已实现
**影响**: Phase 1 从"重写引擎"改为"增强功能"

### 决策 2: 纯 Rust 架构
**日期**: 2026-02-10
**决策**: 放弃 Haskell FFI 混合方案
**理由**: 降低复杂度，统一技术栈，利用已有多语种经验
**影响**: 无需维护 Haskell 桥接层

### 决策 3: Phase 2 优先实施
**日期**: 2026-02-13
**决策**: 先实施 HTTP 服务器（Phase 2），后补完 Phase 1
**理由**: 需要尽快上线，Phase 2 不依赖 Phase 1 动态规则
**影响**: Phase 1 基于 Phase 2 反馈优化

### 决策 4: fastText 作为可选特性
**日期**: 2026-02-13
**决策**: fastText 默认禁用，通过 feature flag 启用
**理由**: 模型较大（5-50MB），推理有延迟（~5ms），非所有场景需要
**影响**: 编译灵活性提高，部署更轻量

---

## 🎓 团队协作

### 沟通机制
- **进度更新**: 每周 `PHASE*_STATUS.md` 更新
- **技术决策**: 本文档决策记录章节
- **代码审查**: PR review + `CODE_REVIEW_*.md`
- **问题跟踪**: GitHub Issues（未来）

### 文档维护
- **责任人**: Claude Code
- **更新频率**:
  - 本文档：每 Phase 完成时更新
  - 状态报告：每周更新
  - 技术债务：实时更新

---

**文档版本**: 3.0
**创建日期**: 2026-02-14
**最后更新**: 2026-02-14
**维护者**: Claude Code
**下次更新**: Phase 1 完成时

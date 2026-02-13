# Phase 1 设计文档：核心功能增强

**版本**: 1.0
**日期**: 2026-02-13
**作者**: Claude Code
**状态**: 待评审

---

## 执行摘要

Phase 1 将在 Phase 0 现代化的基础上，实现 Duckling Rust 的核心功能增强，包括：

1. **JSON 规则加载** - 支持动态规则配置
2. **模糊匹配** - 智能缩写词还原与拼写纠错
3. **热重载** - Apollo 配置中心集成

**关键决策**：
- ✅ 混合架构：静态规则（Rust）+ 动态规则（Apollo）
- ✅ 增强方案：基础模糊匹配 + 可选 fastText 智能扩展
- ✅ 可选组件：Apollo 和 fastText 均可独立启用/禁用
- ✅ 性能监测：内置各模块耗时统计

**预期成果**：
- 2-3 周完成
- 解析延迟 <10ms（P50）
- 支持动态规则热更新
- 缩写词还原准确率 >85%

---

## 1. 架构设计

### 1.1 总体架构

```
┌─────────────────────────────────────────────────────────┐
│                   用户输入: "明早8点"                    │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│  Layer 1: 预处理与扩展                                   │
│  ├─ 模板正则化: "明(天|日)早" → "明天早上"               │
│  ├─ fastText 扩展: "明早" → ["明天早上", "明天早晨"]     │
│  │  (可选，默认禁用)                                     │
│  └─ 编辑距离纠错: "tomorow" → "tomorrow"                 │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│  Layer 2: 规则匹配引擎                                   │
│  ├─ 动态规则（Apollo）: 业务术语优先                     │
│  │  (可选，默认禁用)                                     │
│  └─ 静态规则（Rust）: 核心时间/数字解析                  │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│  Layer 3: 结果排序                                       │
│  └─ 朴素贝叶斯: 多候选结果排序                           │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│  性能监测: 记录各模块耗时                                │
└─────────────────────────────────────────────────────────┘
                         ↓
                输出: DateTime(2026-02-14 08:00)
```

### 1.2 核心组件

#### SmartMatcher（智能匹配器）
- **职责**: 输入预处理与扩展
- **模块**:
  - PatternNormalizer: 模板正则化
  - LevenshteinMatcher: 编辑距离纠错
  - FastTextExpander: 词向量相似度扩展（可选）

#### HybridParser（混合解析器）
- **职责**: 协调规则匹配与结果排序
- **模块**:
  - StaticRuleEngine: Rust 硬编码规则
  - DynamicRuleEngine: JSON 动态规则
  - MLRanker: 朴素贝叶斯排序

#### ApolloClient（配置中心客户端）
- **职责**: 配置热更新（可选）
- **功能**:
  - 长轮询监听配置变更
  - 自动重载规则
  - 降级策略（连接失败时）

#### PerformanceMetrics（性能监测）
- **职责**: 记录各模块耗时
- **指标**:
  - 总请求数
  - 各模块平均耗时
  - 占比分析

---

## 2. 模块详细设计

### 2.1 模糊匹配设计

#### 三层匹配策略

```rust
pub struct SmartMatcher {
    // Layer 1: 模板正则化（必需）
    pattern_normalizer: PatternNormalizer,

    // Layer 2: 编辑距离（必需）
    levenshtein: FuzzyMatcher,

    // Layer 3: fastText 扩展（可选）
    fasttext_expander: Option<Arc<FastTextExpander>>,

    // 配置
    config: MatcherConfig,
}

pub struct MatcherConfig {
    pub enable_pattern_norm: bool,     // 默认 true
    pub enable_levenshtein: bool,      // 默认 true
    pub enable_fasttext: bool,         // 默认 false
    pub levenshtein_threshold: f32,    // 默认 0.85
    pub fasttext_threshold: f32,       // 默认 0.85
}
```

#### 模板正则化

**目的**: 减少同义词配置量

```rust
// 5-10 个高频模式
vec![
    ("明(天|日)(早|晨)", "明天早上"),
    ("明(天|日)(晚|夜)", "明天晚上"),
    ("今(天|日)(早|晨)", "今天早上"),
]
```

#### fastText 词向量扩展

**目的**: 智能识别同义词和缩写

```rust
// 预计算候选词向量（200-500 个常用时间短语）
let candidates = vec![
    "明天早上", "明天早晨", "明天晚上",
    "今天早上", "今天晚上",
    // ...
];

// 运行时相似度搜索
// "明早" vs "明天早上" → similarity = 0.92 ✅
// "明早" vs "今天早上" → similarity = 0.65 ❌
```

**模型选择**: fastText（Facebook）
- 模型大小: 5-50MB（压缩后）
- 推理延迟: <5ms
- Rust 支持: finalfusion crate

### 2.2 动态规则引擎

#### JSON Schema

```json
{
  "version": "1.0.0",
  "dynamic_rules": [
    {
      "id": "morning_rush",
      "priority": 100,
      "enabled": true,
      "pattern": {
        "type": "exact",
        "value": "早高峰"
      },
      "output": {
        "type": "time_range",
        "start_hour": 7,
        "end_hour": 9
      }
    },
    {
      "id": "at_oclock",
      "priority": 90,
      "pattern": {
        "type": "template",
        "value": "at {number} o'clock",
        "constraints": {
          "number": {"min": 1, "max": 24}
        }
      },
      "output": {
        "type": "time",
        "hour": "{number}",
        "minute": 0
      }
    }
  ]
}
```

#### 模式类型

| 类型 | 说明 | 示例 |
|------|------|------|
| exact | 精确匹配 | "早高峰" |
| regex | 正则表达式 | r"at (\d+) o'clock" |
| template | 模板占位符 | "at {number} o'clock" |

#### 优先级机制

- 动态规则优先级 > 静态规则
- 同类规则按 priority 字段排序
- 匹配后立即返回（不继续匹配）

### 2.3 Apollo 集成

#### 可选启用

```rust
// 场景 1: 不使用 Apollo（默认）
let parser = HybridParser::builder()
    .build()?;

// 场景 2: 启用 Apollo
let parser = HybridParser::builder()
    .with_apollo(ApolloConfig {
        server: "http://apollo:8080",
        app_id: "duckling",
        namespace: "application",
    })
    .build()?;
```

#### 热更新流程

```
Apollo 配置更新
    ↓
长轮询检测变更
    ↓
下载新配置（JSON）
    ↓
解析并验证
    ↓
原子替换 Arc<RwLock<DynamicRules>>
    ↓
新请求使用新规则（<100ms）
```

#### 降级策略

- 连接失败: 使用缓存配置
- 解析失败: 保留旧配置，记录错误
- 验证失败: 拒绝更新，告警

### 2.4 性能监测

#### 监测指标

```rust
pub struct PerformanceMetrics {
    total_requests: AtomicU64,

    // 各模块累计耗时（纳秒）
    pattern_norm_ns: AtomicU64,
    levenshtein_ns: AtomicU64,
    fasttext_ns: AtomicU64,
    dynamic_rules_ns: AtomicU64,
    static_rules_ns: AtomicU64,
    ml_ranking_ns: AtomicU64,
    total_parse_ns: AtomicU64,
}
```

#### 报告格式

```
Performance Metrics Report
==========================
Total Requests: 1000
Average Total:  8.5 ms

Module Breakdown:
  Pattern Norm:   0.5 ms (5.9%)
  Levenshtein:    0.8 ms (9.4%)
  FastText:       2.1 ms (24.7%)
  Dynamic Rules:  0.3 ms (3.5%)
  Static Rules:   3.2 ms (37.6%)
  ML Ranking:     1.6 ms (18.8%)
```

#### 使用方式

```rust
// 运行测试
for input in test_inputs {
    parser.parse(input)?;
}

// 查看报告
println!("{}", parser.metrics_report());

// 重置统计（用于新一轮基准测试）
parser.metrics_reset();
```

---

## 3. 数据结构

### 3.1 Value 枚举

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Duration(DurationValue),
    Time(TimeValue),
    DateTime(DateTimeValue),
    TimeRange(TimeRangeValue),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DurationValue {
    pub amount: i64,
    pub unit: TimeUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}
```

### 3.2 动态规则

```rust
pub struct DynamicRule {
    pub id: String,
    pub priority: u32,
    pub enabled: bool,
    pub pattern: DynamicPattern,
    pub output: OutputTemplate,
}

pub enum DynamicPattern {
    Exact(String),
    Regex(Regex),
    Template {
        template: String,
        placeholders: Vec<Placeholder>,
    },
}

pub struct Placeholder {
    pub name: String,           // "number"
    pub value_type: ValueType,  // Integer
    pub constraint: Option<Constraint>,
}
```

---

## 4. API 设计

### 4.1 初始化

```rust
use duckling::{HybridParser, ApolloConfig};

// 最小化启动
let parser = HybridParser::builder()
    .build()?;

// 完整配置
let parser = HybridParser::builder()
    .with_apollo(ApolloConfig {
        server: "http://apollo:8080",
        app_id: "duckling",
        namespace: "application",
    })
    .with_fasttext("/models/time_zh.bin")
    .build()?;
```

### 4.2 解析

```rust
// 单次解析
let results = parser.parse("明早8点")?;
for result in results {
    println!("{:?}", result.value);
    // DateTime { date: 2026-02-14, time: 08:00:00 }
}

// 批量解析
let inputs = vec!["明早8点", "in 5 minutes", "后天下午"];
for input in inputs {
    let results = parser.parse(input)?;
    // ...
}
```

### 4.3 配置更新

```rust
// 手动更新动态规则
parser.reload_dynamic_rules(new_json)?;

// Apollo 自动更新（后台运行）
// 无需手动调用
```

### 4.4 性能监测

```rust
// 获取报告
let report = parser.metrics_report();
println!("{}", report);

// 导出 JSON
let json = serde_json::to_string(&report)?;

// 重置统计
parser.metrics_reset();
```

---

## 5. 实施计划

### Week 1: 基础架构（Feb 13-19）

**目标**: 核心解析流程跑通

- [ ] Day 1-2: 项目结构搭建
  - 创建模块目录
  - 定义 Value 枚举
  - 定义核心 trait

- [ ] Day 3-4: 静态规则实现
  - `src/rules/integer.rs`
  - `src/rules/duration.rs`
  - `src/rules/time.rs`（"in {duration}"）
  - 测试用例

- [ ] Day 5-6: 基础模糊匹配
  - PatternNormalizer（5-10 个模式）
  - LevenshteinMatcher
  - SmartMatcher 集成

### Week 2: 动态规则 + Apollo（Feb 20-26）

**目标**: 动态规则可用，Apollo 可选集成

- [ ] Day 1-2: 动态规则引擎
  - DynamicRule 数据结构
  - JSON schema 解析
  - 模板匹配器（{number}, {time}）
  - 优先级排序

- [ ] Day 3-4: Apollo 集成
  - ApolloClient 实现
  - 长轮询配置监听
  - 热更新机制
  - 可选启用设计

- [ ] Day 5-6: HybridParser 集成
  - 动态 + 静态规则协同
  - 完整解析流程
  - 端到端测试

### Week 3: fastText + 性能优化（Feb 27 - Mar 5）

**目标**: 智能化提升，性能监测

- [ ] Day 1-2: fastText 集成
  - 添加 finalfusion 依赖
  - FastTextExpander 实现
  - 下载/压缩中文模型
  - 相似度搜索优化

- [ ] Day 3-4: 性能监测
  - PerformanceMetrics 实现
  - 各模块计时
  - 报告生成
  - 基准测试

- [ ] Day 5-6: 文档与示例
  - API 文档
  - Apollo 配置示例
  - 自定义规则示例
  - 性能调优指南

---

## 6. 性能目标

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 解析延迟（P50） | <10ms | 基准测试 |
| 解析延迟（P99） | <50ms | 基准测试 |
| 内存占用（基础） | <100MB | 不含 fastText |
| 内存占用（完整） | <150MB | 含 fastText |
| 启动时间（基础） | <100ms | 不含 Apollo/fastText |
| 启动时间（完整） | <500ms | 含 Apollo/fastText |
| Apollo 热更新 | <100ms | 配置切换延迟 |
| 模块耗时占比 | 可见 | PerformanceMetrics |

---

## 7. 验收标准

### 7.1 基础功能

- [ ] 支持 5+ 种 Value 类型
- [ ] 静态规则: 10+ 个核心时间规则
- [ ] 动态规则: 支持 exact, regex, template
- [ ] 模糊匹配: 编辑距离 + 模板正则化
- [ ] Apollo 可选集成
- [ ] fastText 可选集成
- [ ] 性能监测完整

### 7.2 性能要求

- [ ] 解析延迟 P50 <10ms
- [ ] 解析延迟 P99 <50ms
- [ ] 内存占用 <150MB
- [ ] Apollo 热更新 <100ms
- [ ] 各模块耗时可见

### 7.3 工程质量

- [ ] 单元测试覆盖率 >80%
- [ ] 集成测试: 10+ 真实场景
- [ ] 文档完整（API + 配置）
- [ ] 示例代码可运行
- [ ] 无 unsafe 代码（除必需）

---

## 8. 风险与缓解

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|---------|
| fastText 模型过大 | 启动慢 | 中 | 压缩模型，可选启用 |
| Apollo 连接失败 | 无法热更新 | 低 | 降级策略，可选启用 |
| 性能不达标 | 用户体验差 | 中 | 性能监测，持续优化 |
| 模糊匹配误匹配 | 解析错误 | 中 | 阈值可配置，NB 排序 |

---

## 9. 后续计划

Phase 1 完成后，Phase 2 将实现：

1. **HTTP 服务** - Actix-web REST API
2. **配置中心完善** - Apollo 高级特性
3. **监控告警** - Prometheus metrics
4. **更多维度** - Numeral/Time 规则移植

---

## 10. 参考文档

- [Phase 0 评估报告](../PHASE0_评估报告.md)
- [架构文档](../ARCHITECTURE.md)
- [迁移指南](../MIGRATION_GUIDE.md)
- [讨论记录](./2026-02-13-phase1-discussion.md)

---

**文档版本**: 1.0
**最后更新**: 2026-02-13
**下次评审**: Phase 1 Week 1 结束

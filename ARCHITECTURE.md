# Rustling 架构设计文档

**版本**: 1.0
**日期**: 2026-02-12
**阶段**: Phase 0 Week 2
**作者**: Claude Code

---

## 目录

1. [概述](#概述)
2. [架构原则](#架构原则)
3. [模块结构](#模块结构)
4. [核心组件](#核心组件)
5. [数据流](#数据流)
6. [类型系统](#类型系统)
7. [扩展机制](#扩展机制)
8. [性能考虑](#性能考虑)

---

## 概述

Rustling 是 Duckling（Haskell 自然语言解析库）的 Rust 移植版本。它采用**三层架构**设计，提供灵活且高性能的文本解析能力。

### 设计目标

- **类型安全**: 利用 Rust 的强类型系统防止运行时错误
- **高性能**: 零成本抽象和内存高效的数据结构
- **可扩展**: 易于添加新规则和模式
- **可测试**: 清晰的模块边界和依赖注入

### 核心能力

- 模式匹配（正则表达式、过滤器）
- 规则组合（最多 6 个输入）
- 饱和解析（exhaustive parsing）
- ML 排序（可选）

---

## 架构原则

### 1. 分层架构

```
┌─────────────────────────────────┐
│   High-Level API (rustling)    │  ← 用户接口层
├─────────────────────────────────┤
│   ML Module (rustling-ml)      │  ← 机器学习层
├─────────────────────────────────┤
│   Core Engine (rustling-core)  │  ← 解析引擎核心
└─────────────────────────────────┘
```

### 2. 依赖方向

- **核心无依赖**: `rustling-core` 不依赖 ML 模块
- **单向依赖**: 上层可依赖下层，反之则不行
- **可插拔**: ML 模块是可选的

### 3. 零成本抽象

- 泛型在编译时单态化
- 内联小函数
- 零运行时反射

---

## 模块结构

### 整体架构图

```mermaid
graph TB
    User[用户代码]

    subgraph rustling[rustling 主 crate]
        API[High-Level API]
        Macros[宏系统]
        Train[训练工具]
    end

    subgraph core[rustling-core]
        Builder[RuleSetBuilder]
        RuleSet[RuleSet]
        Rules[规则引擎]
        Patterns[模式匹配]
        Stash[值存储]
        Error[错误处理]
    end

    subgraph ml[rustling-ml]
        Classifier[分类器]
        Model[模型]
        Training[训练逻辑]
    end

    User --> API
    API --> Macros
    API --> Train
    API --> Builder
    Train --> Classifier
    Builder --> RuleSet
    RuleSet --> Rules
    Rules --> Patterns
    Rules --> Stash
    Patterns --> Stash
    Train --> Training
    Training --> Model
    Classifier --> Model
```

### 模块职责

#### `rustling-core` (核心引擎)

| 文件 | 职责 | 关键类型 |
|------|------|---------|
| `lib.rs` | 公共 API、类型定义 | `RuleSet`, `ParsedNode` |
| `builder.rs` | 规则集构建器 | `RuleSetBuilder` |
| `rule.rs` | 规则定义与执行 | `Rule1..6`, `TerminalRule` |
| `pattern.rs` | 模式匹配 | `TextPattern`, `FilterNodePattern` |
| `stash.rs` | 值存储与索引 | `Stash`, `StashIndexable` |
| `error.rs` | 错误类型 | `RustlingError` |
| `helpers.rs` | 边界检查 | `BoundariesChecker` |
| `range.rs` | 范围类型 | `Range` |

#### `rustling-ml` (机器学习)

| 文件 | 职责 | 关键类型 |
|------|------|---------|
| `lib.rs` | ML 公共 API | `Classifier`, `Model` |

#### `rustling` (高层 API)

| 文件 | 职责 | 关键类型 |
|------|------|---------|
| `lib.rs` | 重新导出、高层封装 | `Parser`, `ParserMatch` |
| `macros.rs` | 辅助宏 | `build_rules!`, `dim!` |
| `train.rs` | 训练工具 | `Example`, `Check` |

---

## 核心组件

### 1. 解析引擎 (Core Engine)

#### RuleSetBuilder

**作用**: 构建规则集的流畅 API

```rust
let builder = RuleSetBuilder::new(
    BoundariesChecker::detailed(),
    BoundariesChecker::separated_alphanumeric_word(),
);

// 终结规则：匹配文本
builder.rule_1("integer",
    builder.reg(r"\d+").unwrap(),
    |text| Ok(Int(text.parse()?))
);

// 组合规则：组合两个值
builder.rule_2("add",
    dim!(Int),
    dim!(Int),
    |a, b| Ok(Int(a.value() + b.value()))
);

let rule_set = builder.build();
```

**设计模式**: Builder + 类型状态机

#### RuleSet

**作用**: 执行规则应用的核心引擎

```rust
pub struct RuleSet<S> {
    composition_rules: Vec<Box<dyn Rule<S>>>,
    terminal_rules: Vec<Box<dyn TerminalRule<S>>>,
    // ...
}
```

**关键方法**:
- `apply_all(text: &str)`: 饱和解析，返回所有匹配

**算法**:
1. 应用终结规则生成初始 tokens
2. 迭代应用组合规则直到饱和
3. 返回所有匹配的 `ParsedNode`

#### Pattern Trait

**作用**: 定义匹配逻辑的抽象

```rust
pub trait Pattern<StashValue> {
    type M;
    fn accepts(&self, node: &Node<StashValue>) -> bool;
    fn produce(&self, nodes: &[Node<StashValue>])
        -> Vec<Match<Self::M>>;
}
```

**实现类型**:
- `TextPattern`: 正则表达式匹配
- `FilterNodePattern`: 值过滤器
- `AnyNodePattern`: 匹配任意节点

#### Stash

**作用**: 高效存储和索引解析值

```rust
pub struct Stash<S: StashIndexable> {
    arenas: FnvHashMap<S::Index, Vec<S>>,
}
```

**优化**: 按类型索引，避免遍历所有值

---

### 2. ML 模块 (ML Module)

#### Classifier

**作用**: 对解析结果进行排序/分类

```rust
pub struct Classifier<Id, Class, Feat> {
    models: FnvHashMap<Id, Model<Class, Feat>>,
}
```

**工作流程**:
1. 提取特征 (`Feature`)
2. 使用朴素贝叶斯模型预测
3. 返回最可能的类别

#### Model

**作用**: 存储训练好的朴素贝叶斯模型

```rust
pub struct Model<Class, Feat> {
    classes: Vec<Class>,
    // P(class)
    prior: FnvHashMap<Class, f32>,
    // P(feature|class)
    likelihood: FnvHashMap<(Feat, Class), f32>,
}
```

---

### 3. 高层 API (High-Level API)

#### Parser

**作用**: 面向用户的便捷接口

```rust
pub struct Parser<V> {
    rule_set: RuleSet<V>,
    classifier: Option<Classifier<...>>,
}

impl<V> Parser<V> {
    pub fn parse(&self, text: &str) -> Vec<ParserMatch<V>>;
}
```

**特性**:
- 自动 ML 排序（如果提供分类器）
- 转换为用户友好的 `ParserMatch`

#### 辅助宏

**`dim!` 宏**: 简化值过滤器创建

```rust
// 匹配所有 Int 值
dim!(Int)

// 匹配满足条件的 Int 值
dim!(Int, vec![Box::new(|a: &Int| a.0 > 0)])
```

**`build_rules!` 宏**: 生成枚举和转换（可选）

---

## 数据流

### 解析流程

```mermaid
flowchart TD
    Start[文本输入] --> Terminal[终结规则匹配]
    Terminal --> Stash1[存入 Stash]
    Stash1 --> Loop{饱和?}

    Loop -->|否| Composition[组合规则匹配]
    Composition --> Combine[组合 tokens]
    Combine --> Stash2[存入 Stash]
    Stash2 --> Loop

    Loop -->|是| Extract[提取 ParsedNode]
    Extract --> ML{有分类器?}

    ML -->|是| Features[提取特征]
    Features --> Classify[ML 排序]
    Classify --> Return[返回结果]

    ML -->|否| Return
```

### 详细步骤

#### 1. 初始化

```rust
let text = "I have 12 thousands";
let stash = Stash::new();
```

#### 2. 终结规则应用

```
文本: "I have 12 thousands"

规则: r"\d+" → Int(12)
规则: "thousands?" → Int(1000)

Stash: [Int(12), Int(1000)]
```

#### 3. 组合规则应用（第 1 轮）

```
模式: Int (>1, <99) + Int (==1000)
生产: Int(12 * 1000) = Int(12000)

Stash: [Int(12), Int(1000), Int(12000)]
```

#### 4. 饱和检测

```
第 2 轮: 无新值生成 → 饱和
```

#### 5. 结果返回

```rust
vec![
    ParserMatch { value: Int(12), byte_range: 7..9, ... },
    ParserMatch { value: Int(1000), byte_range: 10..19, ... },
    ParserMatch { value: Int(12000), byte_range: 7..19, ... },
]
```

---

## 类型系统

### 核心 Trait

#### NodePayload

**作用**: 定义可解析的值类型

```rust
pub trait NodePayload: Clone {
    type Payload: Clone + PartialEq + Debug;
    fn extract_payload(&self) -> Option<Self::Payload>;
}
```

**示例**:
```rust
impl NodePayload for Int {
    type Payload = MyPayload;
    fn extract_payload(&self) -> Option<MyPayload> {
        Some(MyPayload)
    }
}
```

#### StashIndexable

**作用**: 允许值按类型索引

```rust
pub trait StashIndexable {
    type Index: Hash + Eq;
    fn index(&self) -> Self::Index;
}
```

**优势**: O(1) 类型查找而非 O(n) 遍历

#### AttemptFrom

**作用**: 可失败的类型转换

```rust
pub trait AttemptFrom<V>: Sized {
    fn attempt_from(v: V) -> Option<Self>;
}
```

**用途**: 从枚举变体提取具体类型

---

### 幽灵类型 (Phantom Types)

#### SendSyncPhantomData

**作用**: 标记类型参数，确保线程安全

```rust
pub struct SendSyncPhantomData<T>(PhantomData<T>);
unsafe impl<T> Send for SendSyncPhantomData<T> {}
unsafe impl<T> Sync for SendSyncPhantomData<T> {}
```

**使用场景**: Rule1..6 和 Pattern 结构体

**安全性**: 已审计，详见 `UNSAFE_CODE_AUDIT.md`

---

## 扩展机制

### 添加新规则

#### 1. 定义值类型

```rust
#[derive(Clone, Debug)]
pub struct Temperature {
    value: f32,
    unit: Unit,
}
```

#### 2. 实现必需 Trait

```rust
impl NodePayload for Temperature { /* ... */ }
impl StashIndexable for Temperature { /* ... */ }
impl InnerStashIndexable for Temperature { /* ... */ }
```

#### 3. 创建规则

```rust
builder.rule_1("celsius",
    builder.reg(r"(-?\d+\.?\d*)\s*°?C").unwrap(),
    |m| Ok(Temperature {
        value: m.group(1).parse()?,
        unit: Unit::Celsius
    })
);
```

### 添加新模式

实现 `Pattern` trait:

```rust
struct CustomPattern;

impl<S> Pattern<S> for CustomPattern {
    type M = MyMatch;

    fn accepts(&self, node: &Node<S>) -> bool {
        // 自定义逻辑
    }

    fn produce(&self, nodes: &[Node<S>])
        -> Vec<Match<Self::M>> {
        // 自定义生产
    }
}
```

---

## 性能考虑

### 优化策略

#### 1. 零成本抽象

- **泛型单态化**: 编译时展开，无虚拟调用
- **内联**: 小函数自动内联
- **Trait 对象**: 仅在需要动态分发时使用

#### 2. 内存效率

- **SmallVec**: 小数组栈分配
- **String Interning**: 字符串去重
- **Arena 分配**: Stash 按类型分组

#### 3. 算法优化

- **早期退出**: 无匹配时快速返回（~245 ns）
- **索引查找**: O(1) 类型索引
- **正则缓存**: 编译一次，重复使用

### 性能基准

参见 `BENCHMARKS.md`:

| 操作 | 时间 |
|------|------|
| 简单匹配 | ~900 ns |
| 组合规则 | ~2.4 µs |
| 长文本解析 | ~8.1 µs |

### 内存使用

- **Stash**: O(n) 其中 n = 解析值数量
- **RuleSet**: O(r) 其中 r = 规则数量
- **String Interner**: O(u) 其中 u = 唯一字符串数

---

## 未来扩展

### Phase 1 计划

1. **JSON 规则加载器**
   - 从 JSON 反序列化规则
   - 支持动态规则更新

2. **模糊匹配**
   - Levenshtein 距离
   - 音似匹配（Soundex）

3. **规则热重载**
   - 无需重启更新规则
   - 版本管理

### Phase 2+ 愿景

- **并行解析**: Rayon 并行化
- **增量解析**: 只重新解析变更部分
- **WASM 支持**: 编译到 WebAssembly
- **规则可视化**: 调试工具

---

## 设计决策

### 为什么用 Rust？

1. **类型安全**: 编译时捕获错误
2. **性能**: 接近 C 的速度
3. **内存安全**: 无 GC 开销
4. **并发**: Fearless concurrency

### 为什么三层架构？

1. **关注点分离**: 核心 / ML / API 各司其职
2. **可测试性**: 每层独立测试
3. **可替换性**: ML 模块可选
4. **清晰依赖**: 防止循环依赖

### 为什么不用宏？

- **可读性**: 宏会降低代码可读性
- **IDE 支持**: 类型提示更好
- **编译时间**: 减少宏展开开销

仅在明确收益时使用宏（如 `dim!`）。

---

## 参考资料

### 外部文档

- [Haskell Duckling](https://github.com/facebook/duckling)
- [Rust 设计模式](https://rust-unofficial.github.io/patterns/)
- [Zero-cost abstractions](https://blog.rust-lang.org/2015/05/11/traits.html)

### 内部文档

- `PHASE0_评估报告.md` - 技术决策
- `UNSAFE_CODE_AUDIT.md` - 安全审计
- `BENCHMARKS.md` - 性能基准
- `MIGRATION_GUIDE.md` - Haskell → Rust 迁移

---

## 附录

### 术语表

| 术语 | 定义 |
|------|------|
| **终结规则** | 直接从文本匹配的规则（Terminal Rule） |
| **组合规则** | 组合已解析值的规则（Composition Rule） |
| **饱和** | 无法生成新值的状态（Saturation） |
| **Stash** | 存储解析值的数据结构 |
| **Pattern** | 匹配逻辑的抽象 |
| **Node** | 解析树节点 |

### 缩写

- **API**: Application Programming Interface
- **ML**: Machine Learning
- **AST**: Abstract Syntax Tree (本项目称为 Parse Tree)
- **DSL**: Domain Specific Language

---

**文档版本**: 1.0
**创建日期**: 2026-02-12
**最后更新**: 2026-02-12
**状态**: ✅ 完成

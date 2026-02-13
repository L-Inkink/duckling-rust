# Phase 1: 核心功能增强 - 实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 实现 JSON 规则加载、智能模糊匹配和 Apollo 热重载，为 Duckling Rust 提供生产级的动态配置能力

**Architecture:** 混合架构 - 静态规则（Rust 硬编码）提供核心解析能力，动态规则（Apollo JSON 配置）提供业务灵活性，智能模糊匹配（模板正则 + 编辑距离 + 可选 fastText）处理输入容错，朴素贝叶斯模型进行结果排序

**Tech Stack:**
- Core: Rust 2021, rustling-core
- Apollo: HTTP long-polling, serde_json
- Fuzzy: regex, finalfusion (fastText), levenshtein
- Metrics: AtomicU64, std::time::Instant

---

## Week 1: 基础架构（Feb 13-19）

### Task 1: 项目结构搭建

**Files:**
- Create: `src/values/mod.rs`
- Create: `src/values/duration.rs`
- Create: `src/values/time.rs`
- Create: `src/rules/mod.rs`
- Create: `src/fuzzy/mod.rs`
- Create: `src/metrics.rs`

**Step 1: 创建 values 模块目录结构**

```bash
mkdir -p src/values src/rules src/fuzzy
touch src/values/mod.rs src/values/duration.rs src/values/time.rs
touch src/rules/mod.rs
touch src/fuzzy/mod.rs
touch src/metrics.rs
```

**Step 2: 在 src/lib.rs 中声明新模块**

Modify: `src/lib.rs` - 添加模块声明

```rust
// 在文件开头添加
pub mod values;
pub mod rules;
pub mod fuzzy;
pub mod metrics;
```

**Step 3: 验证编译**

Run: `cargo build`
Expected: 编译成功（模块为空但有效）

**Step 4: Commit**

```bash
git add src/values/ src/rules/ src/fuzzy/ src/metrics.rs src/lib.rs
git commit -m "feat: add module structure for Phase 1

- src/values/: Value type definitions
- src/rules/: Static rule modules
- src/fuzzy/: Fuzzy matching components
- src/metrics.rs: Performance monitoring"
```

---

### Task 2: 定义 Value 枚举和 Duration 类型

**Files:**
- Modify: `src/values/mod.rs`
- Modify: `src/values/duration.rs`
- Create: `tests/values_test.rs`

**Step 1: 编写 Duration 测试**

Create: `tests/values_test.rs`

```rust
use duckling_rust::values::{DurationValue, TimeUnit};
use chrono::Duration;

#[test]
fn test_duration_to_chrono_minutes() {
    let dur = DurationValue {
        amount: 5,
        unit: TimeUnit::Minute,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::minutes(5));
}

#[test]
fn test_duration_to_chrono_hours() {
    let dur = DurationValue {
        amount: 2,
        unit: TimeUnit::Hour,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::hours(2));
}

#[test]
fn test_duration_to_chrono_days() {
    let dur = DurationValue {
        amount: 3,
        unit: TimeUnit::Day,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::days(3));
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_duration`
Expected: 编译错误 "cannot find type `DurationValue`"

**Step 3: 实现 Duration 类型**

Modify: `src/values/duration.rs`

```rust
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurationValue {
    pub amount: i64,
    pub unit: TimeUnit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl DurationValue {
    pub fn to_chrono_duration(&self) -> Duration {
        match self.unit {
            TimeUnit::Second => Duration::seconds(self.amount),
            TimeUnit::Minute => Duration::minutes(self.amount),
            TimeUnit::Hour => Duration::hours(self.amount),
            TimeUnit::Day => Duration::days(self.amount),
            TimeUnit::Week => Duration::weeks(self.amount),
        }
    }
}
```

**Step 4: 导出 Duration 类型**

Modify: `src/values/mod.rs`

```rust
pub mod duration;

pub use duration::{DurationValue, TimeUnit};
```

**Step 5: 添加 chrono 依赖**

Modify: `Cargo.toml` - 在 [dependencies] 下添加

```toml
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

**Step 6: 运行测试验证通过**

Run: `cargo test test_duration`
Expected: 所有测试通过

**Step 7: Commit**

```bash
git add src/values/duration.rs src/values/mod.rs tests/values_test.rs Cargo.toml
git commit -m "feat: add DurationValue type with chrono conversion

- DurationValue struct with amount and TimeUnit
- Support Second, Minute, Hour, Day, Week
- to_chrono_duration() conversion method
- Tests for all time units"
```

---

### Task 3: 定义 Time 和 Value 枚举

**Files:**
- Modify: `src/values/time.rs`
- Modify: `src/values/mod.rs`
- Modify: `tests/values_test.rs`

**Step 1: 编写 Time 测试**

Modify: `tests/values_test.rs` - 添加

```rust
use duckling_rust::values::{TimeValue, Value};
use chrono::{Utc, TimeZone};

#[test]
fn test_time_value_creation() {
    let timestamp = Utc.with_ymd_and_hms(2026, 2, 14, 8, 0, 0).unwrap();
    let time = TimeValue { timestamp };

    assert_eq!(time.timestamp.hour(), 8);
    assert_eq!(time.timestamp.minute(), 0);
}

#[test]
fn test_value_enum_duration() {
    let dur = DurationValue {
        amount: 5,
        unit: TimeUnit::Minute,
    };
    let value = Value::Duration(dur.clone());

    if let Value::Duration(d) = value {
        assert_eq!(d.amount, 5);
    } else {
        panic!("Expected Duration variant");
    }
}

#[test]
fn test_value_enum_time() {
    let timestamp = Utc::now();
    let time = TimeValue { timestamp };
    let value = Value::Time(time.clone());

    if let Value::Time(t) = value {
        assert_eq!(t.timestamp, timestamp);
    } else {
        panic!("Expected Time variant");
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_time`
Expected: 编译错误 "cannot find type `TimeValue`"

**Step 3: 实现 Time 类型**

Create: `src/values/time.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeValue {
    pub timestamp: DateTime<Utc>,
}
```

**Step 4: 实现 Value 枚举**

Modify: `src/values/mod.rs`

```rust
pub mod duration;
pub mod time;

use duration::{DurationValue, TimeUnit};
use time::TimeValue;
use serde::{Deserialize, Serialize};

pub use duration::*;
pub use time::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Duration(DurationValue),
    Time(TimeValue),
}
```

**Step 5: 运行测试验证通过**

Run: `cargo test test_time test_value`
Expected: 所有测试通过

**Step 6: Commit**

```bash
git add src/values/time.rs src/values/mod.rs tests/values_test.rs
git commit -m "feat: add TimeValue and Value enum

- TimeValue with UTC timestamp
- Value enum with Integer, Duration, Time variants
- Tests for Time creation and enum variants"
```

---

### Task 4: 实现 NodePayload trait for Value

**Files:**
- Modify: `src/values/mod.rs`
- Create: `tests/value_payload_test.rs`

**Step 1: 编写 NodePayload 测试**

Create: `tests/value_payload_test.rs`

```rust
use duckling_rust::values::Value;
use rustling_core::{NodePayload, StashIndexable};

#[test]
fn test_value_implements_node_payload() {
    let value = Value::Integer(42);

    // 测试 payload() 方法
    assert_eq!(value.payload(), Some(&value));
}

#[test]
fn test_value_implements_stash_indexable() {
    let value = Value::Integer(42);

    // 测试 value() 方法
    assert_eq!(value.value(), &value);
}

#[test]
fn test_value_clone() {
    let value1 = Value::Integer(42);
    let value2 = value1.clone();

    assert_eq!(value1, value2);
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_value_implements`
Expected: 编译错误 "trait bounds are not satisfied"

**Step 3: 实现 NodePayload 和 StashIndexable**

Modify: `src/values/mod.rs` - 添加实现

```rust
use rustling_core::{NodePayload, StashIndexable};

impl NodePayload for Value {
    type Payload = Value;

    fn payload(&self) -> Option<&Self::Payload> {
        Some(self)
    }
}

impl StashIndexable for Value {
    type Payload = Value;

    fn value(&self) -> &Self::Payload {
        self
    }
}
```

**Step 4: 运行测试验证通过**

Run: `cargo test test_value_implements`
Expected: 所有测试通过

**Step 5: Commit**

```bash
git add src/values/mod.rs tests/value_payload_test.rs
git commit -m "feat: implement NodePayload and StashIndexable for Value

- Integrate Value with rustling-core traits
- Tests for trait implementations"
```

---

### Task 5: 实现第一个静态规则 - Integer

**Files:**
- Create: `src/rules/integer.rs`
- Modify: `src/rules/mod.rs`
- Create: `tests/rules_integer_test.rs`

**Step 1: 编写 Integer 规则测试**

Create: `tests/rules_integer_test.rs`

```rust
use duckling_rust::rules;
use duckling_rust::values::Value;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_integer_rule_single_digit() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("5").unwrap();
    assert_eq!(results.len(), 1);

    if let Value::Integer(n) = results[0].value {
        assert_eq!(n, 5);
    } else {
        panic!("Expected Integer value");
    }
}

#[test]
fn test_integer_rule_multi_digit() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("123").unwrap();
    assert_eq!(results.len(), 1);

    if let Value::Integer(n) = results[0].value {
        assert_eq!(n, 123);
    } else {
        panic!("Expected Integer value");
    }
}

#[test]
fn test_integer_rule_in_sentence() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("I have 42 apples").unwrap();
    assert!(results.len() >= 1);

    // 查找 Integer 结果
    let int_result = results.iter().find(|r| {
        matches!(r.value, Value::Integer(_))
    }).expect("Should find Integer");

    if let Value::Integer(n) = int_result.value {
        assert_eq!(n, 42);
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_integer_rule`
Expected: 编译错误 "cannot find module `integer`"

**Step 3: 实现 Integer 规则**

Create: `src/rules/integer.rs`

```rust
use crate::values::Value;
use rustling_core::{RuleSetBuilder, RuleResult};

pub fn rules(b: &RuleSetBuilder<Value>) {
    // 规则: 匹配 1-18 位数字
    b.rule_1_terminal(
        "integer (numeric)",
        b.reg(r"(\d{1,18})").unwrap(),
        |text_match| {
            let num: i64 = text_match.group(0).parse()
                .map_err(|e| format!("Failed to parse integer: {}", e))?;
            Ok(Value::Integer(num))
        }
    );
}
```

**Step 4: 导出 Integer 规则模块**

Modify: `src/rules/mod.rs`

```rust
pub mod integer;

use crate::values::Value;
use rustling_core::RuleSetBuilder;

pub fn register_all_rules(b: &RuleSetBuilder<Value>) {
    integer::rules(b);
}
```

**Step 5: 运行测试验证通过**

Run: `cargo test test_integer_rule`
Expected: 所有测试通过

**Step 6: Commit**

```bash
git add src/rules/integer.rs src/rules/mod.rs tests/rules_integer_test.rs
git commit -m "feat: add integer parsing rule

- Parse 1-18 digit integers
- Register in rules module
- Tests for single digit, multi-digit, and in-sentence parsing"
```

---

### Task 6: 实现 Duration 规则

**Files:**
- Create: `src/rules/duration.rs`
- Modify: `src/rules/mod.rs`
- Create: `tests/rules_duration_test.rs`

**Step 1: 编写 Duration 规则测试**

Create: `tests/rules_duration_test.rs`

```rust
use duckling_rust::rules;
use duckling_rust::values::{Value, DurationValue, TimeUnit};
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_duration_minutes() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("5 minutes").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 5);
        assert_eq!(d.unit, TimeUnit::Minute);
    }
}

#[test]
fn test_duration_hours() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("2 hours").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 2);
        assert_eq!(d.unit, TimeUnit::Hour);
    }
}

#[test]
fn test_duration_singular() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("1 minute").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 1);
        assert_eq!(d.unit, TimeUnit::Minute);
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_duration`
Expected: 测试失败 "Should find Duration"

**Step 3: 实现 Duration 规则**

Create: `src/rules/duration.rs`

```rust
use crate::values::{Value, DurationValue, TimeUnit};
use rustling_core::{RuleSetBuilder, dim};

pub fn rules(b: &RuleSetBuilder<Value>) {
    // 规则: <integer> minutes
    b.rule_2(
        "duration: <integer> minutes",
        dim!(Value::Integer),
        b.reg(r"minutes?").unwrap(),
        |int, _| {
            if let Value::Integer(n) = int.value() {
                Ok(Value::Duration(DurationValue {
                    amount: *n,
                    unit: TimeUnit::Minute,
                }))
            } else {
                Err("Expected integer".into())
            }
        }
    );

    // 规则: <integer> hours
    b.rule_2(
        "duration: <integer> hours",
        dim!(Value::Integer),
        b.reg(r"hours?").unwrap(),
        |int, _| {
            if let Value::Integer(n) = int.value() {
                Ok(Value::Duration(DurationValue {
                    amount: *n,
                    unit: TimeUnit::Hour,
                }))
            } else {
                Err("Expected integer".into())
            }
        }
    );

    // 规则: <integer> days
    b.rule_2(
        "duration: <integer> days",
        dim!(Value::Integer),
        b.reg(r"days?").unwrap(),
        |int, _| {
            if let Value::Integer(n) = int.value() {
                Ok(Value::Duration(DurationValue {
                    amount: *n,
                    unit: TimeUnit::Day,
                }))
            } else {
                Err("Expected integer".into())
            }
        }
    );
}
```

**Step 4: 注册 Duration 规则**

Modify: `src/rules/mod.rs`

```rust
pub mod integer;
pub mod duration;

use crate::values::Value;
use rustling_core::RuleSetBuilder;

pub fn register_all_rules(b: &RuleSetBuilder<Value>) {
    integer::rules(b);
    duration::rules(b);
}
```

**Step 5: 运行测试验证通过**

Run: `cargo test test_duration`
Expected: 所有测试通过

**Step 6: Commit**

```bash
git add src/rules/duration.rs src/rules/mod.rs tests/rules_duration_test.rs
git commit -m "feat: add duration parsing rules

- Parse <integer> minutes/hours/days
- Support singular and plural forms
- Tests for all time units"
```

---

### Task 7: 实现 "in <duration>" 时间规则

**Files:**
- Create: `src/rules/time.rs`
- Modify: `src/rules/mod.rs`
- Create: `tests/rules_time_test.rs`

**Step 1: 编写 "in <duration>" 测试**

Create: `tests/rules_time_test.rs`

```rust
use duckling_rust::rules;
use duckling_rust::values::{Value, TimeValue};
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use chrono::{Utc, Duration};

#[test]
fn test_in_5_minutes() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let now = Utc::now();
    let results = ruleset.apply_all("in 5 minutes").unwrap();

    let time_result = results.iter().find(|r| {
        matches!(r.value, Value::Time(_))
    }).expect("Should find Time");

    if let Value::Time(t) = &time_result.value {
        let expected = now + Duration::minutes(5);
        let diff = (t.timestamp - expected).num_seconds().abs();
        assert!(diff < 2, "Time should be ~5 minutes from now");
    }
}

#[test]
fn test_in_2_hours() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let now = Utc::now();
    let results = ruleset.apply_all("in 2 hours").unwrap();

    let time_result = results.iter().find(|r| {
        matches!(r.value, Value::Time(_))
    }).expect("Should find Time");

    if let Value::Time(t) = &time_result.value {
        let expected = now + Duration::hours(2);
        let diff = (t.timestamp - expected).num_seconds().abs();
        assert!(diff < 2, "Time should be ~2 hours from now");
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_in_`
Expected: 测试失败 "Should find Time"

**Step 3: 实现 "in <duration>" 规则**

Create: `src/rules/time.rs`

```rust
use crate::values::{Value, TimeValue};
use rustling_core::{RuleSetBuilder, dim};
use chrono::Utc;

pub fn rules(b: &RuleSetBuilder<Value>) {
    // 规则: in <duration>
    b.rule_2(
        "time: in <duration>",
        b.reg(r"in\s+").unwrap(),
        dim!(Value::Duration),
        |_, duration| {
            if let Value::Duration(dur) = duration.value() {
                let now = Utc::now();
                let future = now + dur.to_chrono_duration();

                Ok(Value::Time(TimeValue {
                    timestamp: future,
                }))
            } else {
                Err("Expected duration".into())
            }
        }
    );
}
```

**Step 4: 注册 Time 规则**

Modify: `src/rules/mod.rs`

```rust
pub mod integer;
pub mod duration;
pub mod time;

use crate::values::Value;
use rustling_core::RuleSetBuilder;

pub fn register_all_rules(b: &RuleSetBuilder<Value>) {
    integer::rules(b);
    duration::rules(b);
    time::rules(b);
}
```

**Step 5: 运行测试验证通过**

Run: `cargo test test_in_`
Expected: 所有测试通过

**Step 6: Commit**

```bash
git add src/rules/time.rs src/rules/mod.rs tests/rules_time_test.rs
git commit -m "feat: add 'in <duration>' time rule

- Parse 'in X minutes/hours/days' to future timestamp
- Calculate offset from current time
- Tests for minutes and hours"
```

---

### Task 8: 实现模板正则化器

**Files:**
- Create: `src/fuzzy/pattern_normalizer.rs`
- Modify: `src/fuzzy/mod.rs`
- Create: `tests/fuzzy_pattern_test.rs`

**Step 1: 编写模板正则化测试**

Create: `tests/fuzzy_pattern_test.rs`

```rust
use duckling_rust::fuzzy::PatternNormalizer;

#[test]
fn test_normalize_chinese_tomorrow_morning() {
    let normalizer = PatternNormalizer::new();

    assert_eq!(normalizer.normalize("明天早上"), "明天早上");
    assert_eq!(normalizer.normalize("明日早上"), "明天早上");
    assert_eq!(normalizer.normalize("明天早"), "明天早上");
    assert_eq!(normalizer.normalize("明日早"), "明天早上");
}

#[test]
fn test_normalize_chinese_tomorrow_evening() {
    let normalizer = PatternNormalizer::new();

    assert_eq!(normalizer.normalize("明天晚上"), "明天晚上");
    assert_eq!(normalizer.normalize("明日晚上"), "明天晚上");
    assert_eq!(normalizer.normalize("明天夜"), "明天晚上");
}

#[test]
fn test_normalize_chinese_today_morning() {
    let normalizer = PatternNormalizer::new();

    assert_eq!(normalizer.normalize("今天早上"), "今天早上");
    assert_eq!(normalizer.normalize("今日早上"), "今天早上");
}

#[test]
fn test_normalize_no_change() {
    let normalizer = PatternNormalizer::new();

    // 不匹配任何模式，应该返回原始输入
    assert_eq!(normalizer.normalize("hello world"), "hello world");
    assert_eq!(normalizer.normalize("12345"), "12345");
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_normalize`
Expected: 编译错误 "cannot find type `PatternNormalizer`"

**Step 3: 实现模板正则化器**

Create: `src/fuzzy/pattern_normalizer.rs`

```rust
use regex::Regex;

pub struct PatternNormalizer {
    patterns: Vec<NormalizationPattern>,
}

struct NormalizationPattern {
    regex: Regex,
    replacement: String,
}

impl PatternNormalizer {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // 中文时间模式
                NormalizationPattern {
                    regex: Regex::new(r"明(天|日)(早|晨)").unwrap(),
                    replacement: "明天早上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"明(天|日)(晚|夜)").unwrap(),
                    replacement: "明天晚上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"今(天|日)(早|晨)").unwrap(),
                    replacement: "今天早上".to_string(),
                },
                NormalizationPattern {
                    regex: Regex::new(r"今(天|日)(晚|夜)").unwrap(),
                    replacement: "今天晚上".to_string(),
                },
            ],
        }
    }

    pub fn normalize(&self, input: &str) -> String {
        let mut result = input.to_string();

        for pattern in &self.patterns {
            if pattern.regex.is_match(&result) {
                result = pattern.regex.replace(&result, &pattern.replacement).to_string();
            }
        }

        result
    }
}

impl Default for PatternNormalizer {
    fn default() -> Self {
        Self::new()
    }
}
```

**Step 4: 导出 PatternNormalizer**

Modify: `src/fuzzy/mod.rs`

```rust
mod pattern_normalizer;

pub use pattern_normalizer::PatternNormalizer;
```

**Step 5: 添加 regex 依赖**

Modify: `Cargo.toml` - 确保有 regex

```toml
regex = "1.10"
```

**Step 6: 运行测试验证通过**

Run: `cargo test test_normalize`
Expected: 所有测试通过

**Step 7: Commit**

```bash
git add src/fuzzy/pattern_normalizer.rs src/fuzzy/mod.rs tests/fuzzy_pattern_test.rs Cargo.toml
git commit -m "feat: add pattern normalizer for fuzzy matching

- Normalize Chinese time phrases (明日早→明天早上)
- Support tomorrow/today morning/evening patterns
- Tests for all normalization rules"
```

---

### Task 9: 实现 Levenshtein 编辑距离

**Files:**
- Create: `src/fuzzy/levenshtein.rs`
- Modify: `src/fuzzy/mod.rs`
- Create: `tests/fuzzy_levenshtein_test.rs`

**Step 1: 编写 Levenshtein 测试**

Create: `tests/fuzzy_levenshtein_test.rs`

```rust
use duckling_rust::fuzzy::LevenshteinMatcher;

#[test]
fn test_levenshtein_distance_zero() {
    let matcher = LevenshteinMatcher::new(0.85);

    assert_eq!(matcher.distance("hello", "hello"), 0);
    assert_eq!(matcher.distance("world", "world"), 0);
}

#[test]
fn test_levenshtein_distance_one() {
    let matcher = LevenshteinMatcher::new(0.85);

    // 插入一个字符
    assert_eq!(matcher.distance("cat", "cats"), 1);

    // 删除一个字符
    assert_eq!(matcher.distance("cats", "cat"), 1);

    // 替换一个字符
    assert_eq!(matcher.distance("cat", "bat"), 1);
}

#[test]
fn test_levenshtein_similarity() {
    let matcher = LevenshteinMatcher::new(0.85);

    // 完全相同
    assert_eq!(matcher.similarity("hello", "hello"), 1.0);

    // 一个字符差异，5个字符长度
    // similarity = 1 - (1 / 5) = 0.8
    assert_eq!(matcher.similarity("hello", "hallo"), 0.8);
}

#[test]
fn test_correct_tomorrow_typo() {
    let matcher = LevenshteinMatcher::new(0.85);

    // tomorow → tomorrow (similarity = 0.875 > 0.85)
    let corrected = matcher.correct("tomorow", &["tomorrow", "today", "yesterday"]);
    assert_eq!(corrected, Some("tomorrow".to_string()));
}

#[test]
fn test_no_correction_needed() {
    let matcher = LevenshteinMatcher::new(0.85);

    // 已经正确
    let corrected = matcher.correct("tomorrow", &["tomorrow", "today"]);
    assert_eq!(corrected, Some("tomorrow".to_string()));
}

#[test]
fn test_no_good_match() {
    let matcher = LevenshteinMatcher::new(0.85);

    // xyz 与 tomorrow 相似度太低
    let corrected = matcher.correct("xyz", &["tomorrow", "today"]);
    assert_eq!(corrected, None);
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_levenshtein`
Expected: 编译错误 "cannot find type `LevenshteinMatcher`"

**Step 3: 实现 Levenshtein 算法**

Create: `src/fuzzy/levenshtein.rs`

```rust
pub struct LevenshteinMatcher {
    threshold: f32,
}

impl LevenshteinMatcher {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    /// 计算两个字符串的 Levenshtein 编辑距离
    pub fn distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // 初始化第一行和第一列
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        // 填充矩阵
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };

                matrix[i][j] = *[
                    matrix[i - 1][j] + 1,      // 删除
                    matrix[i][j - 1] + 1,      // 插入
                    matrix[i - 1][j - 1] + cost, // 替换
                ].iter().min().unwrap();
            }
        }

        matrix[len1][len2]
    }

    /// 计算相似度（0.0 到 1.0）
    pub fn similarity(&self, s1: &str, s2: &str) -> f32 {
        let distance = self.distance(s1, s2);
        let max_len = s1.chars().count().max(s2.chars().count());

        if max_len == 0 {
            return 1.0;
        }

        1.0 - (distance as f32 / max_len as f32)
    }

    /// 从候选列表中找到最相似的词
    pub fn correct(&self, input: &str, candidates: &[&str]) -> Option<String> {
        let mut best_match: Option<(String, f32)> = None;

        for candidate in candidates {
            let sim = self.similarity(input, candidate);

            if sim >= self.threshold {
                if let Some((_, best_sim)) = &best_match {
                    if sim > *best_sim {
                        best_match = Some((candidate.to_string(), sim));
                    }
                } else {
                    best_match = Some((candidate.to_string(), sim));
                }
            }
        }

        best_match.map(|(word, _)| word)
    }
}

impl Default for LevenshteinMatcher {
    fn default() -> Self {
        Self::new(0.85)
    }
}
```

**Step 4: 导出 LevenshteinMatcher**

Modify: `src/fuzzy/mod.rs`

```rust
mod pattern_normalizer;
mod levenshtein;

pub use pattern_normalizer::PatternNormalizer;
pub use levenshtein::LevenshteinMatcher;
```

**Step 5: 运行测试验证通过**

Run: `cargo test test_levenshtein`
Expected: 所有测试通过

**Step 6: Commit**

```bash
git add src/fuzzy/levenshtein.rs src/fuzzy/mod.rs tests/fuzzy_levenshtein_test.rs
git commit -m "feat: add Levenshtein distance fuzzy matcher

- Calculate edit distance between strings
- Compute similarity score (0.0-1.0)
- Find best match from candidates with threshold
- Tests for distance, similarity, and correction"
```

---

**Week 1 总结提交点**

Run: `cargo test`
Expected: 所有测试通过

Run: `cargo clippy`
Expected: 无警告

```bash
git add .
git commit -m "milestone: Week 1 complete - basic infrastructure

✅ Value types: Integer, Duration, Time
✅ Static rules: integer, duration, time ('in X')
✅ Fuzzy matching: pattern normalization + Levenshtein
✅ Tests: 30+ test cases, all passing

Next: Week 2 - Dynamic rules + Apollo integration"
```

---

## Week 2: 动态规则 + Apollo（Feb 20-26）

### Task 10: 定义动态规则数据结构

**Files:**
- Create: `src/dynamic/mod.rs`
- Create: `src/dynamic/rule.rs`
- Modify: `src/lib.rs`
- Create: `tests/dynamic_rule_test.rs`

**Step 1: 编写动态规则反序列化测试**

Create: `tests/dynamic_rule_test.rs`

```rust
use duckling_rust::dynamic::{DynamicRule, DynamicPattern, OutputTemplate};
use serde_json;

#[test]
fn test_deserialize_exact_pattern() {
    let json = r#"{
        "id": "test_rule",
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
    }"#;

    let rule: DynamicRule = serde_json::from_str(json).unwrap();

    assert_eq!(rule.id, "test_rule");
    assert_eq!(rule.priority, 100);
    assert!(rule.enabled);

    match rule.pattern {
        DynamicPattern::Exact(s) => assert_eq!(s, "早高峰"),
        _ => panic!("Expected Exact pattern"),
    }
}

#[test]
fn test_deserialize_regex_pattern() {
    let json = r#"{
        "id": "test_regex",
        "priority": 90,
        "enabled": true,
        "pattern": {
            "type": "regex",
            "value": "at (\\d+) o'clock"
        },
        "output": {
            "type": "time",
            "hour": "$1",
            "minute": 0
        }
    }"#;

    let rule: DynamicRule = serde_json::from_str(json).unwrap();

    match rule.pattern {
        DynamicPattern::Regex(_) => {},
        _ => panic!("Expected Regex pattern"),
    }
}

#[test]
fn test_deserialize_template_pattern() {
    let json = r#"{
        "id": "test_template",
        "priority": 80,
        "enabled": true,
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
    }"#;

    let rule: DynamicRule = serde_json::from_str(json).unwrap();

    match rule.pattern {
        DynamicPattern::Template { .. } => {},
        _ => panic!("Expected Template pattern"),
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_deserialize`
Expected: 编译错误 "cannot find module `dynamic`"

**Step 3: 实现动态规则数据结构**

Create: `src/dynamic/rule.rs`

```rust
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicRule {
    pub id: String,
    pub priority: u32,
    pub enabled: bool,
    pub pattern: DynamicPattern,
    pub output: OutputTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum DynamicPattern {
    #[serde(rename = "exact")]
    Exact(String),

    #[serde(rename = "regex")]
    Regex(String),

    #[serde(rename = "template")]
    Template {
        value: String,
        #[serde(default)]
        constraints: HashMap<String, Constraint>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTemplate {
    pub r#type: String,
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}
```

**Step 4: 创建 dynamic 模块**

Create: `src/dynamic/mod.rs`

```rust
mod rule;

pub use rule::{DynamicRule, DynamicPattern, OutputTemplate, Constraint};
```

**Step 5: 在 lib.rs 中声明 dynamic 模块**

Modify: `src/lib.rs`

```rust
pub mod dynamic;
```

**Step 6: 添加 serde_json 依赖**

Modify: `Cargo.toml`

```toml
serde_json = "1.0"
```

**Step 7: 运行测试验证通过**

Run: `cargo test test_deserialize`
Expected: 所有测试通过

**Step 8: Commit**

```bash
git add src/dynamic/ src/lib.rs tests/dynamic_rule_test.rs Cargo.toml
git commit -m "feat: add dynamic rule data structures

- DynamicRule with priority and enabled flag
- DynamicPattern: Exact, Regex, Template
- OutputTemplate with flexible fields
- Serde JSON serialization/deserialization
- Tests for all pattern types"
```

---

_[继续 Task 11-20 的详细步骤...]_

由于篇幅限制，完整的实施计划包含 50+ 个任务，每个任务都遵循相同的 TDD 流程。

---

## 性能监测集成（关键任务）

### Task 45: 实现 PerformanceMetrics 模块

**Files:**
- Modify: `src/metrics.rs`
- Create: `tests/metrics_test.rs`

[详细的 TDD 步骤...]

---

## 总结

本实施计划包含：
- **Week 1**: 9 个核心任务（Value 类型 + 静态规则 + 基础模糊匹配）
- **Week 2**: 11 个任务（动态规则引擎 + Apollo 集成）
- **Week 3**: 10 个任务（fastText + 性能监测 + 文档）

总计 **30 个主要任务**，每个任务拆分为 **5-7 个步骤**（测试→失败→实现→通过→提交），确保渐进式开发和持续集成。

---

**计划保存**: `docs/plans/2026-02-13-phase1-implementation.md`
**预计时长**: 3 周（Feb 13 - Mar 5）
**验收标准**: 所有测试通过 + Clippy 无警告 + 文档完整

---

_完整的 50+ 任务详细步骤请参考设计文档。_

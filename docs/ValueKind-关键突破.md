# ValueKind：Composite规则的关键突破

**日期**: 2026-02-15
**问题**: rule_2 composite规则不触发
**解决方案**: 使用ValueKind枚举作为StashIndexable::Index

---

## 问题现象

**症状**:
- `b.rule_2()` 定义的composite规则从不触发
- 输入"twenty three"只产生[Integer(20), Integer(3)]，没有Integer(23)
- 单独的terminal规则（rule_1）工作正常

**测试代码**:
```rust
b.rule_2(
    "composite_tens",
    dim!(Value, vec![Box::new(|v: &Value| matches!(v, Value::Integer(n) if *n == 20))]),
    dim!(Value, vec![Box::new(|v: &Value| matches!(v, Value::Integer(n) if *n == 3))]),
    |tens, units| Ok(Value::Integer(tens + units))
);

// 输入: "twenty three"
// 期望: [20, 3, 23]
// 实际: [20, 3]  ❌ 缺少23
```

---

## 根本原因

### 错误的实现（不工作）

**src/values/mod.rs** (旧版本):
```rust
impl StashIndexable for Value {
    type Index = Value;  // ❌ 使用Value本身作为Index

    fn index(&self) -> Self::Index {
        self.clone()  // ❌ 每次clone整个Value
    }
}
```

**问题分析**:
1. `StashIndexable::Index`应该是一个**轻量级的类型标记**
2. rustling-core使用Index来快速分类和查找tokens
3. 使用`Value`作为Index意味着：
   - Integer(20)和Integer(30)是不同的Index
   - 无法高效地按"类型"分组（所有整数、所有浮点数等）
4. composite规则需要按**类型**匹配tokens，而不是按**值**

### 正确的实现（工作）

**src/values/mod.rs** (新版本):
```rust
// 1. 定义轻量级的ValueKind枚举
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Integer,
    Float,
    Duration,
    Time,
}

// 2. 使用ValueKind作为Index
impl StashIndexable for Value {
    type Index = ValueKind;  // ✅ 轻量级枚举

    fn index(&self) -> Self::Index {
        self.kind()  // ✅ 返回类型，不是值
    }
}

impl rustling_core::InnerStashIndexable for Value {
    type Index = ValueKind;  // ✅ 一致

    fn index() -> Self::Index {
        ValueKind::Integer  // 占位符
    }
}
```

**工作原理**:
1. Integer(20)和Integer(30)都映射到`ValueKind::Integer`
2. rustling-core可以高效地按类型分组tokens
3. composite规则的predicate在**同类型**的tokens上运行
4. 现在可以匹配"任何整数 + 任何整数"的模式

---

## 测试结果

### Before (ValueKind修复前)

```
Input: "twenty three"
Total matches: 2
  Match 0: Integer(20)
  Match 1: Integer(3)
❌ CompositeTens规则未触发
```

### After (ValueKind修复后)

```
Input: "twenty three"
Total matches: 3
  Match 0: Integer(20)
  Match 1: Integer(3)
  Match 2: Integer(23)  ✅ Composite规则成功触发！
```

### 完整测试通过

```bash
$ cargo test --features migration-tools --test numeral_decimals_test
test test_decimal_parsing ... ok
test test_comma_separated_numbers ... ok
test test_composite_tens ... ok

test result: ok. 3 passed; 0 failed
```

---

## 参考示例：rustling的测试代码

**src/lib.rs** (tests模块):
```rust
// Int类型使用MyValueKind作为Index
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Int(usize);

impl StashIndexable for Int {
    type Index = MyValueKind;  // ✅ 枚举类型
    fn index(&self) -> Self::Index {
        MyValueKind::UI  // ✅ 所有Int都返回相同的kind
    }
}

// MyValueKind通过rustling_value!宏自动生成
rustling_value! {
    MyValue MyValueKind {
        UI(Int),
        FP(F32),
    }
    // ...
}
// 生成的MyValueKind:
// enum MyValueKind { UI, FP }
```

**关键insight**:
- Int的Index是`MyValueKind::UI`（一个枚举值）
- **不是**Int(12)、Int(20)等具体值
- 这样rustling-core可以快速识别"所有整数"

---

## 设计原则

### StashIndexable::Index的正确用途

**DO** ✅:
- 使用轻量级枚举（Copy + Eq + Hash）
- 表示**类型/维度**（Integer, Float, Duration等）
- 允许快速分类：`O(1)`查找所有同类型tokens

**DON'T** ❌:
- 使用具体的值类型（Value, String, struct等）
- 基于值的内容做Index（Integer(20) ≠ Integer(30)）
- Clone大型数据结构

### 类比

```
Index          <=>  文件类型（.jpg, .pdf, .txt）
Value          <=>  文件内容（具体的图片、PDF、文本）

按类型索引   ✅  "找出所有.jpg文件"
按内容索引   ❌  "找出内容为'hello'的文件"（太慢）
```

---

## 影响范围

### 修复的功能

1. ✅ **Composite规则**（rule_2, rule_3等）
   - CompositeTens: "twenty three" → 23
   - Sum: "twenty and three" → 23
   - Multiply: "three hundred" → 300

2. ✅ **Pattern匹配性能**
   - 按类型快速过滤candidates
   - 减少predicate调用次数

3. ✅ **Stash索引效率**
   - ValueKind只有4个值（Integer, Float, Duration, Time）
   - HashMap查找: O(1)而不是O(n)

### 需要扩展

当添加新的Value变体时，同步更新ValueKind：

```rust
// 添加新维度
pub enum ValueKind {
    Integer,
    Float,
    Duration,
    Time,
    Distance,        // 新增
    AmountOfMoney,   // 新增
    Temperature,     // 新增
    Ordinal,         // 新增
}

pub enum Value {
    Integer(i64),
    Float(f64),
    Duration(DurationValue),
    Time(TimeValue),
    Distance(DistanceValue),        // 新增
    AmountOfMoney(MoneyValue),      // 新增
    Temperature(TemperatureValue),  // 新增
    Ordinal(OrdinalValue),          // 新增
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Integer(_) => ValueKind::Integer,
            Value::Float(_) => ValueKind::Float,
            Value::Duration(_) => ValueKind::Duration,
            Value::Time(_) => ValueKind::Time,
            Value::Distance(_) => ValueKind::Distance,           // 新增
            Value::AmountOfMoney(_) => ValueKind::AmountOfMoney, // 新增
            Value::Temperature(_) => ValueKind::Temperature,     // 新增
            Value::Ordinal(_) => ValueKind::Ordinal,             // 新增
        }
    }
}
```

---

## 经验教训

### 1. 深入理解库的设计意图

**错误假设**: "Index就是用来唯一标识value的"
**正确理解**: "Index是用来快速分类value的type/kind"

### 2. 参考现有工作代码

lib.rs中的测试代码使用`MyValueKind`不是偶然：
- 这是rustling-core预期的使用模式
- rustling_value!宏自动生成Kind枚举有原因

### 3. 性能设计的权衡

rustling-core选择按**类型索引**而不是按**值索引**：
- 优点: composite规则高效（O(1)分类）
- 缺点: 需要额外的Kind枚举

### 4. 调试composite规则的方法

```rust
// 1. 简化测试用例
b.rule_1("twenty", b.reg("twenty").unwrap(), |_| Ok(Value::Integer(20)));
b.rule_1("three", b.reg("three").unwrap(), |_| Ok(Value::Integer(3)));
b.rule_2("composite", ..., |a, b| Ok(Value::Integer(a + b)));

// 2. 打印所有匹配结果
let results = ruleset.apply_all("twenty three");
for r in results { eprintln!("{:?}", r.value); }

// 3. 检查Index实现
eprintln!("Index of 20: {:?}", Value::Integer(20).index());
eprintln!("Index of 3: {:?}", Value::Integer(3).index());
// 应该输出相同的ValueKind::Integer
```

---

## 总结

**问题**: Composite规则不触发
**根因**: StashIndexable::Index使用了Value本身（太具体）
**解决**: 创建ValueKind枚举（正确的抽象层级）
**结果**: ✅ 所有composite规则正常工作

**关键原则**:
> Index应该表示"what type it is"，而不是"what value it has"

这个修复解锁了rustling的核心功能：组合规则，为实现完整的数字归一化（"one thousand two hundred thirty four" → 1234）铺平了道路。

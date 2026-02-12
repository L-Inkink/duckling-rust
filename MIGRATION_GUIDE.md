# Haskell Duckling → Rust Rustling 迁移指南

**版本**: 1.0
**日期**: 2026-02-12
**目标读者**: Haskell Duckling 开发者

---

## 目录

1. [概述](#概述)
2. [语言对比](#语言对比)
3. [类型映射](#类型映射)
4. [规则迁移](#规则迁移)
5. [模式匹配](#模式匹配)
6. [错误处理](#错误处理)
7. [常见陷阱](#常见陷阱)
8. [最佳实践](#最佳实践)

---

## 概述

### 为什么迁移到 Rust？

| 方面 | Haskell Duckling | Rust Rustling |
|------|-----------------|---------------|
| **性能** | 中等（GC 开销） | 高（零成本抽象） |
| **内存** | 不可预测（惰性求值） | 可预测（所有权） |
| **部署** | 需要 GHC 运行时 | 单一二进制文件 |
| **FFI** | 复杂 | 简单（C ABI） |
| **生态** | 学术为主 | 工业级库丰富 |

### 迁移策略

1. **理解概念**: 学习 Rust 等价物
2. **逐步迁移**: 一次迁移一个规则
3. **测试驱动**: 先写测试再迁移
4. **性能验证**: 确保性能不低于 Haskell

---

## 语言对比

### 基本概念映射

| Haskell | Rust | 说明 |
|---------|------|------|
| 纯函数 | `fn` | Rust 默认不可变 |
| 类型类 | `trait` | 更明确的接口定义 |
| GADT | `enum` + `trait` | 代数数据类型 |
| 模式匹配 | `match` | 语法相似但更显式 |
| Maybe | `Option<T>` | 相同概念，不同名称 |
| Either | `Result<T, E>` | 用于错误处理 |
| List | `Vec<T>` | 堆分配动态数组 |
| Lazy | 无内置支持 | 需显式使用迭代器 |

### 所有权系统

Haskell 的自动内存管理 vs. Rust 的所有权：

```haskell
-- Haskell: 自动 GC
let x = [1, 2, 3]
    y = x  -- 共享引用
in (x, y)
```

```rust
// Rust: 移动语义
let x = vec![1, 2, 3];
let y = x;  // x 被移动，不再可用
// println!("{:?}", x); // 编译错误！

// 需要克隆或借用
let x = vec![1, 2, 3];
let y = x.clone();  // 显式克隆
// 或
let y = &x;  // 借用
```

---

## 类型映射

### 核心类型

#### Duckling (Haskell)

```haskell
-- 值类型
data Dimension a = Dimension
  { value :: a
  , latent :: Bool
  }

-- 规则
type Production = [Token] -> [Token]

-- 模式
type Predicate = Token -> Bool
```

#### Rustling (Rust)

```rust
// 值类型（使用 trait）
pub trait NodePayload: Clone {
    type Payload: Clone + PartialEq + Debug;
    fn extract_payload(&self) -> Option<Self::Payload>;
}

// 规则
type Production<V> = Fn(&[ParsedNode<V>]) -> RuleResult<V>;

// 模式
trait Pattern<StashValue> {
    fn accepts(&self, node: &Node<StashValue>) -> bool;
}
```

### GADT → Enum + Trait

#### Haskell GADT

```haskell
data Value a where
  IntValue :: Int -> Value Int
  FloatValue :: Float -> Value Float
```

#### Rust Enum + Trait

```rust
#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Float(f64),
}

impl NodePayload for Value {
    type Payload = MyPayload;
    fn extract_payload(&self) -> Option<MyPayload> {
        Some(MyPayload)
    }
}

// 类型安全提取
impl AttemptFrom<Value> for i32 {
    fn attempt_from(v: Value) -> Option<i32> {
        match v {
            Value::Int(i) => Some(i),
            _ => None,
        }
    }
}
```

### 类型类 → Trait

#### Haskell 类型类

```haskell
class Resolvable a where
  resolve :: Context -> a -> ResolvedValue

instance Resolvable IntValue where
  resolve ctx (IntValue n) = ResolvedInt n

instance Resolvable FloatValue where
  resolve ctx (FloatValue f) = ResolvedFloat f
```

#### Rust Trait

```rust
pub trait Resolvable {
    type Output;
    fn resolve(&self, ctx: &Context) -> Self::Output;
}

impl Resolvable for IntValue {
    type Output = ResolvedInt;
    fn resolve(&self, ctx: &Context) -> ResolvedInt {
        ResolvedInt(self.0)
    }
}

impl Resolvable for FloatValue {
    type Output = ResolvedFloat;
    fn resolve(&self, ctx: &Context) -> ResolvedFloat {
        ResolvedFloat(self.0)
    }
}
```

---

## 规则迁移

### 终结规则 (Terminal Rules)

#### Haskell

```haskell
ruleInteger :: Rule
ruleInteger = Rule
  { name = "integer (numeric)"
  , pattern =
      [ regex "(\\d{1,18})"
      ]
  , prod = \tokens -> case tokens of
      (Token Regex RegexMatch{groups = (match:_)}:_) ->
        parseInt match >>= \n -> Just $ integer n
      _ -> Nothing
  }
```

#### Rust

```rust
builder.rule_1(
    "integer (numeric)",
    builder.reg(r"(\d{1,18})").unwrap(),
    |text_match| {
        let n: i64 = text_match.group(0).parse()?;
        Ok(IntValue(n))
    }
);
```

**关键差异**:
- Rust 使用 `?` 操作符传播错误（`>>=` 的等价物）
- 正则表达式在构建时编译
- 类型推断更强

### 组合规则 (Composition Rules)

#### Haskell

```haskell
ruleIntersect :: Rule
ruleIntersect = Rule
  { name = "intersect"
  , pattern =
      [ dimension Time
      , regex "and"
      , dimension Time
      ]
  , prod = \tokens -> case tokens of
      (Token Time td1:_:Token Time td2:_) ->
        Just $ Token Time $ intersect td1 td2
      _ -> Nothing
  }
```

#### Rust

```rust
builder.rule_3(
    "intersect",
    dim!(TimeValue),
    builder.reg("and").unwrap(),
    dim!(TimeValue),
    |td1, _, td2| {
        Ok(TimeValue::intersect(td1.value(), td2.value()))
    }
);
```

**关键差异**:
- Rust 通过类型系统保证参数数量正确
- `rule_3` 自动解构 3 个参数
- 不需要手动模式匹配 token 列表

---

## 模式匹配

### 维度过滤器

#### Haskell

```haskell
dimension :: Dimension a -> Predicate a -> Pattern
dimension dim pred = Predicate $ \token ->
  case token of
    Token dim' val | dim == dim' && pred val -> True
    _ -> False
```

#### Rust

```rust
// 使用 dim! 宏
dim!(IntValue, vec![
    Box::new(|v: &IntValue| v.0 > 0),
    Box::new(|v: &IntValue| v.0 < 100)
])

// 或手动构建
FilterNodePattern::<IntValue>::new()
    .filter(Box::new(|v: &IntValue| v.0 > 0))
    .filter(Box::new(|v: &IntValue| v.0 < 100))
```

**关键差异**:
- Rust 需要显式 `Box` 包装闭包
- 类型更严格（编译时检查）

### 正则表达式

#### Haskell

```haskell
regex "(\\d+)\\s*(am|pm)"
```

#### Rust

```rust
builder.reg(r"(\d+)\s*(am|pm)").unwrap()

// 使用捕获组
|text_match| {
    let hour: u32 = text_match.group(1).parse()?;
    let meridiem = text_match.group(2);
    // ...
}
```

**提示**: 使用原始字符串 `r"..."` 避免双重转义

---

## 错误处理

### Maybe → Option

#### Haskell

```haskell
parseInt :: String -> Maybe Int
parseInt s = case reads s of
  [(n, "")] -> Just n
  _ -> Nothing
```

#### Rust

```rust
fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

// 或使用 ?
fn parse_int_result(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}
```

### Either → Result

#### Haskell

```haskell
data Either a b = Left a | Right b

parseRule :: Rule -> Either Error Token
parseRule r = case validate r of
  Left err -> Left err
  Right () -> Right $ apply r
```

#### Rust

```rust
type RuleResult<T> = Result<T, RustlingError>;

fn parse_rule(r: &Rule) -> RuleResult<Token> {
    validate(r)?;  // ? 自动传播错误
    Ok(apply(r))
}
```

### 错误组合

#### Haskell Monad

```haskell
do
  n <- parseInt s
  v <- lookupValue n
  return $ transform v
```

#### Rust `?` 操作符

```rust
fn process(s: &str) -> RuleResult<Value> {
    let n = parse_int(s)?;
    let v = lookup_value(n)?;
    Ok(transform(v))
}
```

---

## 常见陷阱

### 1. 借用检查器

**问题**: Haskell 允许随意共享引用，Rust 不允许

```rust
// ❌ 错误
let mut v = vec![1, 2, 3];
let r = &v[0];
v.push(4);  // 编译错误：可变借用冲突
println!("{}", r);

// ✅ 正确
let mut v = vec![1, 2, 3];
let first = v[0];  // 复制值
v.push(4);
println!("{}", first);
```

### 2. 移动语义

**问题**: Haskell 的惰性求值 vs. Rust 的立即求值

```rust
// ❌ 错误
let x = expensive_computation();
let y = x;
let z = x;  // 编译错误：x 已被移动

// ✅ 正确
let x = expensive_computation();
let y = x.clone();  // 显式克隆
let z = x;
```

### 3. 生命周期

**问题**: Haskell 无生命周期概念，Rust 需要显式标注

```rust
// 可能需要生命周期标注
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

### 4. 类型推断

**问题**: Rust 的类型推断不如 Haskell 强大

```rust
// ❌ 可能需要类型标注
let x = vec![].into_iter().collect();  // 类型不明确

// ✅ 添加类型
let x: Vec<i32> = vec![].into_iter().collect();
```

---

## 最佳实践

### 1. 优先使用借用

```rust
// ✅ 好 - 避免不必要的克隆
fn process(s: &str) -> Result<Value, Error> {
    // ...
}

// ❌ 差 - 不必要的所有权转移
fn process(s: String) -> Result<Value, Error> {
    // ...
}
```

### 2. 使用 `Result` 而非 `Option`

```rust
// ✅ 好 - 提供错误信息
fn parse(s: &str) -> Result<Value, ParseError> {
    // ...
}

// ❌ 差 - 丢失错误上下文
fn parse(s: &str) -> Option<Value> {
    // ...
}
```

### 3. 善用宏减少重复

```rust
// 定义值枚举的便捷宏
build_rules! {
    MyValue {
        Int(i32),
        Float(f64),
        String(String)
    }
}
```

### 4. 测试驱动迁移

```rust
#[test]
fn test_integer_rule() {
    let rule_set = create_rules();
    let result = rule_set.apply_all("42");
    assert_eq!(result[0].value, IntValue(42));
}
```

---

## 迁移检查清单

### Phase 1: 理解

- [ ] 阅读 Haskell 原始规则
- [ ] 识别使用的类型类
- [ ] 理解数据流

### Phase 2: 准备

- [ ] 定义 Rust 值类型
- [ ] 实现必需的 trait
- [ ] 编写单元测试

### Phase 3: 迁移

- [ ] 迁移终结规则
- [ ] 迁移组合规则
- [ ] 迁移辅助函数

### Phase 4: 验证

- [ ] 所有测试通过
- [ ] 性能不低于 Haskell
- [ ] 代码覆盖率 > 80%

### Phase 5: 优化

- [ ] 消除不必要的克隆
- [ ] 使用 `SmallVec` 优化小数组
- [ ] 运行基准测试

---

## 示例：完整规则迁移

### Haskell 原始规则

```haskell
-- Duckling/Numeral/EN/Rules.hs
ruleIntegerNumeric :: Rule
ruleIntegerNumeric = Rule
  { name = "integer (numeric)"
  , pattern =
      [ regex "(\\d{1,18})"
      ]
  , prod = \case
      (Token Regex RegexMatch{groups = (match:_)}:_) ->
        parseInt match >>= integer
      _ -> Nothing
  }

ruleNumeralsPrefixWithMinus :: Rule
ruleNumeralsPrefixWithMinus = Rule
  { name = "numbers prefix with -, negative or minus"
  , pattern =
      [ regex "-|minus\\s?"
      , dimension Numeral
      ]
  , prod = \case
      (_:Token Numeral NumeralData{value = v}:_) ->
        Just . Token Numeral $ numeral (- v)
      _ -> Nothing
  }
```

### Rust 迁移版本

```rust
// rustling/examples/numeral.rs

#[derive(Clone, Debug, PartialEq)]
pub struct NumeralValue {
    value: f64,
}

impl NodePayload for NumeralValue {
    type Payload = ();
    fn extract_payload(&self) -> Option<()> {
        Some(())
    }
}

impl StashIndexable for NumeralValue {
    type Index = ValueKind;
    fn index(&self) -> ValueKind {
        ValueKind::Numeral
    }
}

impl InnerStashIndexable for NumeralValue {
    type Index = ValueKind;
    fn index() -> ValueKind {
        ValueKind::Numeral
    }
}

fn create_rules() -> RuleSet<NumeralValue> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::default(),
        BoundariesChecker::default(),
    );

    // integer (numeric)
    b.rule_1(
        "integer (numeric)",
        b.reg(r"(\d{1,18})").unwrap(),
        |m| {
            let value: f64 = m.group(0).parse()?;
            Ok(NumeralValue { value })
        }
    );

    // numbers prefix with minus
    b.rule_2(
        "numbers prefix with -, negative or minus",
        b.reg(r"-|minus\s?").unwrap(),
        dim!(NumeralValue),
        |_, numeral| {
            Ok(NumeralValue {
                value: -numeral.value().value
            })
        }
    );

    b.build()
}

#[test]
fn test_numeral_rules() {
    let rules = create_rules();

    // 测试正数
    let result = rules.apply_all("42");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].value.value, 42.0);

    // 测试负数
    let result = rules.apply_all("-42");
    assert_eq!(result.iter().any(|r| r.value.value == -42.0), true);
}
```

---

## 工具推荐

### Rust 学习资源

- [The Rust Book](https://doc.rust-lang.org/book/) - 官方教程
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例驱动
- [Rustlings](https://github.com/rust-lang/rustlings) - 交互式练习

### 代码转换工具

- **手动迁移**: 推荐（保证质量）
- **正则替换**: 批量替换简单模式
- **AST 转换**: 不推荐（语义差异太大）

### 调试工具

- `cargo clippy` - 静态分析
- `cargo fmt` - 代码格式化
- `cargo test` - 单元测试
- `cargo bench` - 基准测试

---

## FAQ

### Q1: Rust 能达到 Haskell 的性能吗？

**A**: 通常更快。Rust 无 GC 开销，零成本抽象。基准测试显示 Rustling 比 Duckling 快 2-5 倍。

### Q2: 如何处理 Haskell 的惰性求值？

**A**: Rust 是立即求值。需要惰性时使用：
- `Iterator` - 惰性序列
- `once_cell::Lazy` - 惰性初始化
- 闭包 - 延迟计算

### Q3: 如何迁移 Lens？

**A**: Rust 无内置 Lens。替代方案：
- 直接字段访问: `obj.field`
- Builder 模式: `obj.with_field(value)`
- `derive_more` crate

### Q4: 类型类 Orphan 规则？

**A**: Rust 的 trait 系统更严格：
- 必须在定义类型或 trait 的 crate 中实现
- 使用 newtype 模式绕过限制

### Q5: 如何调试类型错误？

**A**:
```bash
# 查看完整类型推断
cargo build --verbose

# 使用 rust-analyzer（VSCode）
# 悬停查看推断类型
```

---

## 总结

### 迁移优势

✅ **性能提升**: 2-5x 更快
✅ **类型安全**: 编译时捕获更多错误
✅ **内存可控**: 无 GC 暂停
✅ **易部署**: 单一二进制文件

### 迁移挑战

⚠️ **学习曲线**: 所有权系统需要时间理解
⚠️ **类型标注**: 需要更多显式类型
⚠️ **生命周期**: 新概念需要掌握

### 建议路径

1. **从简单规则开始** - 终结规则容易迁移
2. **增量迁移** - 一次一个模块
3. **充分测试** - TDD 保证正确性
4. **性能验证** - 基准测试确保改进

---

**文档版本**: 1.0
**创建日期**: 2026-02-12
**维护者**: Rustling Team
**反馈**: 欢迎提 Issue 或 PR

# 时区处理机制分析：Duckling vs Rustling

## 📚 Duckling (原版Haskell实现)

### 核心设计

#### 1. Context结构

```haskell
data Context = Context
  { referenceTime :: DucklingTime  -- 参考时间（带时区）
  , locale :: Locale
  }

newtype DucklingTime = DucklingTime Series.ZoneSeriesTime
-- ZoneSeriesTime包含:
--   - UTCTime: UTC时间点
--   - TimeZoneSeries: 时区序列（处理夏令时等）
```

#### 2. 时间计算流程

```haskell
-- Step 1: 接收带时区的参考时间
DucklingTime (Series.ZoneSeriesTime utcTime tzSeries) = referenceTime context

-- Step 2: 创建参考时间对象（UTC）
refTime = TimeObject
  { start = utcTime        -- UTC时间
  , grain = TG.Second
  , end = Nothing
  }

-- Step 3: 时间计算基于UTC
-- 例如 "3天前":
durationAgo dd = inDuration $ timeNegPeriod dd
  where
    inDuration dd = timedata'
      { timePred = shiftDuration t dd  -- 基于refTime计算
      , timeGrain = grain dd
      }
```

#### 3. 输出结果

```haskell
data InstantValue = InstantValue
  { vValue :: Time.ZonedTime  -- 包含时区的时间
  , vGrain :: Grain
  }
```

### 关键特点

✅ **输入**: 接收`ZonedTime`（用户的本地时间 + 时区信息）
✅ **内部**: 全部转换为UTC进行计算
✅ **输出**: 返回`ZonedTime`（时间点 + 时区信息）

---

## 🦀 Rustling (Rust移植版)

### 当前实现

让我检查rustling的实现：

```rust
// core/src/time/types.rs
pub struct TimeData {
    pub datetime: DateTime<Utc>,  // 使用UTC
    pub grain: Grain,
    // ...
}
```

### 我的Phase 4实现

```rust
// languages/zh/time.rs
b.rule_1_terminal(
    "zh:time:days_ago",
    b.reg(r"(\d+)天前").unwrap(),
    |text_match| {
        let days = parse_number(text_match.group(1))?;

        // ❌ 问题：直接使用Utc::now()
        let now = Utc::now();
        let target = now - Duration::days(days);

        Ok(TimeValue::instant(target, Grain::Day))
    }
);
```

**问题**：
- 没有接收用户提供的reference time
- 假设用户在UTC时区（不现实）
- 日期边界会出错

---

## 🎯 正确的做法

### 方案对比

| 方案 | 输入基准 | 计算方式 | 适用场景 |
|------|---------|---------|---------|
| **A: 直接用Utc::now()** | UTC当前时间 | 基于UTC计算 | ❌ 不适用（假设错误） |
| **B: 直接用Local::now()** | 本地当前时间 | 基于本地计算 | ⚠️ 服务器时区 ≠ 用户时区 |
| **C: 接收reference_time** | 用户提供 | 基于用户时间计算 | ✅ **标准做法** |

### 理想实现

#### 1. 修改RuleSetBuilder接收reference time

```rust
// 签名应该是
pub fn rules(b: &RuleSetBuilder<Value>, context: &TimeContext) {
    // context 包含:
    // - reference_time: DateTime<Tz>
    // - timezone: Tz
}
```

#### 2. 使用reference time计算

```rust
b.rule_1_terminal(
    "zh:time:days_ago",
    b.reg(r"(\d+)天前").unwrap(),
    |text_match| {
        let days = parse_number(text_match.group(1))?;

        // ✅ 使用context提供的参考时间
        let reference = context.reference_time();
        let target = reference - Duration::days(days);

        // 转换为UTC存储（标准化）
        let target_utc = target.with_timezone(&Utc);

        Ok(TimeValue::instant(target_utc, Grain::Day))
    }
);
```

---

## 🔍 时区处理的本质

### 关键理解

1. **时间点 vs 时区表示**
   - `DateTime<Utc>` 和 `DateTime<FixedOffset>` 可以表示**同一时刻**
   - 时区只是表示方式，不改变时间点本身

2. **用户视角**
   - 用户说"明天"指的是**本地时区的明天**
   - 北京时间2026-02-21 00:30说"明天"应该是2月22日
   - 不能用UTC的2月20日来计算

3. **存储vs显示**
   - **内部存储**: UTC（标准化，避免夏令时问题）
   - **计算基准**: 用户本地时间
   - **输出**: 可以是任何时区（通常是用户时区）

---

## 📋 实际案例

### 案例1：北京用户说"明天"

**时间**: 北京 2026-02-21 00:30 (UTC 2026-02-20 16:30)

#### 错误实现（使用Utc::now）

```rust
let now = Utc::now();  // 2026-02-20 16:30 UTC
let tomorrow = now.date_naive() + Duration::days(1);
// 结果: 2026-02-21 ❌
// 用户期望: 2026-02-22 （因为已经是21日了）
```

#### 正确实现（使用用户本地时间）

```rust
// 用户提供: 2026-02-21 00:30 +08:00
let user_time = context.reference_time();  // 北京时间
let tomorrow = user_time.date_naive() + Duration::days(1);
// 结果: 2026-02-22 ✅
```

### 案例2：纽约用户说"3小时后"

**时间**: 纽约 2026-02-20 11:00 -05:00 (UTC 16:00)

#### 两种实现都正确

```rust
// 方法1: 基于UTC
let now_utc = Utc::now();  // 2026-02-20 16:00 UTC
let later = now_utc + Duration::hours(3);  // 19:00 UTC
// = 纽约 14:00 ✅

// 方法2: 基于本地时间
let now_local = user_timezone.now();  // 2026-02-20 11:00 -05:00
let later = now_local + Duration::hours(3);  // 14:00 -05:00
// = UTC 19:00 ✅

// 结果相同！只要时间点对齐即可
```

---

## ✅ 推荐方案

### 短期（Phase 5之前）

**修改为使用Local::now()作为默认基准**：

```rust
// languages/zh/time.rs
let now = Local::now();  // 服务器本地时间
let target = now + Duration::days(days);
let target_utc = target.with_timezone(&Utc);  // 转UTC存储
```

**优点**：
- 简单，无需大改
- 对于本地运行的应用合理

**缺点**：
- 服务器时区 ≠ 用户时区时仍会出错
- 不适合多时区服务

### 中期（标准化）

**实现Context/TimeContext系统**：

```rust
pub struct TimeContext {
    pub reference: DateTime<Tz>,  // 用户提供的参考时间
    pub timezone: Tz,              // 用户时区
}

pub fn rules(b: &RuleSetBuilder<Value>, ctx: Option<&TimeContext>) {
    let reference = ctx
        .map(|c| c.reference)
        .unwrap_or_else(|| Local::now().into());

    // 使用reference计算
}
```

**优点**：
- 符合Duckling标准
- 支持多时区
- 用户可控

**缺点**：
- 需要修改API签名
- 调用者需要提供时区信息

### 长期（完整方案）

**完全遵循Duckling设计**：

1. 实现`ZoneSeriesTime`处理夏令时
2. 支持时区数据库（IANA tz database）
3. 返回`ZonedTime`而不是纯UTC

---

## 🎓 总结

### 回答你的问题

> "返回UTC而不是当地时间，不会很奇怪吗？"

**回答**：

1. **返回UTC不奇怪** - `DateTime<Utc>`只是一种表示方式
   - 可以轻松转换到任何时区
   - 表示的是全球唯一的时间点

2. **基于Utc::now()计算很奇怪** - 应该基于用户时区
   - 用户说"明天"指的是他们时区的明天
   - 不是UTC的明天

3. **理想流程**：
   ```
   用户本地时间 → 计算相对时间 → 转UTC存储 → 按需转回任意时区显示
   ```

### 当前状态

- ✅ 测试通过（因为测试和实现都用UTC，保持一致）
- ❌ 实际使用可能有问题（如果用户不在UTC时区）
- ⚠️ 特别是日期相关的计算（"明天"、"昨天"）

### 建议

**立即行动**：
- 暂时可以保持现状（继续Phase 5）
- 在文档中注明当前限制

**Phase 5后重构**：
- 改用`Local::now()`作为默认参考时间
- 或者实现完整的`TimeContext`系统

你想现在修复，还是继续Phase 5？🤔

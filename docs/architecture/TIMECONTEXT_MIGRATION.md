# TimeContext迁移记录

**日期**: 2026-02-20
**任务**: 完整重构ZH Time规则以使用TimeContext
**结果**: ✅ 成功 - 86条规则全部迁移，110/110测试通过

---

## 🎯 目标

将所有ZH Time规则从直接使用`Utc::now()`改为使用`TimeContext`，确保：
1. 相对时间计算基于用户的本地时区（不是UTC）
2. 返回值标准化为UTC（符合Duckling设计）
3. 支持测试时注入特定参考时间

---

## 📋 关键问题与解决方案

### 问题1: Rust所有权 + Move闭包冲突

**错误模式**：
```rust
// ❌ 错误：第一个闭包消费ctx后，第二个无法使用
fn rules(ctx: Arc<TimeContext>) {
    b.rule_1_terminal("rule1", regex, move |_| {
        ctx.reference_local()  // ctx被move
    });

    b.rule_1_terminal("rule2", regex, move |_| {
        ctx.reference_local()  // ❌ 错误：ctx已被moved
    });
}
```

**正确模式**：
```rust
// ✅ 正确：为每个规则在闭包外clone
fn rules(ctx: Arc<TimeContext>) {
    let ctx_rule1 = Arc::clone(&ctx);  // 闭包外clone
    b.rule_1_terminal("rule1", regex, move |_| {
        ctx_rule1.reference_local()  // 闭包拥有ctx_rule1
    });

    let ctx_rule2 = Arc::clone(&ctx);  // 再clone一次
    b.rule_1_terminal("rule2", regex, move |_| {
        ctx_rule2.reference_local()
    });
}
```

**原因**：
- `move`闭包要求**拥有**捕获的变量（用于'static生命周期）
- `Arc::clone()`创建新的引用计数指针（开销极小，只是原子递增）
- 每个闭包需要独立的Arc副本

---

### 问题2: 闭包内clone vs 闭包外clone

**尝试过的错误方案**：
```rust
// ❌ 错误：闭包想借用ctx但move要求所有权
b.rule_1_terminal("rule", regex, move |_| {
    let local_ref = ctx.clone().reference_local();
    //              ^^^ 需要&ctx（借用），但move要求所有权
});
```

**解决方案**：
```rust
// ✅ 必须在闭包外clone
let ctx_rule = Arc::clone(&ctx);
b.rule_1_terminal("rule", regex, move |_| {
    let local_ref = ctx_rule.reference_local();
});
```

---

### 问题3: Helper函数的ctx传递

**原始签名**：
```rust
fn add_hour_minute_rules(b: &RuleSetBuilder<Value>) {
    // 规则使用Utc::now()
}
```

**更新后**：
```rust
fn add_hour_minute_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // 每个规则clone ctx
    let ctx_rule1 = Arc::clone(&ctx);
    b.rule_1_terminal(..., move |_| {
        ctx_rule1.reference_local()
    });
}

// 调用处
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));
    add_hour_minute_rules(b, Arc::clone(&ctx));
}
```

---

### 问题4: 类型不匹配 DateTime<FixedOffset> vs DateTime<Utc>

**错误**：
```rust
let local_ref = ctx.reference_local();  // DateTime<FixedOffset>
let dt = local_ref.with_minute(15).unwrap();  // 仍是FixedOffset
TimeData::new(dt, Grain::Minute);  // ❌ 需要DateTime<Utc>
```

**解决**：
```rust
let local_ref = ctx.reference_local();
let dt = local_ref.with_minute(15).unwrap()
    .with_timezone(&Utc);  // 转换为UTC
TimeData::new(dt, Grain::Minute);  // ✅
```

---

### 问题5: 循环中动态创建规则

**场景**：DOW × Part of Day组合规则
```rust
for (dow_pat, weekday) in &dow_patterns {
    for (pod_pat, hour) in &pod_patterns {
        // ❌ 错误：所有闭包共享同一个ctx，会被move
        b.rule_1_terminal(&rule_name, regex, move |_| {
            ctx.reference_local()  // 第一次循环后ctx已被moved
        });
    }
}
```

**解决**：
```rust
for (dow_pat, weekday) in &dow_patterns {
    for (pod_pat, hour) in &pod_patterns {
        let weekday_copy = *weekday;
        let hour_copy = *hour;
        let ctx_rule = Arc::clone(&ctx);  // 每次循环都clone

        b.rule_1_terminal(&rule_name, regex, move |_| {
            ctx_rule.reference_local()
        });
    }
}
```

---

### 问题6: 批量替换的陷阱

**经验教训**：

1. **sed批量替换需谨慎**
   - ❌ `sed 's/|_|/move |_|/g'` → 产生 `move move |_|`
   - ✅ 使用Python脚本进行复杂替换

2. **变量名生成**
   - Python脚本自动从规则名生成ctx变量名
   - 需要处理特殊字符（`:`, `-` → `_`）

3. **作用域问题**
   - 批量替换可能创建错误的变量名
   - 需要手动验证每个helper函数的规则

---

## 📊 迁移统计

| 模块 | 规则数 | 更新方式 | 状态 |
|------|--------|----------|------|
| 主函数 - 简单规则 | 13 | 手动 | ✅ |
| 主函数 - 周/月/年 | 6 | 手动 | ✅ |
| 主函数 - 命名星期 | 7 | 手动 | ✅ |
| 主函数 - Weekend | 3 | 手动 | ✅ |
| Phase 4 - Duration | 6 | 手动 | ✅ |
| Phase 4 - Interval | 7 | 手动 | ✅ |
| Helper - Part of Day | 6 | Python脚本 + 手动 | ✅ |
| Helper - Hour/Minute | 13 | Python脚本 + 手动 | ✅ |
| Helper - Month/Day | 8 | Python脚本 + 手动 | ✅ |
| Helper - Year | 5 | Python脚本 + 手动 | ✅ |
| Helper - Intersection | 12 | Python脚本 + 手动 | ✅ |
| **总计** | **86** | **混合** | **✅ 100%** |

---

## ✅ 验证结果

```bash
cargo test --test zh_time_*

Phase 2: 25/25 passed ✅
Phase 3: 21/21 passed ✅
Phase 4: 25/25 passed ✅
Variants: 22/22 passed ✅
Edge Cases: 17/17 passed ✅
━━━━━━━━━━━━━━━━━━━━━━━━━━━
总计: 110/110 tests passing (100%)
```

---

## 🎓 最佳实践总结

### Arc::clone模式（核心）

```rust
// 1. 函数签名
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));

    // 2. 为每个规则clone（在闭包外）
    let ctx_rule = Arc::clone(&ctx);

    // 3. move闭包捕获独立副本
    b.rule_1_terminal("rule", regex, move |_| {
        // 4a. 日期操作用local
        let local_ref = ctx_rule.reference_local();
        let tomorrow = local_ref + Duration::days(1);

        // 4b. 时间操作可用UTC
        let target = ctx_rule.reference_utc() + Duration::hours(3);

        // 5. 转回UTC返回
        let dt_utc = result.with_timezone(&Utc);
        Ok(TimeValue::instant(dt_utc, grain))
    });
}
```

### Helper函数模式

```rust
fn add_helper_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    //                                            ^^^^^^^^^^^^^^^^ 接受Arc

    let ctx_rule1 = Arc::clone(&ctx);  // 为每个规则clone
    b.rule_1_terminal(..., move |_| { ... });

    let ctx_rule2 = Arc::clone(&ctx);  // 再clone
    b.rule_1_terminal(..., move |_| { ... });
}

// 调用时也clone
add_helper_rules(b, Arc::clone(&ctx));
```

### 循环中的规则

```rust
for pattern in &patterns {
    let ctx_dynamic = Arc::clone(&ctx);  // ← 循环内clone
    let pattern_copy = *pattern;

    b.rule_1_terminal(&rule_name, regex, move |_| {
        ctx_dynamic.reference_local()
    });
}
```

---

## 🔍 调试技巧

1. **编译错误优先级**
   - 先修复"cannot find value"（变量未定义）
   - 再修复"use of moved value"（所有权问题）
   - 最后修复类型不匹配

2. **批量操作验证**
   - 每次批量替换后立即编译
   - 不要累积多个脚本修改

3. **测试驱动**
   - Phase 4规则完全更新后先测试
   - 确认pattern正确再扩展到其他规则

---

## 🚀 性能影响

- **Arc::clone()开销**：原子递增引用计数（~1-2 CPU周期）
- **86个规则 × Arc::clone()**：总开销 < 200纳秒
- **可忽略不计** vs 正则匹配和时间计算

---

## 📝 未来改进

1. **类型系统改进**
   - 考虑使用泛型`TimeContext<Tz>`支持任意时区
   - 或使用trait object避免Arc clone

2. **性能优化**
   - 如果性能敏感，可考虑`&'static TimeContext`
   - 但当前开销已经可忽略

3. **API简化**
   - 可以添加builder pattern简化规则定义
   - 自动处理ctx clone

---

**结论**: TimeContext迁移成功完成，所有测试通过，时区处理现已正确！ ✅

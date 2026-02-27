# Phase 2: Time 维度迁移规划

**项目**: Duckling → Rustling Time 维度
**分支**: phase2-time-implementation
**创建时间**: 2025-02-15
**预计时间**: 1-2 周

---

## 目标概述

### 主要目标
将 Duckling 的 Time 维度迁移到 Rustling，支持时间解析功能。

### 复杂度评估
| 维度 | Numeral | Time | 复杂度比 |
|------|---------|------|---------|
| 语言数 | 48 | 46 | 0.96x |
| 代码行数 | ~300/lang | ~1500/lang | 5x |
| 核心类型 | 简单 | 复杂 | 10x |
| 辅助函数 | 少 | 多 | 8x |
| 规则类型 | 3 种 | 6+ 种 | 2x |

**结论**: Time 维度比 Numeral **复杂 5-10 倍**

---

## 初步分析

### 代码规模

**Haskell 源码统计**:
```bash
EN Time Rules: 2,841 lines
ZH Time Rules: 1,359 lines
ES Time Rules: 1,672 lines

核心文件:
- Computed.hs: 46,269 bytes (计算逻辑)
- Types.hs: 30,046 bytes (类型定义)
- Helpers.hs: 25,905 bytes (辅助函数)
- HolidayHelpers.hs: 1,518 bytes (节假日)
```

### 关键挑战

#### 1. 复杂类型系统
```haskell
data TimeData = TimeData {
  form :: Form,           -- 时间形式
  grain :: Grain,         -- 粒度 (second, minute, hour, day, week, month, year)
  datetime :: DateTime,   -- 日期时间
  timezone :: Maybe Timezone,
  latent :: Bool,         -- 是否潜在（需要上下文）
  holiday :: Maybe Text   -- 节假日
}
```

#### 2. 时间粒度 (Grain)
```haskell
data Grain = Second | Minute | Hour | Day | Week | Month | Quarter | Year
```

**挑战**: 不同粒度的时间计算和转换

#### 3. 相对时间
```
"明天" → today + 1 day
"下周" → next week
"上个月" → last month
"三天后" → today + 3 days
```

**挑战**: 需要当前时间作为参考点

#### 4. 时间区间
```
"从周一到周五"
"2月1日至3月31日"
"早上9点到下午5点"
```

**挑战**: 需要解析两个时间点并组合

#### 5. 时区处理
```
"北京时间下午3点"
"UTC+8 15:00"
"EST 3 PM"
```

**挑战**: 时区转换和标准化

#### 6. 节假日
```
"圣诞节"
"春节"
"感恩节"
"国庆节"
```

**挑战**: 各国节假日不同，计算逻辑复杂

---

## 技术方案

### 方案 1: 完全自动化 (理想但困难)

**优点**:
- 复用 Phase 1 的工具链
- 快速批量生成

**缺点**:
- Time 规则太复杂，组合规则占比高 (60%+)
- 时间计算逻辑难以自动转换
- 类型系统差异大

**预计自动化率**: 40-50%

### 方案 2: 核心手工 + 辅助自动化 (推荐)

**策略**:
1. **手工实现核心类型和函数**
   - TimeData 类型
   - Grain 枚举
   - 时间计算函数 (intersect, shift, etc.)

2. **自动生成简单规则**
   - 字典规则 (月份、星期、节假日名称)
   - 简单正则规则 (日期格式)

3. **手工实现复杂规则**
   - 组合规则 (相对时间、区间)
   - 上下文依赖规则

**预计自动化率**: 30-40%

### 方案 3: 渐进式迁移 (最稳妥)

**阶段划分**:

#### Phase 2.1: 核心基础设施 (3 天)
- [ ] 实现 Rust Time 类型系统
- [ ] 实现核心时间计算函数
- [ ] 时区支持 (使用 chrono crate)

#### Phase 2.2: 英语试点 (2 天)
- [ ] EN 语言的简单规则 (字典 + 正则)
- [ ] EN 语言的组合规则 (手工)
- [ ] Corpus 测试验证

#### Phase 2.3: 中文扩展 (2 天)
- [ ] ZH 语言规则
- [ ] 农历支持 (可选)
- [ ] Corpus 测试

#### Phase 2.4: 批量生成 (3 天)
- [ ] 为其余 44 种语言生成代码
- [ ] 修复编译错误
- [ ] 基础测试

#### Phase 2.5: 完善与优化 (2 天)
- [ ] 补充手动规则
- [ ] 性能优化
- [ ] 完整 Corpus 测试

**总计**: 12 天 (约 2 周)

---

## 推荐方案: 方案 3 (渐进式)

### 理由

1. **风险可控**: 每个阶段独立验证
2. **学习曲线**: 从简单到复杂逐步理解
3. **质量保证**: EN/ZH 试点保证核心功能
4. **可调整**: 根据实际情况调整后续计划

---

## Phase 2.1 详细计划

### 目标
建立 Time 维度的核心基础设施

### 任务清单

#### 1. Rust 类型系统 (1 天)

**创建文件**: `core/src/time_types.rs`

```rust
pub enum Grain {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

pub struct TimeData {
    pub datetime: DateTime<Tz>,
    pub grain: Grain,
    pub latent: bool,
    pub holiday: Option<String>,
}

pub enum TimeValue {
    Instant(TimeData),
    Interval { from: TimeData, to: TimeData },
}
```

**依赖**:
- `chrono` crate (时间处理)
- `chrono-tz` crate (时区)

#### 2. 时间计算函数 (1 天)

**创建文件**: `core/src/time_helpers.rs`

**关键函数**:
```rust
// 时间交集
pub fn intersect(td1: &TimeData, td2: &TimeData) -> Option<TimeData>;

// 时间偏移
pub fn shift(td: &TimeData, direction: Direction, grain: Grain, n: i32) -> TimeData;

// 时间序列
pub fn sequence(tds: Vec<TimeData>) -> Option<TimeData>;

// 粒度转换
pub fn coarser_grain(g1: Grain, g2: Grain) -> Grain;
```

#### 3. 时区支持 (0.5 天)

**创建文件**: `core/src/timezone.rs`

**功能**:
- 时区解析 ("UTC+8", "EST", "PST")
- 时区转换
- 默认时区设置

#### 4. 测试验证 (0.5 天)

**创建文件**: `core/tests/time_helpers_test.rs`

**测试用例**:
- intersect: "2024年" ∩ "2月" = "2024年2月"
- shift: "今天" + 1 day = "明天"
- sequence: ["周一", "周二", "周三"] → 连续三天

---

## Phase 2.2 详细计划

### 目标
英语 Time 规则试点实现

### 任务清单

#### 1. 提取 EN Time 规则 (0.5 天)

```bash
python3 tools/migration/extract_rules.py \
  ~/Project/duckling/Duckling/Time/EN/Rules.hs \
  --output extracted/time/en.json
```

**预期**:
- 字典规则: ~20 个 (月份、星期、节假日)
- 正则规则: ~40 个 (日期格式)
- 组合规则: ~60 个 (需手动实现)

#### 2. 生成简单规则 (0.5 天)

**创建模板**: `templates/time_rules.rs.tera`

**生成代码**: `languages/en/time.rs`

#### 3. 手动实现组合规则 (1 天)

**关键规则**:
- `ruleRelativeTime`: "明天", "昨天", "今天"
- `ruleInterval`: "从...到..."
- `ruleLastNext`: "下周", "上个月"
- `ruleIntersect`: "2月15日", "周一下午"

#### 4. Corpus 测试 (0.5 天)

**转换测试**: `Duckling/Time/EN/Corpus.yml` → Rust tests

**目标通过率**: ≥60%

---

## Phase 2.3-2.5 概要

### Phase 2.3: 中文扩展
- 复用 EN 的核心逻辑
- 处理中文特殊语法 ("后天", "大后天")
- 农历支持 (可选，Phase 3)

### Phase 2.4: 批量生成
- 为其余 44 种语言生成代码
- 预计编译成功率: 50-60%
- 修复常见错误

### Phase 2.5: 完善优化
- 补充手动规则
- 性能基准测试
- 完整 Corpus 测试 (目标 70%+)

---

## 风险与缓解

### 主要风险

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|----------|
| 类型系统不匹配 | 高 | 高 | 提前设计，参考 chrono |
| 组合规则复杂 | 高 | 中 | EN/ZH 试点充分测试 |
| 时区处理困难 | 中 | 中 | 使用成熟的 chrono-tz |
| 农历计算复杂 | 中 | 低 | Phase 3 再考虑 |
| Corpus 通过率低 | 中 | 中 | 设定合理目标 (60-70%) |

### 应对策略

1. **降低预期**: Time 维度不追求 100% 通过率
2. **分阶段验证**: 每个阶段独立测试
3. **快速迭代**: 遇到阻塞及时调整方案
4. **文档先行**: 详细记录设计决策

---

## 成功标准

### Phase 2 完成标准

**必须达成**:
- ✅ 核心类型系统实现
- ✅ EN 和 ZH 语言基本可用
- ✅ 30+ 种语言编译成功
- ✅ Corpus 测试通过率 ≥60%

**期望达成**:
- ✅ 40+ 种语言编译成功
- ✅ EN/ZH Corpus 通过率 ≥70%
- ✅ 时区基本支持

**可选**:
- 农历支持
- 节假日完整实现
- 性能优化

---

## 后续工作

### Phase 3: Duration 维度
- 相对简单 ("3小时", "两周")
- 预计 3-5 天

### Phase 4: 其他维度
- AmountOfMoney
- Distance, Volume, Temperature
- 预计各 2-3 天

---

## 开始行动

### 立即开始

**第一步**: 实现核心类型系统
```bash
# 创建文件结构
mkdir -p core/src/time
touch core/src/time/mod.rs
touch core/src/time/types.rs
touch core/src/time/helpers.rs
touch core/src/time/grain.rs
```

**第二步**: 添加依赖
```toml
# Cargo.toml
[dependencies]
chrono = "0.4"
chrono-tz = "0.8"
```

**第三步**: 实现基础类型
- Grain 枚举
- TimeData 结构体
- TimeValue 枚举

---

**Phase 2 开始！预计完成时间: 2 周** 🚀

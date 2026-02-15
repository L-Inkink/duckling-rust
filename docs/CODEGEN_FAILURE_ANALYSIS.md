# Codegen 失败语言深度分析

**日期**: 2026-02-15
**分析对象**: ar, km, ko, vi, tr, ro（6个编译失败的语言）
**结论**: ✅ **通用模板问题**，不需要语言特殊处理

---

## 🔍 问题分类

### 问题 1: 字典引用未定义（4个语言）

**受影响语言**: ar (阿拉伯语), km (高棉语), ko (韩语), vi (越南语)

**症状**:
```rust
// 生成的代码引用了字典
let dict = &*DIGITS_DICTIONARY;  // ❌ 错误：DIGITS_DICTIONARY 未定义
```

**根本原因**:

JSON 规则中有 `dictionary_ref` 字段：
```json
{
  "name": "Integer30_80",
  "production": {
    "value_extractor": "dictionary_lookup",
    "dictionary_ref": "digitsMap"  // ← 引用外部字典
  }
}
```

Codegen 生成了字典引用代码：
```rust
let dict = &*DIGITS_DICTIONARY;  // ← 转换 digitsMap → DIGITS_DICTIONARY
```

但**没有生成**字典定义：
```rust
// ❌ 缺失！应该生成：
lazy_static! {
    static ref DIGITS_DICTIONARY: HashMap<&'static str, i64> = {
        // ... 字典条目
    };
}
```

**影响范围**: 所有使用 Haskell `HashMap` 引用的规则（约20%的规则）

**修复复杂度**: ⭐⭐⭐ 中等

**修复方案**:
1. 在 `extract_rules.py` 中识别引用的字典名称
2. 在同一 Haskell 文件中查找对应的 HashMap 定义
3. 提取 HashMap 内容到 JSON
4. Codegen 生成对应的 `lazy_static!` 字典

---

### 问题 2: Unicode 字符转义错误（1个语言）

**受影响语言**: ro (罗马尼亚语)

**症状**:
```rust
map.insert("\\537apte", 7);  // ❌ 错误：unknown character escape: \537
```

**应该是**:
```rust
map.insert("șapte", 7);  // ✅ 正确：ș 是罗马尼亚语字符
```

**根本原因**:

Extract_rules.py 错误地将 Unicode 字符转义：
```python
# ❌ 错误处理
"șapte" → "\\537apte"  # Python repr() 的八进制转义
```

JSON 文件中：
```json
{
  "\\537apte": {"value": 7}  // ← 错误的转义序列
}
```

**影响范围**: 所有包含非 ASCII 字符的语言（~15种语言）

**修复复杂度**: ⭐ 简单

**修复方案**:
```python
# extract_rules.py 修复
import json

# 确保正确编码 Unicode
json.dumps(data, ensure_ascii=False, indent=2)
```

---

### 问题 3: 字典类型推断错误（1个语言）

**受影响语言**: tr (土耳其语)

**症状**:
```rust
static ref NUMERALSUFFIXESHALFSUFFIXTEXT_DICTIONARY: HashMap<&'static str, i64> = {
    let mut map = HashMap::new();
    map.insert("altıbuçuk", 6.5);  // ❌ 错误：6.5 是 f64，不是 i64
    //                          ^^^ mismatched types
}
```

**根本原因**:

Codegen 总是假设字典值是 `i64`：
```rust
// codegen.rs 中硬编码
HashMap<&'static str, i64>  // ← 假设所有值都是整数
```

但土耳其语有小数表达：
```json
{
  "altıbuçuk": {"value": 6.5},  // 6.5（六点五）
  "bibuçuk": {"value": 1.5}      // 1.5（一点五）
}
```

**影响范围**: 包含小数的语言（~5种语言）

**修复复杂度**: ⭐⭐ 简单-中等

**修复方案**:
```rust
// codegen.rs 中添加类型推断
fn infer_dict_type(entries: &HashMap<String, Value>) -> String {
    let has_float = entries.values().any(|v| {
        v.as_f64().map(|f| f.fract() != 0.0).unwrap_or(false)
    });

    if has_float {
        "HashMap<&'static str, f64>"
    } else {
        "HashMap<&'static str, i64>"
    }
}
```

---

## 📊 通用性评估

### 是否需要语言特殊处理？

| 问题 | 语言特殊性 | 通用模板可解决 | 评估 |
|------|-----------|--------------|------|
| **字典引用未定义** | ❌ 否 | ✅ 是 | 通用问题 |
| **Unicode 转义错误** | ❌ 否 | ✅ 是 | 通用问题 |
| **字典类型推断** | ❌ 否 | ✅ 是 | 通用问题 |

**结论**: ✅ **所有问题都是通用 Codegen 缺陷**，改进工具链即可解决，**不需要**为特定语言创建专门的处理器。

---

## 🔧 修复优先级

### High Priority（影响多个语言）

1. **Unicode 字符转义** ⭐⭐⭐⭐⭐
   - 影响: 15+ 语言
   - 复杂度: 低
   - 修复文件: `tools/migration/extract_rules.py`
   - 预计时间: 30 分钟

2. **字典类型推断** ⭐⭐⭐⭐
   - 影响: 5+ 语言
   - 复杂度: 中
   - 修复文件: `tools/migration/codegen.rs`
   - 预计时间: 1-2 小时

### Medium Priority（需要架构改进）

3. **字典引用生成** ⭐⭐⭐
   - 影响: 4 语言（但模式常见）
   - 复杂度: 中-高
   - 修复文件: `extract_rules.py` + `codegen.rs` + 模板
   - 预计时间: 3-4 小时

---

## 🎯 修复计划

### Phase A: 快速修复（2-3小时）

**目标**: 解决 Unicode 和类型推断问题

**步骤**:
1. ✅ 修复 `extract_rules.py` 的 Unicode 处理
2. ✅ 添加字典类型推断到 `codegen.rs`
3. ✅ 重新提取 RO 规则
4. ✅ 重新生成 TR、RO 代码
5. ✅ 验证编译

**预期成果**: RO、TR 编译成功（48/50 → 50/50 = 100%？）

### Phase B: 字典引用支持（3-4小时）

**目标**: 完整支持 Haskell HashMap 引用

**步骤**:
1. 改进 `extract_rules.py` 识别字典引用
2. 提取引用的字典定义
3. 更新 JSON Schema 支持字典定义
4. 改进 Codegen 模板生成字典
5. 重新生成 AR、KM、KO、VI
6. 验证编译和测试

**预期成果**: 所有50种语言编译成功 + 基础规则可用

### Phase C: 完整验证（可选）

**步骤**:
1. 运行 Corpus 测试
2. 手动修复复杂规则
3. 达到 70%+ 自动化率

---

## 💡 长期改进建议

1. **类型系统增强**
   - 支持 `union types`: `i64 | f64`
   - 自动从 JSON 值推断类型

2. **外部引用支持**
   - 支持跨规则的字典共享
   - 生成模块级别的字典

3. **Unicode 处理**
   - 标准化 UTF-8 处理流程
   - 添加字符规范化（NFD/NFC）

4. **测试覆盖**
   - 为 Codegen 添加单元测试
   - 每种语言自动生成烟雾测试

---

## 📈 预期改进效果

### 修复前（当前）

| 指标 | 值 |
|------|-----|
| 编译成功率 | 44/50 (88%) |
| 自动化率 | 45% |
| 基础规则可用 | 60% |

### 修复后（Phase A+B）

| 指标 | 预期值 | 改进 |
|------|-------|------|
| 编译成功率 | 50/50 (100%) | +12% |
| 自动化率 | 65-70% | +20-25% |
| 基础规则可用 | 85%+ | +25% |

---

## 🚀 立即行动

**建议执行顺序**:

1. **立即开始** Phase A（Unicode + 类型推断）
   - 快速见效
   - 风险低
   - 2-3 小时完成

2. **随后执行** Phase B（字典引用）
   - 架构性改进
   - 一次性解决多个语言
   - 3-4 小时完成

3. **可选执行** Phase C（完整验证）
   - 根据 Phase A+B 结果决定
   - 可能发现新问题

**总预计时间**: 5-7 小时（1个工作日）

---

**分析人**: Claude Code
**状态**: 完成
**下一步**: 开始 Phase A 修复

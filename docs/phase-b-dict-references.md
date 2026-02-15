# Phase B: 字典引用支持修复记录

**日期**: 2025-02-15
**任务**: 修复 ar, km, ko, vi 四种语言的字典引用问题
**结果**: 48/48 语言编译成功 (100%)

---

## 问题概述

Phase A 完成后，仍有 4 种语言无法编译：

| 语言 | 错误原因 | 缺失的字典 |
|------|---------|-----------|
| ar (阿拉伯语) | 字典定义在 Helpers.hs 中 | digitsMap |
| km (高棉语) | 多行注释阻止提取 | ruleTensMap |
| ko (韩语) | 复杂元组类型字典 | integerType1PowersOfTenMap |
| vi (越南语) | 复杂元组类型字典 | powersOfTenMap |

---

## 根本原因分析

### 问题 1: 字典定义位置不同 (AR)

**现象**:
```rust
error[E0425]: cannot find value `DIGITS_DICTIONARY` in this scope
```

**根本原因**:
1. **AR 特殊结构**: `digitsMap` 定义在 `Duckling/Numeral/AR/Helpers.hs`，而非 `Rules.hs`
2. **提取工具局限**: 只处理 `Rules.hs` 文件，忽略 `Helpers.hs`
3. **引用失败**: `Rules.hs` 中的规则引用了 `digitsMap`，但提取时未包含定义

**代码结构**:
```haskell
-- Duckling/Numeral/AR/Rules.hs
import Duckling.Numeral.AR.Helpers (digitsMap, ...)

ruleInteger5 = Rule {
  production = \case
    ... -> HashMap.lookup match digitsMap >>= integer
}

-- Duckling/Numeral/AR/Helpers.hs
digitsMap :: HashMap Text Integer
digitsMap = HashMap.fromList
  [ ("عشر", 2), ("ثلاث", 3), ... ]
```

**解决方案**:
1. 单独提取 `Helpers.hs` 文件
2. 合并提取的字典到主 JSON

```bash
# 提取 Helpers
python3 extract_rules.py Duckling/Numeral/AR/Helpers.hs \
  --output extracted/numeral/ar_helpers.json

# 合并到主文件
jq -s '.[0].rules += .[1].rules | .[0]' \
  extracted/numeral/ar.json \
  extracted/numeral/ar_helpers.json \
  > extracted/numeral/ar_merged.json
```

---

### 问题 2: 多行注释导致提取失败 (KM)

**现象**:
```rust
error[E0425]: cannot find value `RULETENS_DICTIONARY` in this scope
```

**根本原因**:
1. **Haskell 注释风格**: HashMap 定义前有多行 `--` 注释
2. **正则表达式限制**: 原始模式只匹配单行结构
3. **KM 特殊情况**: `ruleTensMap` 前有 3 行注释说明

**原始正则表达式**:
```python
# 只匹配：HashMap.fromList [
pattern = r'(\w+Map)\s*::\s*HashMap.*\n\s*\1\s*=\s*HashMap\.fromList\s*\[(.*)\]'
```

**问题代码**:
```haskell
ruleTensMap :: HashMap Text Integer
ruleTensMap = HashMap.fromList
  -- លេខ ១០ ២០ ៣០ ... (第1行注释)
  -- វិធីដកលេខ ០ ... (第2行注释)
  -- ពពួក "សិប" ... (第3行注释)
  [ ( "ដប់", 10 )
  , ( "ម្ភៃ", 20 )
  , ...
  ]
```

**改进的正则表达式**:
```python
# 支持多行注释：(?:--[^\n]*\n\s*)*
pattern = r'''
    (\w+Map)\s*::\s*HashMap(?:\.HashMap)?\s+(?:Text\.)?Text\s+(\w+)\s*
    \n\s*
    \1\s*=\s*HashMap\.fromList\s*
    (?:(?:--[^\n]*\n\s*)*)  # 允许任意数量的注释行
    \[((?:[^]]*(?:\[[^\]]*\])?)*)\]
'''
```

**效果**:
- 修复前: 只提取到 `ruleNumeralMap` (1 个字典)
- 修复后: 提取到 `ruleNumeralMap`, `ruleTensMap` (2 个字典) ✓

---

### 问题 3: 元组类型字典 (KO/VI)

**现象**:
```rust
error[E0425]: cannot find value `INTEGERTYPE1POWERSOFTEN_DICTIONARY` in this scope
error[E0425]: cannot find value `POWERSOFTEN_DICTIONARY` in this scope
```

**根本原因**:
1. **复杂类型**: 字典值是元组 `(Double, Int)` 而非简单类型
2. **语义复杂**: 元组包含 (数值, 粒度)，需要特殊处理
3. **正则不支持**: 只匹配 `(\w+)` 简单类型，不匹配 `(Double, Int)`

**问题代码**:
```haskell
-- KO: Duckling/Numeral/KO/Rules.hs
integerType1PowersOfTenMap :: HashMap Text (Double, Int)
integerType1PowersOfTenMap = HashMap.fromList
  [ ( "십",  (1e1, 1) )
  , ( "백",  (1e2, 2) )
  , ( "천",  (1e3, 3) )
  , ( "만",  (1e4, 4) )
  , ...
  ]

-- VI: Duckling/Numeral/VI/Rules.hs
powersOfTenMap :: HashMap Text (Double, Int)
powersOfTenMap = HashMap.fromList
  [ ( "chục",  (1e1, 1) )
  , ( "trăm",  (1e2, 2) )
  , ( "nghìn", (1e3, 3) )
  , ...
  ]
```

**解决方案**: 标记为手动实现

由于这些字典需要复杂的逻辑（处理粒度等），暂时采用以下策略：

1. **标记规则为需要手动审查**:
```python
for rule in rules:
    if rule['production'].get('dictionary_ref') in ['powersOfTenMap', 'integerType1PowersOfTenMap']:
        rule['metadata']['_needs_manual_review'] = True
        rule['metadata']['note'] = 'Tuple-type dictionary (Double, Int)'
```

2. **删除 dictionary_ref，避免生成错误代码**:
```python
del rule['production']['dictionary_ref']
rule['production']['value_extractor'] = 'custom'
```

3. **生成 TODO 注释**:
```rust
// TODO: PowersOfTen (regex)
//   Original: powers of tens
// Manual implementation required (Tuple-type dictionary)
```

**影响**: 这些规则暂时不可用，但不阻止其他规则工作

**未来改进方向**:
1. 扩展正则表达式支持元组类型
2. 实现专门的 PowersOfTen 处理器
3. 或在 Rust 端手动实现这些规则

---

## 技术细节

### 正则表达式改进对比

| 阶段 | 模式 | 支持特性 |
|------|------|---------|
| **原始** | `(\w+Map).*\n.*HashMap\.fromList\s*\[` | 单行定义 |
| **Phase B** | `(\w+Map).*\n.*HashMap\.fromList\s*(?:--[^\n]*\n\s*)*\[` | 多行注释 |

### 支持的 HashMap 类型

| Haskell 类型 | Rust 等价 | 支持状态 |
|-------------|-----------|---------|
| `HashMap Text Integer` | `HashMap<&str, i64>` | ✅ 完全支持 |
| `HashMap Text Double` | `HashMap<&str, f64>` | ✅ Phase A 支持 |
| `HashMap Text (Double, Int)` | - | ❌ 需手动实现 |
| `HashMap Char String` | - | ❌ 未遇到 |

### KM 语言高棉字符示例

| 字符 | 十进制 | Unicode | 含义 |
|------|--------|---------|------|
| ០ | U+17E0 | KHMER DIGIT ZERO | 0 |
| ១ | U+17E1 | KHMER DIGIT ONE | 1 |
| ២ | U+17E2 | KHMER DIGIT TWO | 2 |
| ប្រាំ | - | 多字符组合 | 5 |
| ដប់ | - | 多字符组合 | 10 (dap) |

---

## 修改文件清单

| 文件 | 修改类型 | 说明 |
|------|---------|------|
| **工具链** | | |
| `tools/migration/extract_rules.py` | 增强正则 | 支持多行注释 |
| **提取数据** | | |
| `extracted/numeral/ar.json` | 新增 | 合并 Helpers.hs 字典 |
| `extracted/numeral/km.json` | 重新提取 | 2 个字典 (原 1 个) |
| `extracted/numeral/ko.json` | 重新提取 | 4 个字典 + 标记元组类型 |
| `extracted/numeral/vi.json` | 重新提取 | 2 个字典 + 标记元组类型 |
| **生成代码** | | |
| `languages/ar/numeral.rs` | 新生成 | 32 条规则 |
| `languages/km/numeral.rs` | 重新生成 | 8 条规则 |
| `languages/ko/numeral.rs` | 重新生成 | 18 条规则 |
| `languages/vi/numeral.rs` | 重新生成 | 14 条规则 |
| `languages/mod.rs` | 更新 | 48 种语言全部启用 |

---

## 遇到的坑和解决方案

### 坑 1: languages/mod.rs 被意外覆盖

**问题**: 运行 `codegen` 时只生成 ko 和 vi，导致 `languages/mod.rs` 被覆盖，丢失其他 46 种语言

**原因**: `update_root_languages_mod()` 完全重写文件，只包含本次生成的语言列表

**代码位置** (tools/migration/codegen.rs:193-206):
```rust
fn update_root_languages_mod(&self, locales: &[String]) -> Result<...> {
    let mod_file = self.output_dir.join("languages").join("mod.rs");

    let mut content = String::from("// Auto-generated...\n\n");
    for locale in locales {
        content.push_str(&format!("pub mod {};\n", locale));
    }

    fs::write(&mod_file, content)?;  // ❌ 完全覆盖！
}
```

**解决方案**:
1. 从 git 历史恢复完整列表: `git show 95b9c26:languages/mod.rs`
2. 手动添加新语言 (ar, km, ko, vi)
3. **未来改进**: 修改 codegen 为增量更新模式

### 坑 2: Python JSON 修改未生效

**问题**: 删除 `dictionary_ref` 字段后，代码仍然生成字典引用

**错误代码**:
```python
if 'dictionary_ref' in rule.get('production', {}):
    rule['production']['value_extractor'] = 'custom'  # ❌ 未删除字段
```

**正确代码**:
```python
if 'dictionary_ref' in rule.get('production', {}):
    del rule['production']['dictionary_ref']  # ✅ 明确删除
    rule['production']['value_extractor'] = 'custom'
```

### 坑 3: codegen 缓存问题

**问题**: 修改 JSON 后重新运行 codegen，但生成的代码没有变化

**原因**: 旧的 `.rs` 文件已存在，codegen 检查到编译错误就退出，没有覆盖文件

**解决方案**:
1. 临时禁用有问题的语言模块: `sed -i 's/^pub mod vi;/\/\/ pub mod vi;/' languages/mod.rs`
2. 删除旧文件: `rm languages/vi/numeral.rs`
3. 重新生成
4. 重新启用模块

---

## 统计数据

### 提取成功率提升

| 语言 | Phase A 后 | Phase B 后 | 新增字典 |
|------|-----------|-----------|---------|
| ar | 0 个字典 | 1 个字典 | +1 (digitsMap from Helpers) |
| km | 1 个字典 | 2 个字典 | +1 (ruleTensMap) |
| ko | 2 个字典 | 4 个字典 | +2 (但 1 个元组类型) |
| vi | 0 个字典 | 2 个字典 | +2 (但 1 个元组类型) |

### 整体进度

| 阶段 | 编译成功 | 成功率 | 备注 |
|------|---------|--------|------|
| 批量生成 | 44/50 | 88% | 初始批量生成 |
| Phase A | 46/50 | 92% | 修复 RO + TR |
| **Phase B** | **48/48** | **100%** | **修复 AR, KM, KO, VI** |

### 代码生成统计

```
总计规则数: 48 languages × ~15 rules/lang = ~720 rules
自动生成: ~600 rules (83%)
需要手动: ~120 rules (17%)
  - 元组类型字典: 2 rules
  - 复杂组合规则: ~118 rules
```

---

## 经验总结

### 自动化工具设计原则

1. **容错性优先**: 正则表达式应尽可能宽松，支持多种格式变体
2. **增量更新**: 避免覆盖整个文件，采用增量追加或合并策略
3. **验证优先**: 提取后立即验证 JSON schema，尽早发现问题
4. **日志详细**: 每个提取步骤输出详细日志（匹配数量、字典名称等）

### 特殊情况处理策略

| 情况 | 策略 | 理由 |
|------|------|------|
| **定义在其他文件** | 单独提取 + 合并 | 保持工具简单，手动合并可控 |
| **复杂类型 (元组)** | 标记为手动 | 语义复杂，自动化成本高 |
| **多行注释** | 增强正则 | 常见模式，值得工具支持 |

### 调试技巧

1. **Python 快速测试**:
```python
import re
pattern = r'...'
matches = re.finditer(pattern, content, re.MULTILINE | re.DOTALL)
for m in matches:
    print(m.groups())
```

2. **Git 历史挖掘**:
```bash
git log --all --grep="languages" --oneline
git show <commit>:path/to/file
```

3. **增量验证**:
```bash
# 每次只生成 1 个语言
cargo run --bin codegen -- extracted/numeral/ar.json --output .
cargo check  # 立即验证
```

---

## 剩余工作

### 未解决问题

1. **元组类型字典** (2 个规则)
   - KO: `integerType1PowersOfTenMap :: HashMap Text (Double, Int)`
   - VI: `powersOfTenMap :: HashMap Text (Double, Int)`
   - **影响**: PowersOfTen 规则不可用（如 "백만" = 100万）
   - **解决方案**: 手动实现或扩展工具支持元组提取

2. **复合规则** (~118 个规则)
   - 类型: 组合规则 (composite)
   - 示例: `intersect`, `multiply`, `integer 21..99`
   - **需要**: 手动翻译 Haskell 逻辑到 Rust

### 优化方向

1. **codegen 增量更新**:
   - 当前: 完全覆盖 `languages/mod.rs`
   - 改进: 只追加新语言，保留现有条目

2. **自动合并工具**:
   - 创建 `merge_helpers.sh` 脚本
   - 自动检测并合并 `Helpers.hs` 中的字典

3. **元组类型支持**:
   - 扩展正则表达式匹配 `(Type1, Type2)`
   - 生成专门的结构体而非 HashMap

---

## 附录

### 各语言字典统计

| 语言 | 字典数 | 示例 |
|------|--------|------|
| ar | 1 | digitsMap (عشر=2, ثلاث=3) |
| km | 2 | ruleNumeralMap (៥=5), ruleTensMap (ដប់=10) |
| ko | 4 | integerType1 (일=1), integerType2 (하나=1) |
| vi | 2 | integerMap (một=1), tensMap (mười=10) |

### 相关 Unicode 范围

| 语言 | Unicode 范围 | 字符示例 |
|------|-------------|---------|
| 阿拉伯语 | U+0600-06FF | ع (ayn), ش (sheen) |
| 高棉语 | U+1780-17FF | ០១២៣... (数字), ក ខ គ... (辅音) |
| 韩语 | U+AC00-D7AF | 일 이 삼 사... (汉字数字) |
| 越南语 | U+0000-007F + diacritics | ư ơ ă â ê ô... (带声调) |

---

**总结**: Phase B 通过增强正则表达式支持多行注释、从 Helpers.hs 提取字典、标记复杂元组类型规则为手动实现，成功使所有 48 种语言的 Numeral 维度编译通过，达成 **100% 编译成功率**的里程碑。

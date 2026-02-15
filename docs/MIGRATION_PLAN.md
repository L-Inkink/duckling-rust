# 全语种迁移计划：Duckling → Rustling

**状态**: 🚧 Phase 0 完成，开始 Phase 1
**更新时间**: 2026-02-14

---

## 📊 总览

### 目标
将 Duckling (Haskell) 的全部 **48 种语言** 和 **14 种维度** 移植到 Rustling (Rust)

### 策略
**半自动化混合方案** - 70%+ 自动化 + 30% 精细化手工

### 进度跟踪

| 阶段 | 任务 | 状态 | 完成度 |
|------|------|------|--------|
| **Phase 0** | 基础设施建设 | ✅ 完成 | 100% |
| **Phase 1** | Numeral 维度试点 | 🚧 进行中 | 10% |
| **Phase 2** | Time 维度迁移 | ⏳ 待开始 | 0% |
| **Phase 3** | 简单维度迁移 (5个) | ⏳ 待开始 | 0% |
| **Phase 4** | 复杂维度迁移 (4个) | ⏳ 待开始 | 0% |
| **Phase 5** | 集成与优化 | ⏳ 待开始 | 0% |

---

## ✅ Phase 0: 基础设施建设 (已完成)

### 已交付工具

#### 1. JSON Schema (`tools/migration/rule_schema.json`)
- ✅ 定义了规则的中间表示格式
- ✅ 支持 4 种模式类型：dictionary, regex, composite, predicate
- ✅ 支持全部 14 种维度类型
- ✅ 包含验证逻辑

#### 2. 规则提取器 (`tools/migration/extract_rules.py`)
- ✅ 从 Haskell 源码提取 HashMap 字典规则 (95% 自动化)
- ✅ 提取简单 regex 规则 (80% 自动化)
- ✅ 识别复合规则并标记需要人工审查
- ✅ 批量处理模式

**测试结果** (English Numeral):
```
✓ Extracted 19 rules:
  - 4 dictionary rules
  - 10 regex rules
  - 5 composite rules (need manual review)
```

#### 3. 代码生成器 (`tools/migration/codegen.rs`)
- ✅ 基于 Tera 模板的 Rust 代码生成
- ✅ 自动生成 `RuleSetBuilder` 代码
- ✅ 创建本地化字符串表
- ✅ 自动更新 `mod.rs` 文件

**模板**:
- ✅ `templates/numeral_rules.rs.tera` - Numeral 维度专用
- ✅ `templates/generic_rules.rs.tera` - 通用模板

#### 4. Corpus 测试转换器 (`tools/migration/corpus_converter.py`)
- ✅ 解析 Duckling Corpus.hs 文件
- ✅ 生成 Rust 测试用例
- ✅ 支持批量处理

### 工具链验证

**已测试**:
```bash
# 提取英语 Numeral 规则
python3 tools/migration/extract_rules.py \
  ~/Project/duckling/Duckling/Numeral/EN/Rules.hs \
  --output extracted/test_en.json

# 结果: ✅ 成功提取 19 条规则
```

---

## 🚧 Phase 1: Numeral 维度试点 (进行中)

### 目标
- 验证工具链完整性
- 迁移全部 48 种语言的 Numeral 规则
- 达到 95%+ Corpus 测试通过率

### 任务清单

#### Task 1.1: 批量提取 Numeral 规则 ⏳
```bash
python3 tools/migration/extract_rules.py \
  --batch ~/Project/duckling/Duckling/Numeral \
  --output extracted/numeral/
```

**预期输出**: 48 个 JSON 文件 (en.json, zh.json, es.json, ...)

#### Task 1.2: 生成 Rust 代码 ⏳
```bash
cargo run --bin codegen --features migration-tools -- \
  extracted/numeral/*.json \
  --output languages/
```

**预期输出**: 48 个 `languages/<locale>/numeral.rs` 文件

#### Task 1.3: 转换 Corpus 测试 ⏳
```bash
python3 tools/migration/corpus_converter.py \
  --batch ~/Project/duckling/Duckling/Numeral \
  --output tests/numeral/
```

#### Task 1.4: 运行测试并修正 ⏳
```bash
cargo test --all -- numeral_corpus
```

**目标**: 95%+ 测试通过

#### Task 1.5: 性能基准测试 ⏳
```bash
cargo bench --bench numeral_bench
```

**目标**: 性能 ≥ Duckling

---

## 🗺️ 路线图

### Week 1 ✅
- [x] JSON Schema 设计
- [x] Haskell 规则提取器 (Python 版本)
- [x] Rust 代码生成器
- [x] Corpus 测试转换器
- [x] 文档完善

### Week 2 (当前)
- [ ] 批量提取 Numeral 规则 (48 语言)
- [ ] 生成 Rust 代码
- [ ] 运行 Corpus 测试
- [ ] 修正失败用例
- [ ] 性能基准测试

### Week 3
- [ ] Time 维度迁移
- [ ] 增强工具链支持组合规则
- [ ] 实现 time helper 函数

### Week 4
- [ ] Duration, Distance, Volume 迁移
- [ ] Temperature, AmountOfMoney 迁移

### Week 5
- [ ] PhoneNumber, Email 迁移
- [ ] URL, Ordinal 迁移

### Week 6
- [ ] 动态语言加载器
- [ ] 内存优化
- [ ] 性能调优
- [ ] 最终验证

---

## 📦 已创建文件清单

### 工具链
- ✅ `tools/migration/README.md` - 工具链文档
- ✅ `tools/migration/rule_schema.json` - JSON Schema 定义
- ✅ `tools/migration/extract_rules.py` - 规则提取器
- ✅ `tools/migration/codegen.rs` - 代码生成器
- ✅ `tools/migration/corpus_converter.py` - 测试转换器

### 模板
- ✅ `templates/numeral_rules.rs.tera` - Numeral 规则模板
- ✅ `templates/generic_rules.rs.tera` - 通用规则模板

### 文档
- ✅ `docs/MIGRATION_PLAN.md` - 本文件

### 测试数据
- ✅ `extracted/test_en.json` - 英语 Numeral 提取示例

---

## 🎯 下一步行动

### 立即执行
1. **批量提取所有 Numeral 规则**
   ```bash
   cd ~/Project/duckling-rust
   python3 tools/migration/extract_rules.py \
     --batch ~/Project/duckling/Duckling/Numeral \
     --output extracted/numeral/
   ```

2. **验证提取质量**
   - 检查每个 JSON 文件的规则数量
   - 抽查 5-10 个文件确保格式正确
   - 统计需要人工审查的规则数量

3. **生成 Rust 代码**
   - 先处理 5 个语言作为测试
   - 确保代码能编译通过
   - 修复模板问题

4. **增量迭代**
   - 10 个语言一批，逐步处理
   - 每批都运行 `cargo check`
   - 记录常见问题模式

---

## 📈 成功指标

### Phase 1 验收标准
- ✅ 48 种语言的 Numeral 规则全部提取
- ✅ 生成的 Rust 代码能通过 `cargo check`
- ✅ 95%+ Corpus 测试通过
- ✅ 性能 ≥ Duckling
- ✅ 代码无 Clippy 警告

### 最终验收标准 (Phase 5)
- ✅ 支持全部 48 语言 × 14 维度 = 672 规则文件
- ✅ 98%+ Duckling Corpus 测试通过
- ✅ 端到端延迟 < 1ms (简单输入)
- ✅ 吞吐量 ≥ Duckling (理想 2-3x)
- ✅ 内存占用 < 100MB (全语言)
- ✅ 测试覆盖率 > 85%

---

## 🤝 贡献指南

### 手动审查规则
如果你看到规则被标记为 `_needs_manual_review`:

1. 打开原始 Haskell 文件
2. 理解 `prod` 函数的逻辑
3. 在生成的 Rust 代码中实现等价逻辑
4. 添加测试用例验证
5. 移除 `_needs_manual_review` 标记

### 报告问题
- GitHub Issues: [duckling-rust/issues](https://github.com/your-org/duckling-rust/issues)
- 标签: `migration`, `needs-review`, `bug`

---

## 📚 参考资料

### Duckling 文档
- [Duckling GitHub](https://github.com/facebook/duckling)
- [Duckling Wiki](https://github.com/facebook/duckling/wiki)

### Rustling 架构
- `core/src/rule_set.rs` - 规则集核心
- `core/src/pattern.rs` - 模式匹配
- `core/src/token.rs` - Token 定义

### 相关技术
- [Tera 模板引擎](https://tera.netlify.app/)
- [serde JSON](https://docs.serde.rs/serde_json/)
- [regex crate](https://docs.rs/regex/)

---

## 📝 更新日志

### 2026-02-14
- ✅ 完成 Phase 0: 基础设施建设
- ✅ 创建全部 4 个工具链工具
- ✅ 验证提取器可用 (English Numeral 测试通过)
- 🚧 开始 Phase 1: Numeral 维度试点

---

**维护者**: Claude Code Migration Team
**最后更新**: 2026-02-14 22:30 UTC+8

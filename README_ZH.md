# Duckling Rust - 自然语言解析引擎

> 基于 [Rustling](https://github.com/sonos/rustling) 扩展的 Duckling Rust 重构版本

[![License](https://img.shields.io/badge/license-Apache%202.0%2FMIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-Phase%200%20Complete-green.svg)](PHASE0_评估报告.md)

---

## 📖 项目简介

Duckling Rust 是 Facebook Duckling 自然语言解析库的现代化 Rust 实现。本项目基于 Sonos 的 Rustling 进行扩展，增加动态规则加载、模糊匹配、HTTP 服务等企业级功能。

**核心功能**:
- 🔍 **自然语言解析** - 从文本中提取时间、数字、金额等结构化数据
- 🤖 **机器学习排序** - 使用朴素贝叶斯对模糊解析结果排序
- 🌐 **多语言支持** - 计划支持中文、英文等 10+ 种语言
- 📱 **跨平台** - 服务端（HTTP API）+ Android（JNI）统一引擎
- ⚡ **高性能** - Rust 零成本抽象，比 Haskell 版本更快

---

## 🎯 项目状态

### 当前进度：Phase 0 完成 ✅

- ✅ **代码评估** - 全面分析 Rustling 代码质量、架构设计
- ✅ **技术决策** - 确定扩展方案，节省 2-3 个月开发时间
- ✅ **风险评估** - 识别技术债务和依赖问题
- ✅ **路线规划** - 制定 Phase 1-5 详细计划

**下一步**: Phase 0 现代化（第 1-2 周）- 依赖更新与审计

---

## 📚 文档导航

| 文档 | 说明 | 阅读对象 |
|-----|------|---------|
| [执行摘要.md](执行摘要.md) | 高层决策摘要（5 分钟速读）| 管理层、决策者 |
| [PHASE0_评估报告.md](PHASE0_评估报告.md) | 完整技术评估（30 分钟）| 技术负责人 |
| [PHASE0_EVALUATION.md](PHASE0_EVALUATION.md) | English version | International team |
| 本文档 | 项目介绍与快速开始 | 开发者 |

---

## 🚀 快速开始

### 前置条件

```bash
# 1. 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. 验证安装
rustc --version  # 应显示 1.70+ 或更高
cargo --version
```

### 编译项目（当前状态）

```bash
# 克隆仓库
cd /Users/link/Project/duckling-rust

# 编译（2019 版本，可能有警告）
cargo build

# 运行测试
cargo test

# 运行 Clippy 检查
cargo clippy
```

**注意**: 当前使用 Rust 2018 edition 和旧依赖，Phase 0 第 1 周将更新。

---

## 🏗️ 架构概览

### 模块组成

```
duckling-rust/
├── core/                 # 核心解析引擎
│   ├── 饱和解析算法      # ✅ 已实现
│   ├── 模式匹配         # ✅ 已实现
│   └── Token 存储       # ✅ 已实现
├── ml/                   # 机器学习模块
│   └── 朴素贝叶斯分类器 # ✅ 已实现
└── src/                  # 高层 API
    ├── Parser           # ✅ 已实现
    └── 训练工具         # ✅ 已实现
```

### 待新增模块（Phase 1-5）

```
duckling-server/          # ❌ HTTP 服务（Actix-web）
duckling-android/         # ❌ Android JNI 封装
rules/                    # ❌ JSON 规则配置
├── en_US/
│   ├── numeral.json
│   └── time.json
└── zh_CN/
    ├── numeral.json
    └── time.json
```

---

## 🛠️ 技术栈

| 组件 | 技术 | 版本 | 状态 |
|-----|------|------|------|
| 核心引擎 | Rust | 2018 → 2021 | ⚠️ 待升级 |
| 正则引擎 | `regex` | 1.0 → 1.10+ | ⚠️ 待升级 |
| 错误处理 | `failure` → `thiserror` | - | ⚠️ 待迁移 |
| ML 库 | 自实现 | - | ✅ 完成 |
| HTTP 服务 | `actix-web` | 4.5 | ❌ 待实现 |
| Android | JNI | - | ❌ 待实现 |
| 配置中心 | Apollo | - | ❌ 待实现 |

---

## 📅 实施计划

### Phase 0: 现代化与审计（2 周）⏳ 进行中

**第 1 周：依赖更新**
- [ ] 升级 Rust edition 2018 → 2021
- [ ] 迁移 `failure` → `thiserror`
- [ ] 更新所有依赖到最新版本
- [ ] 修复编译警告和 Clippy 问题
- [ ] 所有测试通过

**第 2 周：审计与文档**
- [ ] 审计不安全代码
- [ ] 添加性能基准测试
- [ ] 绘制架构图
- [ ] 设置 CI/CD（GitHub Actions）
- [ ] 编写 Haskell → Rust 迁移指南

### Phase 1-5: 功能开发（9-14 周）

| Phase | 时长 | 关键功能 |
|-------|-----|---------|
| Phase 1 | 2-3 周 | JSON 规则加载、模糊匹配、热重载 |
| Phase 2 | 2-3 周 | HTTP 服务、Apollo 集成、监控 |
| Phase 3 | 1-2 周 | C FFI、Android JNI、Kotlin SDK |
| Phase 4 | 2-3 周 | Numeral/Time 维度移植 |
| Phase 5 | 2-3 周 | 多语言、生产部署、文档 |

**总计**: 11-16 周（约 2.5-4 个月）

---

## 🧪 示例用法

### 当前功能（Rustling 原生）

```rust
use rustling::*;

// 构建规则集
let b = RuleSetBuilder::new(
    BoundariesChecker::detailed(),
    BoundariesChecker::separated_alphanumeric_word(),
);

// 定义数字规则
b.rule_1("integer", b.reg(r"\d+")?, |text_match| {
    Ok(IntegerValue(text_match.group(0).parse()?))
});

// 定义组合规则
b.rule_2(
    "number thousands",
    dim!(IntegerValue, vec![Box::new(|a| a.value > 1 && a.value < 99)]),
    dim!(IntegerValue, vec![Box::new(|a| a.value == 1000)]),
    |a, b| Ok(IntegerValue(a.value * 1000))
);

let rules = b.build();

// 解析
let results = rules.apply_all("twelve thousands")?;
// 结果: [12, 1000, 12000]
```

### 计划功能（Phase 1+）

```rust
// JSON 规则加载
let parser = Parser::from_json_rules("rules/en_US/numeral.json")?;

// 模糊匹配
let results = parser.parse_fuzzy("tomorow at 3pm", 0.85)?;
// "tomorow" → "tomorrow" (编辑距离容错)

// HTTP 服务器
let server = DucklingServer::new(8080)
    .with_apollo("http://config.server.com")
    .start()
    .await?;

// Android
val parser = DucklingParser.create(rulesJson)
val tokens = parser.parse("明天下午3点")
```

---

## 🔬 技术亮点

### 1. 饱和解析算法

Rustling 正确实现了 Duckling 的核心算法：

```rust
// 伪代码
fn saturate_parse(text):
    tokens = apply_terminal_rules(text)  // 正则匹配
    loop max 10 times:
        new_tokens = apply_composition_rules(tokens)
        if no_new_tokens or too_many_tokens:
            break
        tokens += new_tokens
    return filter_by_boundaries(tokens)
```

### 2. 零成本抽象

- `SmallVec` - 小数组栈分配，避免堆内存
- `Rc` - 引用计数共享，无运行时开销
- `string-interner` - 符号驻留，减少字符串拷贝

### 3. ML 集成

朴素贝叶斯分类器对模糊解析结果排序：

```rust
P(正确解析|特征) ∝ P(特征|正确) * P(正确)

特征:
- 规则名称
- 解析树高度
- Token 数量
- 子节点特征（递归）
```

---

## 🤝 贡献指南

### 当前阶段（Phase 0）

目前项目处于**现代化阶段**，欢迎贡献：

1. **依赖更新** - 帮助迁移 `failure` → `thiserror`
2. **测试补充** - 添加边界情况测试
3. **文档改进** - 翻译英文文档、添加示例
4. **基准测试** - 性能对比 Haskell 版本

### 开发流程

```bash
# 1. Fork 并克隆
git clone https://github.com/your-username/duckling-rust.git
cd duckling-rust

# 2. 创建功能分支
git checkout -b feature/your-feature

# 3. 开发并测试
cargo build
cargo test
cargo clippy

# 4. 提交 PR
git push origin feature/your-feature
# 在 GitHub 上创建 Pull Request
```

---

## 📊 性能对比

### 预期性能（待 Phase 0 验证）

| 指标 | Haskell Duckling | Rustling | 预期提升 |
|-----|-----------------|----------|---------|
| 解析延迟 | ~100ms | ~50ms | **2x** |
| 内存使用 | ~50MB | ~20MB | **2.5x** |
| 启动时间 | ~500ms | ~50ms | **10x** |
| QPS（单核）| ~100 | ~500 | **5x** |

**注**: 数据基于架构分析推测，Phase 1 将添加实际基准测试。

---

## 📜 许可证

本项目继承 Rustling 的双重许可：

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

可任选其一使用。

---

## 🙏 致谢

- **Facebook** - 原始 Duckling 设计与实现
- **Sonos/Snips** - Rustling Rust 移植
- **Rust 社区** - 优秀的工具链和库生态

---

## 📞 联系方式

- **项目仓库**: `/Users/link/Project/duckling-rust/`
- **问题追踪**: GitHub Issues（待创建）
- **技术文档**: [PHASE0_评估报告.md](PHASE0_评估报告.md)

---

## 🗺️ 路线图

```
2026-02-10  ✅ Phase 0 评估完成
2026-02-17  ⏳ Phase 0 现代化完成（目标）
2026-03-10  🎯 Phase 1-2 完成（HTTP 服务）
2026-03-31  🎯 Phase 3-4 完成（Android + 维度）
2026-04-21  🎯 Phase 5 完成（生产部署）
```

**预计上线时间**: 2026 年 4 月底

---

**最后更新**: 2026-02-10
**维护者**: Claude Code
**版本**: Phase 0 - v0.1.0

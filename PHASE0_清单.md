# Phase 0 实施清单

**目标**: 现代化 Rustling 代码库，为 Phase 1-5 打好基础
**时长**: 2 周
**状态**: ⏳ 准备开始

---

## 📋 总览

- **已完成**: ✅ 评估报告、技术决策、文档编写
- **进行中**: ⏳ 依赖现代化（第 1 周）
- **待开始**: ⏸️ 审计与文档（第 2 周）

---

## 第 1 周：依赖现代化

### Day 1: 环境设置（2026-02-10 或 11）

- [ ] **安装 Rust 工具链**
  ```bash
  # 如果尚未安装
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env

  # 验证安装
  rustc --version  # 应该 >= 1.70
  cargo --version
  ```

- [ ] **设置 Git 远程仓库**
  ```bash
  cd /Users/link/Project/duckling-rust

  # 保留上游引用
  git remote add upstream https://github.com/sonos/rustling.git

  # 添加你的远程仓库（如需要）
  git remote add origin <your-fork-url>

  # 创建现代化分支
  git checkout -b phase0-modernization
  ```

- [ ] **初始编译测试**
  ```bash
  # 编译（预期有警告）
  cargo build

  # 运行测试（预期全部通过）
  cargo test

  # 检查问题
  cargo clippy 2>&1 | tee clippy_before.log
  ```

### Day 2-3: Cargo.toml 更新

- [ ] **更新根 Cargo.toml**
  ```toml
  [package]
  name = "rustling"
  version = "0.10.0"  # 升级版本号
  edition = "2021"     # 2018 → 2021
  rust-version = "1.70"  # 最低 Rust 版本

  [workspace]
  members = ["core", "ml"]  # 保持不变
  ```

- [ ] **更新 core/Cargo.toml**
  ```toml
  [package]
  edition = "2021"

  [dependencies]
  regex = "1.10"              # 1.0 → 1.10
  smallvec = "1.13"           # 0.6 → 1.13
  thiserror = "1.0"           # 新增（替换 failure）
  string-interner = "0.17"    # 0.7 → 0.17
  serde = { version = "1.0", features = ["derive"] }
  ```

- [ ] **更新 ml/Cargo.toml**
  ```toml
  [package]
  edition = "2021"

  [dependencies]
  thiserror = "1.0"           # 新增（替换 failure）
  fnv = "1.0"                 # 保持不变
  serde = { version = "1.0", features = ["derive"] }
  ```

- [ ] **更新主 Cargo.toml 依赖**
  ```toml
  [dependencies]
  rustling-core = { path = "core" }
  rustling-ml = { path = "ml" }
  thiserror = "1.0"           # 新增（替换 failure）
  fnv = "1.0"
  serde = { version = "1.0", features = ["derive"] }
  ```

### Day 4-5: 错误处理迁移（failure → thiserror）

- [ ] **创建新错误类型**（`core/src/error.rs`）
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum RustlingError {
      #[error("Rule error: {0}")]
      Rule(String),

      #[error("Parse error: {0}")]
      Parse(String),

      #[error("Regex error: {0}")]
      Regex(#[from] regex::Error),

      #[error("Other error: {0}")]
      Other(String),
  }

  pub type Result<T> = std::result::Result<T, RustlingError>;
  ```

- [ ] **替换 core/src/lib.rs**
  ```rust
  // 删除
  #[macro_use]
  extern crate failure;

  // 添加
  mod error;
  pub use error::{RustlingError, Result as CoreResult};

  // 全局替换
  // CoreResult<T> = Result<T, ::failure::Error>
  // → CoreResult<T> = Result<T, RustlingError>
  ```

- [ ] **更新 core/src/rule.rs**
  ```rust
  // 替换 RuleError
  use crate::error::RustlingError;

  // 删除
  #[derive(Debug, Fail)]
  pub enum RuleError { ... }

  // 使用 RustlingError::Rule(...) 替代
  ```

- [ ] **更新 ml/src/lib.rs**
  ```rust
  // 删除
  #[macro_use]
  extern crate failure;

  // 添加
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MLError {
      #[error("No classes in classifier")]
      NoClasses,

      #[error("Classification error: {0}")]
      Other(String),
  }

  pub type MLResult<T> = Result<T, MLError>;
  ```

- [ ] **更新主 src/lib.rs**
  ```rust
  // 替换所有 failure 引用
  pub use rustling_core::RustlingError;
  pub type RustlingResult<T> = rustling_core::CoreResult<T>;
  ```

- [ ] **验证编译**
  ```bash
  cargo build 2>&1 | tee build.log
  # 修复所有编译错误
  ```

### Day 6: 依赖更新验证

- [ ] **运行测试套件**
  ```bash
  # 运行所有测试
  cargo test --all

  # 查看失败测试
  cargo test --all -- --nocapture

  # 修复失败的测试
  ```

- [ ] **运行 Clippy**
  ```bash
  cargo clippy --all -- -D warnings

  # 修复所有警告
  cargo clippy --all --fix --allow-dirty
  ```

- [ ] **格式化代码**
  ```bash
  cargo fmt --all
  ```

- [ ] **生成文档**
  ```bash
  cargo doc --no-deps --open

  # 检查文档是否正确生成
  ```

### Day 7: 第 1 周总结与提交

- [ ] **最终验证**
  ```bash
  # 清理构建
  cargo clean

  # 重新编译（release 模式）
  cargo build --release

  # 运行测试
  cargo test --release

  # Clippy 检查
  cargo clippy --all -- -D warnings
  ```

- [ ] **Git 提交**
  ```bash
  git add .
  git commit -m "Phase 0 Week 1: Modernize dependencies

  - Upgrade Rust edition 2018 → 2021
  - Migrate failure → thiserror
  - Update all dependencies to latest versions
  - Fix all clippy warnings
  - All tests passing
  "

  git push origin phase0-modernization
  ```

- [ ] **文档更新**
  - 更新 CHANGELOG.md（记录变更）
  - 更新 Cargo.toml 版本号 → 0.10.0

---

## 第 2 周：审计与文档

### Day 8-9: 代码审计

- [ ] **审计不安全代码**
  ```bash
  # 查找所有 unsafe 代码
  rg "unsafe" --type rust

  # 重点审计: core/src/lib.rs:270
  # unsafe impl Send for SendSyncPhantomData<T>
  # unsafe impl Sync for SendSyncPhantomData<T>
  ```

- [ ] **评估 unsafe 必要性**
  - [ ] 检查是否可用安全代码替换
  - [ ] 添加详细注释说明安全性保证
  - [ ] 如无法替换，编写安全性证明文档

- [ ] **运行 Miri（可选）**
  ```bash
  # 安装 Miri
  rustup component add miri

  # 运行 Miri 检查
  cargo +nightly miri test
  ```

- [ ] **内存泄漏检查（可选）**
  ```bash
  # 使用 valgrind（需安装）
  cargo build --release
  valgrind --leak-check=full \
    target/release/examples/<example_name>
  ```

### Day 10-11: 基准测试

- [ ] **创建基准测试**（`benches/parser_bench.rs`）
  ```rust
  use criterion::{black_box, criterion_group, criterion_main, Criterion};
  use rustling::*;

  fn benchmark_parse(c: &mut Criterion) {
      let rules = create_test_rules();

      c.bench_function("parse simple number", |b| {
          b.iter(|| {
              rules.apply_all(black_box("twenty three"))
          })
      });

      c.bench_function("parse complex", |b| {
          b.iter(|| {
              rules.apply_all(black_box("twelve thousands and forty two"))
          })
      });
  }

  criterion_group!(benches, benchmark_parse);
  criterion_main!(benches);
  ```

- [ ] **添加 criterion 依赖**
  ```toml
  [dev-dependencies]
  criterion = "0.5"

  [[bench]]
  name = "parser_bench"
  harness = false
  ```

- [ ] **运行基准测试**
  ```bash
  cargo bench

  # 生成报告
  open target/criterion/report/index.html
  ```

- [ ] **对比 Haskell 性能（可选）**
  ```bash
  # 在原 Duckling 仓库运行
  cd /Users/link/Project/duckling
  stack bench

  # 记录结果，与 Rust 版本对比
  ```

### Day 12: 架构文档

- [ ] **绘制架构图**
  - 使用 Mermaid 或 PlantUML
  - 展示模块依赖关系
  - 展示数据流

- [ ] **创建 ARCHITECTURE.md**
  ```markdown
  # Rustling 架构设计

  ## 模块结构
  [插入架构图]

  ## 核心组件
  ### 1. 解析引擎（core）
  ### 2. ML 模块（ml）
  ### 3. 高层 API（src）

  ## 数据流
  文本 → 终结规则 → Token → 组合规则 → 饱和 → ML 排序 → 结果
  ```

- [ ] **创建 MIGRATION_GUIDE.md**
  ```markdown
  # Haskell → Rust 迁移指南

  ## 类型映射
  | Haskell | Rust |
  |---------|------|
  | GADT | trait + 泛型 |
  | 类型类 | trait |

  ## 规则迁移
  [详细说明如何迁移规则]
  ```

### Day 13: CI/CD 设置

- [ ] **创建 GitHub Actions 配置**（`.github/workflows/ci.yml`）
  ```yaml
  name: CI

  on: [push, pull_request]

  jobs:
    test:
      runs-on: ubuntu-latest
      strategy:
        matrix:
          rust: [stable, nightly]
      steps:
        - uses: actions/checkout@v3
        - uses: actions-rs/toolchain@v1
          with:
            toolchain: ${{ matrix.rust }}
        - name: Build
          run: cargo build --verbose
        - name: Test
          run: cargo test --verbose
        - name: Clippy
          run: cargo clippy -- -D warnings
        - name: Format
          run: cargo fmt -- --check

    bench:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v3
        - name: Benchmark
          run: cargo bench
  ```

- [ ] **测试 CI 流水线**
  ```bash
  # 推送代码触发 CI
  git push origin phase0-modernization

  # 检查 GitHub Actions 运行结果
  ```

### Day 14: 第 2 周总结

- [ ] **完成所有文档**
  - [x] PHASE0_评估报告.md
  - [x] 执行摘要.md
  - [x] README_ZH.md
  - [x] PHASE0_清单.md（本文档）
  - [ ] ARCHITECTURE.md
  - [ ] MIGRATION_GUIDE.md
  - [ ] CHANGELOG.md

- [ ] **最终检查**
  ```bash
  # 编译
  cargo build --release

  # 测试
  cargo test --all

  # 基准测试
  cargo bench

  # Clippy
  cargo clippy --all -- -D warnings

  # 格式
  cargo fmt --all -- --check

  # 文档
  cargo doc --no-deps
  ```

- [ ] **Git 标签**
  ```bash
  git add .
  git commit -m "Phase 0 Week 2: Audit and documentation

  - Audit unsafe code
  - Add benchmark suite
  - Create architecture documentation
  - Setup CI/CD pipeline
  - Phase 0 complete ✅
  "

  git tag -a v0.10.0 -m "Phase 0 Complete: Rustling Modernized"
  git push origin phase0-modernization --tags
  ```

---

## ✅ Phase 0 验收标准

### 必须达成（P0）

- [ ] ✅ Rust edition 2021
- [ ] ✅ 所有依赖更新到最新
- [ ] ✅ `failure` 完全迁移到 `thiserror`
- [ ] ✅ 所有测试通过（`cargo test --all`）
- [ ] ✅ Clippy 无警告（`cargo clippy --all -- -D warnings`）
- [ ] ✅ 代码格式化（`cargo fmt --all`）

### 推荐达成（P1）

- [ ] 🎯 不安全代码审计完成
- [ ] 🎯 基准测试套件添加
- [ ] 🎯 架构文档完成
- [ ] 🎯 CI/CD 流水线设置
- [ ] 🎯 迁移指南完成

### 可选达成（P2）

- [ ] 💡 Miri 检查通过
- [ ] 💡 与 Haskell 性能对比
- [ ] 💡 内存泄漏检查
- [ ] 💡 代码覆盖率报告

---

## 🚨 常见问题

### Q1: 编译失败 "cannot find macro `format_err`"

**原因**: `failure` 库宏未替换

**解决**:
```rust
// 替换
format_err!("error message")

// 为
RustlingError::Other("error message".to_string())
```

### Q2: `string-interner` API 变更

**原因**: 0.7 → 0.17 有破坏性变更

**解决**: 查看 [string-interner changelog](https://docs.rs/string-interner/0.17.0/string_interner/)，更新 API 调用

### Q3: Clippy 警告过多

**解决**: 使用自动修复
```bash
cargo clippy --all --fix --allow-dirty
```

### Q4: 测试失败

**步骤**:
1. 查看具体失败测试：`cargo test -- --nocapture`
2. 检查是否依赖版本导致
3. 逐个修复，确保逻辑不变

---

## 📊 进度追踪

### 第 1 周进度

| Day | 任务 | 状态 | 完成日期 |
|-----|------|------|---------|
| 1 | 环境设置 | ⏸️ | - |
| 2-3 | Cargo.toml 更新 | ⏸️ | - |
| 4-5 | 错误处理迁移 | ⏸️ | - |
| 6 | 验证 | ⏸️ | - |
| 7 | 提交 | ⏸️ | - |

### 第 2 周进度

| Day | 任务 | 状态 | 完成日期 |
|-----|------|------|---------|
| 8-9 | 代码审计 | ⏸️ | - |
| 10-11 | 基准测试 | ⏸️ | - |
| 12 | 架构文档 | ⏸️ | - |
| 13 | CI/CD | ⏸️ | - |
| 14 | 总结 | ⏸️ | - |

**状态图例**:
- ✅ 已完成
- ⏳ 进行中
- ⏸️ 待开始
- ❌ 已跳过

---

## 🎯 下一步

Phase 0 完成后，立即开始 **Phase 1: 核心功能增强**

重点工作:
1. JSON 规则加载器
2. 模糊匹配模块
3. 规则热重载机制

预计时间: 2-3 周

---

**文档版本**: v1.0
**创建日期**: 2026-02-10
**最后更新**: 2026-02-10

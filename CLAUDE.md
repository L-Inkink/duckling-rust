# Rustling 项目配置

## 项目概述

Rust 实现的 Facebook Duckling NLP 解析器，支持多语言时间/数字解析。

## 自动任务流程

### Roadmap 更新

**触发时机**：每次 `finishing-a-development-branch` 完成后自动执行

**文件位置**：`docs/plans/PROJECT_ROADMAP_V3.md` 末尾的「📋 Phase 状态追踪」区块

**更新规则**：

1. **任务完成时**：
   - 在当前 Phase 的「已完成任务」中添加 `✅ Task N: [任务名] - YYYY-MM-DD`
   - 如果遇到问题，在「遇到的问题」中添加：
     ```markdown
     - **[问题简述]** (YYYY-MM-DD)
       - 现象：...
       - 根因：...
       - 解决：...
     ```

2. **跨 Phase 迁移时**（如 Phase 3 → Phase 4）：
   - 先展示迁移预览给用户确认
   - 用户确认后，将上一 Phase 移动到「已完成 Phase 存档」
   - 只保留「关键问题及解决方案」，移除详细记录

**使用的工具**：
- `Read` - 读取当前状态
- `Edit` - 修改任务状态、添加问题记录
- `Bash` - git add + commit

### Git 提交规范

每次功能完成后自动 commit，提交信息格式：
```
[类型]: 简短描述

详细说明（可选）

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

类型前缀：feat, fix, docs, chore, refactor, test

## 常用命令

```bash
# 运行测试
cargo test

# 检查代码
cargo clippy

# 运行服务器
cargo run --features server
```

# Design: Android Cross-Compilation + Python ctypes Wrapper

**Date**: 2026-02-28
**Status**: Approved
**Context**: Android 设备上离线运行 Python pipeline，duckling-rust 作为归一化步骤

---

## 目标

1. 为 Android 三大 ABI 交叉编译 `librustling.so`（arm64-v8a / armeabi-v7a / x86_64）
2. 提供 Python `ctypes` 包装层，让 pipeline 可 `from rustling.ffi import parse` 直接调用
3. 工具链本地可用（rustup + Homebrew NDK + cargo-ndk）

---

## 现状

| 工具 | 状态 |
|------|------|
| Rust | ✅ v1.93（Homebrew） |
| rustup | ❌ 未安装 |
| Android NDK | ❌ 未安装 |
| cargo-ndk | ❌ 未安装 |
| C FFI (`src/ffi.rs`) | ✅ 已实现，`FfiParseResult` struct |
| C header (`include/rustling.h`) | ✅ 已更新 |
| Python ctypes wrapper | ❌ 待创建 |

---

## 架构

```
src/ffi.rs (FfiParseResult)
    │
    │ cargo-ndk (rustup 管理的交叉编译工具链)
    ▼
android/jniLibs/
    ├── arm64-v8a/librustling.so
    ├── armeabi-v7a/librustling.so
    └── x86_64/librustling.so
    │
    └── python/rustling/ffi.py  (ctypes 包装)
            └── parse(text, locale) → list[dict]
```

---

## 组件设计

### 组件 1: `scripts/install_android_toolchain.sh`

一键安装向导，幂等（可重复运行）：

1. `brew install rustup` — 安装 rustup 工具链管理器
2. `rustup-init --no-modify-path` — 初始化 rustup（不修改 PATH）
3. `~/.cargo/bin/rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android`
4. `brew install android-ndk` — 安装最新 Android NDK
5. `~/.cargo/bin/cargo install cargo-ndk`
6. 输出验证摘要

### 组件 2: `.cargo/config.toml`（追加）

追加三段 Android linker 配置（linker 路径从 Homebrew NDK 安装位置推断）：

```toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "armv7a-linux-androideabi21-clang"

[target.x86_64-linux-android]
linker = "x86_64-linux-android21-clang"
```

API level 21（Android 5.0），与 `minSdkVersion` 对齐。

### 组件 3: `scripts/build_android.sh`

```
检查 ANDROID_NDK_HOME → 设置 PATH 到 NDK clang →
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 \
          -o android/jniLibs build --release →
file android/jniLibs/**/*.so 验证 ELF 格式
```

### 组件 4: Python 包 `python/rustling/`

**`ffi.py`**：
- `_ParseResult(ctypes.Structure)` 对应 `FfiParseResult { json, count, error }`
- `_load()` 自动检测平台选择 `.so` / `.dylib` / `.dll`；Android 上传 `None`
- `parse(text, locale) → list[dict]`：调用 `rustling_parse`，释放内存，返回 JSON

**`__init__.py`**：`from .ffi import parse`

---

## 文件变更清单

| 文件 | 操作 |
|------|------|
| `scripts/install_android_toolchain.sh` | 新增 |
| `scripts/build_android.sh` | 新增 |
| `.cargo/config.toml` | 追加 Android linker 段 |
| `python/rustling/__init__.py` | 新增 |
| `python/rustling/ffi.py` | 新增 |

---

## 验证方法

```bash
# 工具链安装验证
~/.cargo/bin/rustup target list --installed | grep android

# Android .so 验证
file android/jniLibs/arm64-v8a/librustling.so
# 期望: ELF 64-bit LSB shared object, ARM aarch64

# 桌面 Python 快速回归（先 cargo build --release）
python python/rustling/ffi_test.py
```

---

## 风险与说明

- Homebrew Rust 与 rustup 并存：构建脚本明确使用 `~/.cargo/bin/cargo`（rustup 管理的版本），日常开发仍可使用 Homebrew Rust
- NDK 路径：Homebrew 安装路径通常为 `/opt/homebrew/opt/android-ndk`，构建脚本自动推断

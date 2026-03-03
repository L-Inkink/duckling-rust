# Rustling → libnlu 静态库集成指南

**完成日期**: 2026-03-02
**适用分支**: `ffi-struct-api-and-android-guide`

> 本文档说明如何将 rustling 编译为静态库（`.a`），并链接进 libnlu（C++ 项目）的构建系统，最终产物为 `libnlu_static_lib.a`，可嵌入 Android APK。

---

## 目录

1. [为什么用静态库](#1-为什么用静态库)
2. [集成架构](#2-集成架构)
3. [前置条件](#3-前置条件)
4. [rustling 侧配置](#4-rustling-侧配置)
5. [libnlu CMakeLists.txt 修改](#5-libnlu-cmakeliststxt-修改)
6. [构建验证](#6-构建验证)
7. [在 C++ 中调用 rustling API](#7-在-c-中调用-rustling-api)
8. [已知问题与注意事项](#8-已知问题与注意事项)

---

## 1. 为什么用静态库

libnlu 最终产物是 `libnlu_static_lib.a`，要嵌入 Android APK。

| 方案 | 优点 | 缺点 |
|------|------|------|
| 动态库 `.so` | 更新无需重新编译 APK | Android 部署需放到特定路径；存在版本冲突风险 |
| **静态库 `.a`** ✅ | 编译时直接嵌入；零运行时依赖；无路径问题 | 产物体积略大（librustling.a ~41MB，链接后不会全部保留） |

---

## 2. 集成架构

```
duckling-rust/src/ffi.rs       ← #[no_mangle] extern "C" 函数
duckling-rust/include/rustling.h  ← C 头文件（公开 API）
        │
        │  cargo build --release --target <arch> --lib
        ▼
duckling-rust/target_user/<arch>/release/librustling.a
        │
        │  CMake: add_library(rustling STATIC IMPORTED)
        │          target_link_libraries(nlu_static_lib rustling)
        ▼
libnlu/libnlu_static_lib.a     ← 最终产物（含 rustling 符号）
        │
        ▼
Android APK / nlu_manager_test
```

**目标架构映射**：

| libnlu 构建类型 | Rust target | librustling.a 路径 |
|----------------|-------------|-------------------|
| `x86` (本地开发) | `x86_64-unknown-linux-gnu` | `target_user/x86_64-unknown-linux-gnu/release/` |
| `arm64` (Android) | `aarch64-linux-android` | `target_user/aarch64-linux-android/release/` |

---

## 3. 前置条件

### rustling 侧

```bash
# 安装 Rust 工具链（用户目录，无需 sudo）
export RUSTUP_HOME=/data0/lizezhou/.rustup
export CARGO_HOME=/data0/lizezhou/.cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# 添加 Android 目标
/data0/lizezhou/.cargo/bin/rustup target add aarch64-linux-android
/data0/lizezhou/.cargo/bin/rustup target add x86_64-unknown-linux-gnu  # 通常已内置
```

### Android NDK（ARM64 构建时需要）

```bash
# NDK r27c 安装到用户目录（无需 sudo）
mkdir -p /data0/lizezhou/android-ndk
cd /data0/lizezhou/android-ndk
curl -L -o ndk.zip https://dl.google.com/android/repository/android-ndk-r27c-linux.zip
unzip -q ndk.zip && rm ndk.zip
# → /data0/lizezhou/android-ndk/android-ndk-r27c/
```

### 环境变量（加入 `~/.bashrc`）

```bash
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$HOME/.cargo/bin:$PATH"
export ANDROID_NDK_HOME="/data0/lizezhou/android-ndk/android-ndk-r27c"
```

---

## 4. rustling 侧配置

### `.cargo/config.toml`（已纳入 git）

```toml
# target/ 目录由 root 创建，用 target_user/ 避免权限问题
[build]
target-dir = "target_user"

# ARM64 Android
[target.aarch64-linux-android]
linker = "/data0/lizezhou/android-ndk/android-ndk-r27c/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
ar     = "/data0/lizezhou/android-ndk/android-ndk-r27c/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

# ARMv7 Android（可选，旧设备）
[target.armv7-linux-androideabi]
linker = "/data0/lizezhou/android-ndk/android-ndk-r27c/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
ar     = "/data0/lizezhou/android-ndk/android-ndk-r27c/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
```

> **注意**：`[build] target-dir = "target_user"` 使所有 cargo 构建输出到 `target_user/`，
> 而不是 root 所有的 `target/`。`target_user/` 已加入 `.gitignore`。

### 手动构建验证

```bash
cd /data0/lizezhou/nlp/duckling-rust

# x86（本地）
cargo build --release --target x86_64-unknown-linux-gnu --lib
ls -lh target_user/x86_64-unknown-linux-gnu/release/librustling.a
# → 41MB ✅

# ARM64 Android
cargo build --release --target aarch64-linux-android --lib
ls -lh target_user/aarch64-linux-android/release/librustling.a
# → 41MB ✅
```

### `Cargo.toml` 关键配置（无需修改，已就绪）

```toml
[lib]
name = "rustling"
crate-type = ["lib", "staticlib", "cdylib"]
```

`staticlib` 确保生成 `.a` 文件，`cdylib` 生成 `.so`（两者可共存）。

---

## 5. libnlu CMakeLists.txt 修改

在 `find_package(Protobuf REQUIRED)` 块之后、`add_library(nlu_static_lib ...)` 之前添加：

```cmake
# ── Rustling NLP 静态库集成 ──────────────────────────────────────────────
set(RUSTLING_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../duckling-rust")

if("${TARGET_COMPILE_TYPE}" STREQUAL "x86")
    set(RUSTLING_TARGET "x86_64-unknown-linux-gnu")
else()
    set(RUSTLING_TARGET "aarch64-linux-android")
endif()

set(RUSTLING_LIB_PATH
    "${RUSTLING_SRC_DIR}/target_user/${RUSTLING_TARGET}/release/librustling.a")

# 构建 rustling（如果 .a 不存在则自动触发 cargo build）
add_custom_command(
    OUTPUT  "${RUSTLING_LIB_PATH}"
    COMMAND cargo build --release --target ${RUSTLING_TARGET} --lib
    WORKING_DIRECTORY "${RUSTLING_SRC_DIR}"
    COMMENT "Building Rustling for ${RUSTLING_TARGET}"
)
add_custom_target(rustling_build DEPENDS "${RUSTLING_LIB_PATH}")

# 导入静态库
add_library(rustling STATIC IMPORTED)
set_target_properties(rustling PROPERTIES
    IMPORTED_LOCATION "${RUSTLING_LIB_PATH}"
)
add_dependencies(rustling rustling_build)
# ─────────────────────────────────────────────────────────────────────────
```

在 `target_compile_definitions(nlu_static_lib ...)` 之后添加：

```cmake
# 链接 rustling 静态库
target_include_directories(nlu_static_lib PRIVATE "${RUSTLING_SRC_DIR}/include")
target_link_libraries(nlu_static_lib rustling)
```

> **`add_custom_command` vs `add_custom_target`**：
> - `add_custom_command(OUTPUT ...)` — CMake 将 `.a` 文件视为构建产物，仅在文件不存在时重新触发 `cargo build`
> - `add_custom_target(rustling_build DEPENDS ...)` — 使其成为有名字的目标，供 `add_dependencies` 引用

---

## 6. 构建验证

```bash
# 确保 cargo 在 PATH 中
export PATH=/data0/lizezhou/.cargo/bin:$PATH

# x86 构建（含 rustling 集成）
cd /data0/lizezhou/nlp/libnlu
./build.sh x86

# 观察 CMake 输出中是否有：
# "Building Rustling for x86_64-unknown-linux-gnu"

# 验证产物
ls -lh output_x86/libnlu_static_lib.a
nm output_x86/libnlu_static_lib.a | grep rustling_parse
# → 应能看到 rustling_parse 等符号
```

---

## 7. 在 C++ 中调用 rustling API

头文件：`duckling-rust/include/rustling.h`（已通过 `target_include_directories` 引入）

```cpp
#include "rustling.h"
#include <nlohmann/json.hpp>  // libnlu 已有 json 库

// 建议在 NLUManager 初始化时调用（线程安全，幂等）
rustling_init();

// 解析文本
RustlingParseResult result = rustling_parse("5 minutes", "en");
if (result.json != nullptr) {
    auto j = nlohmann::json::parse(result.json);
    // j[0]["value"]["type"]    == "duration"
    // j[0]["value"]["seconds"] == 300.0
    // j[0]["body"]             == "5 minutes"
}
rustling_free_result(result);  // 必须释放，否则内存泄漏

// 检查 locale 支持
if (rustling_locale_supported("zh")) {
    auto res = rustling_parse("明天下午三点", "zh");
    // ...
    rustling_free_result(res);
}
```

### 可用 API（来自 `include/rustling.h`）

| 函数 | 说明 |
|------|------|
| `rustling_init()` | 预热解析器（建议在启动时调用一次） |
| `rustling_parse(text, locale)` | 解析文本，返回 `RustlingParseResult`（含 JSON） |
| `rustling_free_result(result)` | 释放 `rustling_parse` 返回的内存 |
| `rustling_locale_supported(locale)` | 检查 locale 是否支持（返回 `bool`） |
| `rustling_supported_locales()` | 返回支持的 locale JSON 数组字符串（需手动释放） |
| `rustling_version()` | 返回版本字符串 |

### 返回 JSON 格式示例

```json
[
  {
    "body": "5 minutes",
    "dim": "duration",
    "start": 0,
    "end": 9,
    "value": {
      "type": "value",
      "seconds": 300.0,
      "normalized": { "value": 300.0, "unit": "second" }
    }
  }
]
```

---

## 8. 已知问题与注意事项

| 问题 | 原因 | 解决方式 |
|------|------|----------|
| `target/` 权限拒绝 | 该目录由 root 创建（Docker 遗留） | `.cargo/config.toml` 中设置 `target-dir = "target_user"` |
| CMake 中 `cargo` 未找到 | build 环境 PATH 不含 cargo | 在 CMake 调用前 `export PATH=/data0/lizezhou/.cargo/bin:$PATH` |
| `.cargo/` 被 gitignore 整体忽略 | 旧 gitignore 写法 `.cargo/` | 改为 `.cargo/*` + `!.cargo/config.toml` |
| ARM64 构建需要 NDK | libnlu arm64 路径写死 `/mnt/ndk/` | 本地 NDK 已安装至 `~/android-ndk/android-ndk-r27c/` |
| 首次构建较慢（~60s） | cargo 全量编译 41MB 静态库 | 产物缓存在 `target_user/`，后续增量编译极快 |

---

## 参考

- `duckling-rust/include/rustling.h` — 完整 C API 声明
- `duckling-rust/src/ffi.rs` — FFI 实现（`#[no_mangle] extern "C"`）
- `duckling-rust/.cargo/config.toml` — 跨编译工具链配置
- `libnlu/CMakeLists.txt` — 集成入口（已修改）
- `libnlu/docs/rust_integration.md` — libnlu 侧集成通用方案

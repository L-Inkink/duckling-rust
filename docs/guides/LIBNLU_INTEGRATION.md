# Rustling → libnlu 静态库集成指南

**完成日期**: 2026-03-03
**适用分支**: `ffi-struct-api-and-android-guide`

> 本文档说明如何将 rustling 编译为静态库（`.a`），并链接进 libnlu（C++ 项目）的构建系统，最终产物为 `libnlu_static_lib.a`，可嵌入 Android APK。
>
> libnlu 实际位于 `~/nlp/dm-sdk/libnlu/`（dm-sdk 项目子目录），修改已应用至该路径。

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
duckling-rust/target_user/<arch>/release/librustling.a (41MB)
        │
        │  CMake: 提取 .o 文件，嵌套加入 libnlu_static_lib.a
        │  ─────────────────────────────────────────────────
        │  ar x librustling.a → 提取所有 .o
        │  ar r libnlu_static_lib.a *.o → 嵌套合并
        ▼
libnlu_static_lib.a (169MB)    ← 最终产物（含完整 rustling）
        │
        ▼
Android APK / nlu_manager_test
```

**嵌套原理**：静态库本质是目标文件（`.o`）的归档。CMake 自定义命令提取 `librustling.a` 中的所有 `.o` 文件，再通过 `ar r` 加入 `libnlu_static_lib.a`，实现真正的嵌套合并。

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

> **路径说明**：
> - dm-sdk 项目结构：`~/nlp/dm-sdk/libnlu/CMakeLists.txt`，duckling-rust 位于 `~/nlp/duckling-rust/`
> - 相对路径：`../../duckling-rust`（libnlu → dm-sdk → nlp → duckling-rust）
> - 若 libnlu 是独立项目（与 duckling-rust 并列），改为 `../duckling-rust`

```cmake
# ── Rustling NLP 静态库集成 ──────────────────────────────────────────────
set(RUSTLING_SRC_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../duckling-rust")

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

# ── 嵌套 librustling.a 到 libnlu_static_lib.a ─────────────────────────────
# 方法：提取 librustling.a 中的所有 .o 文件，加入 libnlu_static_lib.a
set(NLUNLU_STATIC_LIB_PATH "${CMAKE_BINARY_DIR}/lib/libnlu_static_lib.a")

# 创建一个辅助脚本来合并静态库
set(MERGE_SCRIPT "${CMAKE_BINARY_DIR}/merge_rustling.sh")
file(WRITE "${MERGE_SCRIPT}" "
#!/bin/bash
set -e
WORK_DIR=\"${CMAKE_BINARY_DIR}/rustling_objs\"
mkdir -p \"$WORK_DIR\"
cd \"$WORK_DIR\"
ar x \"${RUSTLING_LIB_PATH}\"
for f in *.o; do
    ar r \"${NLUNLU_STATIC_LIB_PATH}\" \"$f\" || true
done
cd \"${CMAKE_BINARY_DIR}\"
rm -rf rustling_objs
")

add_custom_command(
    OUTPUT  "${NLUNLU_STATIC_LIB_PATH}.merged"
    COMMAND chmod +x "${MERGE_SCRIPT}" && "${MERGE_SCRIPT}"
    DEPENDS nlu_static_lib rustling_build
    COMMENT "Embedding librustling.a into libnlu_static_lib.a"
)

add_custom_target(nlu_static_lib_merged ALL DEPENDS "${NLUNLU_STATIC_LIB_PATH}.merged")
# ─────────────────────────────────────────────────────────────────────────
```

> **嵌套原理**：`target_link_libraries` 对静态库仅记录依赖，不会把依赖的静态库内容合并进来。因此需要自定义命令手动提取 `librustling.a` 中的 `.o` 文件并 `ar r` 到 `libnlu_static_lib.a`。

---

## 6. 构建验证

### 6.1 构建产物

```bash
# 确保 cargo 在 PATH 中
export PATH=/data0/lizezhou/.cargo/bin:$PATH

# x86 构建（含 rustling 嵌套）
cd ~/nlp/dm-sdk/libnlu/cmake/build
make nlu_static_lib_merged

# 验证产物大小（嵌套前 129MB → 嵌套后 169MB）
ls -lh lib/libnlu_static_lib.a
# → 169MB ✅

# 验证对象数量（嵌套前 26 → 嵌套后 468）
ar -t lib/libnlu_static_lib.a | wc -l
# → 468 ✅
```

### 6.2 验证 rustling 符号存在

```bash
# 检查 rustling 对象文件
ar -t lib/libnlu_static_lib.a | grep rustl
# → rustling.rustling.xxxxx-cgu.0*.rcgu.o (442 个)

# 检查符号定义
nm lib/libnlu_static_lib.a | grep rustling_parse
# → 0000000000000000 T rustling_parse ✅
```

### 6.3 运行功能验证测试

```bash
# 编译测试程序
g++ -o verify_libnlu_rustling \
    ~/nlp/dm-sdk/libnlu/verify_libnlu_rustling.cpp \
    -I~/nlp/duckling-rust/include \
    -I~/nlp/dm-sdk/libnlu \
    lib/libnlu_static_lib.a \
    -lpthread -ldl -lm

# 运行
./verify_libnlu_rustling
```

**预期输出**：
```
=== libnlu_static_lib.a embedded rustling verification ===
rustling version: 0.10.0
  ✓ version OK
supported locales: ["ar","bg","ca","da","de","el","en","es","fr","ga","he","hr","hu","it","ja","ka","ko","nb","nl","pl","pt","ro","ru","sv","tr","uk","vi","zh"]
  ✓ locales OK
  ✓ init OK

--- parse tests ---
parse("42", "en"):
  json: [{"byte_end":2,"byte_start":0,"char_end":2,"char_start":0,"value":{"Integer":42}}]
  count: 1
  ✓ Integer found
parse("tomorrow", "en"):
  json: [{"byte_end":8,"byte_start":0,"char_end":8,"char_start":0,"value":{"Time":{"Instant":{"datetime":"2026-03-04T00:00:00Z","form":"Unspecified","grain":"Day","latent":false}}}}]
  count: 1
  ✓ Time found
parse("5 minutes", "en"):
  json: [{"byte_end":1,"byte_start":0,"char_end":1,"char_start":0,"value":{"Integer":5}},{"byte_end":9,"byte_start":0,"char_end":9,"char_start":0,"value":{"Duration":{"amount":5,"unit":"Minute"}}}]
  count: 2
  ✓ Duration found
parse("", "en"):
  count: 0
  ✓ empty input handled

=== All tests completed ===
```

> 测试文件位置：`~/nlp/dm-sdk/libnlu/verify_libnlu_rustling.cpp`

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
| `nlu_manager_test` 链接失败（`GLIBC_2.38`） | dm-sdk `third_party/x86/protobuf-3.6.1/lib/libprotobuf.so` 为高版本 glibc 编译，本机 Ubuntu 20.04 不支持 | 与 rustling 无关；`libnlu_static_lib.a` 已成功构建（169MB）；可在 Docker / 匹配环境中完成最终链接 |
| 静态库链接后符号未找到 | `target_link_libraries` 对静态库仅记录依赖，不嵌入内容 | 使用嵌套合并：提取 `librustling.a` 的 `.o` 文件，`ar r` 加入 `libnlu_static_lib.a` |

---

## 参考

- `duckling-rust/include/rustling.h` — 完整 C API 声明
- `duckling-rust/src/ffi.rs` — FFI 实现（`#[no_mangle] extern "C"`）
- `duckling-rust/.cargo/config.toml` — 跨编译工具链配置
- `duckling-rust/tests/verify_static_lib.c` — x86_64 静态库验证（C 测试）
- `libnlu/CMakeLists.txt` — 集成入口（已修改）
- `libnlu/verify_libnlu_rustling.cpp` — libnlu 嵌套验证（C++ 测试）

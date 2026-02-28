# Android + Python + Rust 集成方案

> **适用场景**：Android 设备上离线运行一个 Python pipeline，duckling-rust（librustling）作为归一化步骤之一被调用。
>
> **本文档为架构参考**，不含可直接运行的完整工程代码。

---

## 目录

1. [方案概述与架构选型](#1-方案概述与架构选型)
2. [工具链与前置条件](#2-工具链与前置条件)
3. [构建 Rust .so（cargo-ndk）](#3-构建-rust-so-cargo-ndk)
4. [C++11 约束（消费方 Android 项目）](#4-c11-约束消费方-android-项目)
5. [Python ctypes 包装层](#5-python-ctypes-包装层)
6. [Chaquopy 打包（Python on Android）](#6-chaquopy-打包python-on-android)
7. [备选方案：PyO3](#7-备选方案-pyo3)
8. [验证方法](#8-验证方法)

---

## 1. 方案概述与架构选型

### 为什么不用 PyO3 直接打包成 Android 库？

| 方案 | 问题 |
|------|------|
| PyO3 编译成 Android 库 | `maturin` 对 Android 的交叉编译支持尚不稳定（experimental） |
| Python 打包成 Android `.aar` | Python 不是 Android 原生环境，无标准打包路径 |
| Chaquopy + PyO3 `.so` | 交叉编译复杂；Chaquopy 只能用于 APK 级别，不能作独立库提供 |

### 推荐方案：ctypes 调用现有 C FFI + cargo-ndk + Chaquopy

```
Rust 源码 (src/ffi.rs)  ←— 现有实现，无需改动
        │
        │ cargo-ndk 交叉编译
        ▼
 librustling.so  (arm64-v8a / armeabi-v7a / x86_64)
        │
        ├──→ Python ctypes wrapper
        │         由 Chaquopy 随 APK 打包到 Android 设备
        │
        └──→ Android NDK CMakeLists.txt（声明 C++11 约束）
```

**优点**：
- `src/ffi.rs` 中已有完整 C FFI，无需新增绑定代码
- `cargo-ndk` 对 Android 交叉编译支持成熟稳定
- ctypes 方案零依赖，在 Chaquopy 的 Python 环境中直接可用
- 与现有 `include/rustling.h` C 头文件天然对齐

---

## 2. 工具链与前置条件

### 必要工具

| 工具 | 版本要求 | 安装方式 |
|------|----------|----------|
| Android NDK | r25+ | Android Studio SDK Manager |
| cargo-ndk | 最新版 | `cargo install cargo-ndk` |
| Rust targets | — | 见下方 |
| Chaquopy | 15.0.1+ | Android 项目 build.gradle |

### 环境变量

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk-r25c
# 确认路径存在
ls $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/
```

### Rust 交叉编译目标

```bash
rustup target add aarch64-linux-android      # ARM64（主流手机）
rustup target add armv7-linux-androideabi    # ARMv7（旧设备）
rustup target add x86_64-linux-android      # x86_64（模拟器）
```

---

## 3. 构建 Rust .so（cargo-ndk）

### 构建命令

```bash
cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64 \
  -o android/jniLibs \
  build --release
```

**产物路径**：
```
android/jniLibs/
├── arm64-v8a/librustling.so
├── armeabi-v7a/librustling.so
└── x86_64/librustling.so
```

### `.cargo/config.toml` 补充 Android linker

```toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "armv7a-linux-androideabi21-clang"

[target.x86_64-linux-android]
linker = "x86_64-linux-android21-clang"
```

> **说明**：`21` 对应 API level 21（Android 5.0），可根据 `minSdkVersion` 调整。`clang` 路径由 `ANDROID_NDK_HOME` 自动推断。

---

## 4. C++11 约束（消费方 Android 项目）

librustling.so 内部使用了 C++11 特性（通过 Rust 的 `std` 运行时），消费方 Android 项目需声明对应的 C++ 标准。

### CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.18)
project(myapp_native)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# 声明导入的预编译 .so
add_library(rustling SHARED IMPORTED)
set_target_properties(rustling PROPERTIES
    IMPORTED_LOCATION
    "${CMAKE_SOURCE_DIR}/jniLibs/${ANDROID_ABI}/librustling.so"
)
```

### build.gradle（app 模块）

```groovy
android {
    defaultConfig {
        externalNativeBuild {
            cmake {
                arguments "-DANDROID_STL=c++_static",
                          "-DCMAKE_CXX_STANDARD=11"
                cppFlags "-std=c++11"
            }
        }
    }
    externalNativeBuild {
        cmake {
            path "CMakeLists.txt"
            version "3.18.1"
        }
    }
}
```

---

## 5. Python ctypes 包装层

> 对应现有 C FFI 接口，见 `src/ffi.rs` 和 `include/rustling.h`。

### rustling/ffi.py

```python
# rustling/ffi.py  — 调用现有 C FFI，无需 PyO3
import ctypes
import json
import sys
import os


class _ParseResult(ctypes.Structure):
    _fields_ = [
        ("json",  ctypes.c_char_p),
        ("count", ctypes.c_uint32),
        ("error", ctypes.c_char_p),
    ]


def _load():
    """
    加载 librustling 共享库。

    Android 上 .so 由系统在进程启动时加载，传 None 即可访问全局符号。
    桌面开发时按平台选择文件名。
    """
    if "ANDROID_DATA" in os.environ:
        # Android 环境：.so 已由 System.loadLibrary 或 JNI 注册
        lib = ctypes.CDLL(None)
    else:
        name = {
            "darwin": "librustling.dylib",
            "win32":  "rustling.dll",
        }.get(sys.platform, "librustling.so")
        lib = ctypes.CDLL(os.path.join(os.path.dirname(__file__), name))

    lib.rustling_parse.restype  = _ParseResult
    lib.rustling_parse.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
    lib.rustling_free_result.argtypes = [_ParseResult]
    lib.rustling_free_result.restype  = None
    lib.rustling_init()
    return lib


_lib = _load()


def parse(text: str, locale: str = "en") -> list:
    """
    解析自然语言文本，返回归一化结果列表。

    Args:
        text:   输入文本，例如 "5 minutes from now"
        locale: 语言标识，例如 "en"、"zh"、"fr"

    Returns:
        解析结果列表（每项为 dict）

    Raises:
        RuntimeError: 解析失败时抛出，携带 Rust 侧错误信息
    """
    r = _lib.rustling_parse(text.encode(), locale.encode())
    if r.error:
        raise RuntimeError(r.error.decode())
    data = json.loads(r.json.decode())
    _lib.rustling_free_result(r)
    return data
```

### Pipeline 调用示例

```python
from rustling.ffi import parse


def normalize(text: str, locale: str = "zh") -> dict:
    """
    对输入文本进行时间/数量归一化，返回第一个匹配结果。
    若无匹配，返回空 dict。
    """
    results = parse(text, locale)
    return results[0] if results else {}


# 示例
if __name__ == "__main__":
    print(normalize("明天下午三点", "zh"))
    # 期望：{'value': {'Time': {...}}, 'dim': 'time', ...}
```

---

## 6. Chaquopy 打包（Python on Android）

[Chaquopy](https://chaquo.com/chaquopy/) 是将 Python 嵌入 Android APK 的 Gradle 插件。

### 消费方 Android 项目 build.gradle

```groovy
plugins {
    id 'com.android.application'
    id 'com.chaquo.python' version '15.0.1'
}

android {
    defaultConfig {
        minSdk 21

        python {
            buildPython "python3"
            // 如需额外 Python 包
            // pip { install "requests" }
        }

        // 将编译好的 .so 纳入 jniLibs
        sourceSets {
            main {
                jniLibs.srcDirs += ["path/to/jniLibs"]
            }
        }
    }
}
```

### Python 文件放置

将 `rustling/` 目录放入 Android 项目的 `src/main/python/` 下：

```
app/src/main/python/
└── rustling/
    ├── __init__.py
    └── ffi.py
```

Chaquopy 会自动将 `src/main/python/` 下的 Python 文件打包进 APK，运行时可直接 `import`。

### 在 Java/Kotlin 中调用

```kotlin
// 加载 .so（必须在 Python 初始化前）
System.loadLibrary("rustling")

// 初始化 Python（Chaquopy 方式）
val python = Python.getInstance()
val module = python.getModule("rustling.ffi")
val result = module.callAttr("parse", "5 minutes", "en")
```

---

## 7. 备选方案：PyO3

若 pipeline 需要更 Pythonic 的 API（类型提示、Pythonic 异常、`__repr__` 等），可考虑 PyO3。

### 适用场景

- 需要在 Python 侧暴露复杂对象模型
- 团队熟悉 Rust + PyO3 工作流
- 目标平台不限于 Android（也打算支持 Linux/macOS 桌面）

### 实现要点

`Cargo.toml` 添加：
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["abi3-py38"] }
```

新增 `src/python.rs`，使用 `#[pymodule]` 导出接口。

### 现阶段不推荐的原因

- `maturin build --target aarch64-linux-android` 目前为 **experimental** 状态
- 需要额外维护 Python ABI 兼容性（abi3 stable ABI）
- 构建链更复杂，排错成本高
- 现有 C FFI（`src/ffi.rs`）已能满足 ctypes 方案的所有需求

---

## 8. 验证方法

### 验证 Android .so 正确构建

```bash
# 构建（需配置好 ANDROID_NDK_HOME）
cargo ndk -t arm64-v8a -o android/jniLibs build --release

# 检查 ELF 格式
file android/jniLibs/arm64-v8a/librustling.so
# 期望：ELF 64-bit LSB shared object, ARM aarch64, ...

# 检查导出符号
nm -D android/jniLibs/arm64-v8a/librustling.so | grep rustling
# 期望：T rustling_init  T rustling_parse  T rustling_free_result
```

### 桌面验证 Python 包装（快速回归）

```bash
# 先编译桌面版
cargo build --release

# 将 .so 放到 python/rustling/ 下，然后
python -c "
import sys
sys.path.insert(0, 'python')
from rustling.ffi import parse
print(parse('5 minutes', 'en'))
# 期望：[{'value': {'Duration': {'amount': 5, 'unit': 'Minute'}}, ...}]
"
```

### Rust 单元测试（确认无回归）

```bash
cargo test --lib
```

---

## 参考资料

- [cargo-ndk GitHub](https://github.com/bbqsrc/cargo-ndk)
- [Chaquopy 文档](https://chaquo.com/chaquopy/doc/current/)
- [Android NDK 指南：使用预编译库](https://developer.android.com/ndk/guides/prebuilt_libs)
- [PyO3 Android 支持（experimental）](https://pyo3.rs/v0.22.0/building-and-distribution/android)
- 本项目 C FFI 接口：`src/ffi.rs`、`include/rustling.h`

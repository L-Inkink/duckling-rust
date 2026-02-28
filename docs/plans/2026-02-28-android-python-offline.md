# Android Cross-Compilation + Python ctypes Wrapper Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 让 duckling-rust 能以 `librustling.so` 形式在 Android 三大 ABI 上离线运行，并通过 Python ctypes 包装层在 Android 上的 Python pipeline 中直接调用。

**Architecture:** cargo-ndk 管理 Android 交叉编译工具链，Python 包通过 ctypes 直接绑定现有 C FFI（`src/ffi.rs` 中的 `FfiParseResult` struct），无需 PyO3 或额外绑定代码。工具链（rustup + Homebrew NDK）与现有 Homebrew Rust 并存，构建脚本明确使用 `~/.cargo/bin/cargo`。

**Tech Stack:** rustup（工具链管理）、cargo-ndk（Android 交叉编译）、Android NDK via Homebrew、Python ctypes（FFI 绑定）

---

## 背景知识

**为什么需要 rustup？**
当前系统 Rust 是 Homebrew 安装的，不支持 `rustup target add`（无法添加 Android 交叉编译目标）。需要安装 rustup 来管理 Android targets。rustup 安装在 `~/.cargo/bin/`，与 Homebrew Rust 并存，互不影响。

**C FFI 当前接口（`src/ffi.rs`）：**
```c
// 核心 struct（C 内存布局）
typedef struct {
    char    *json;    // JSON 结果数组，失败时为 NULL
    uint32_t count;   // 匹配数量
    char    *error;   // 错误信息，成功时为 NULL
} FfiParseResult;

// 导出函数
FfiParseResult rustling_parse(const char* text, const char* locale);
void rustling_free_result(FfiParseResult result);
void rustling_free_string(char* s);
void rustling_init(void);
char* rustling_version(void);
```

**目录约定：**
- Android .so 产物：`android/jniLibs/{arm64-v8a,armeabi-v7a,x86_64}/librustling.so`
- Python 包：`python/rustling/`
- 构建脚本：`scripts/`

---

## Task 1: 创建一键安装工具链脚本

**Files:**
- Create: `scripts/install_android_toolchain.sh`

**Step 1: 创建脚本文件**

```bash
cat > scripts/install_android_toolchain.sh << 'SCRIPT'
#!/usr/bin/env bash
# install_android_toolchain.sh — 安装 Android 交叉编译工具链
# 可重复运行（幂等）
set -euo pipefail

RUSTUP="$HOME/.cargo/bin/rustup"
CARGO="$HOME/.cargo/bin/cargo"

echo "=== [1/5] 检查/安装 rustup ==="
if command -v rustup &>/dev/null || [ -x "$RUSTUP" ]; then
    echo "    rustup 已安装，跳过"
else
    echo "    brew install rustup..."
    brew install rustup
    # 初始化 rustup，不修改 shell 配置文件（与 Homebrew Rust 并存）
    rustup-init -y --no-modify-path --default-toolchain stable
fi
echo "    rustup: $("$RUSTUP" --version 2>/dev/null || rustup --version)"

echo ""
echo "=== [2/5] 添加 Android Rust targets ==="
for TARGET in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android; do
    if "$RUSTUP" target list --installed | grep -q "^$TARGET$"; then
        echo "    $TARGET 已安装"
    else
        echo "    添加 $TARGET..."
        "$RUSTUP" target add "$TARGET"
    fi
done

echo ""
echo "=== [3/5] 检查/安装 Android NDK ==="
if brew list android-ndk &>/dev/null; then
    echo "    android-ndk 已安装"
else
    echo "    brew install android-ndk..."
    brew install android-ndk
fi
NDK_HOME="$(brew --prefix android-ndk)"
echo "    NDK 路径: $NDK_HOME"

echo ""
echo "=== [4/5] 检查/安装 cargo-ndk ==="
if "$CARGO" install --list 2>/dev/null | grep -q "^cargo-ndk"; then
    echo "    cargo-ndk 已安装"
else
    echo "    cargo install cargo-ndk..."
    "$CARGO" install cargo-ndk
fi
echo "    cargo-ndk: $("$HOME/.cargo/bin/cargo-ndk" --version 2>/dev/null || echo '已安装')"

echo ""
echo "=== [5/5] 环境变量提示 ==="
echo ""
echo "  在构建脚本中或 ~/.zshrc 中添加："
echo "  export ANDROID_NDK_HOME=\"$NDK_HOME\""
echo ""
echo "=== 工具链安装完成 ==="
echo ""
echo "下一步: ./scripts/build_android.sh"
SCRIPT

chmod +x scripts/install_android_toolchain.sh
```

**Step 2: 运行脚本安装工具链**

```bash
./scripts/install_android_toolchain.sh
```

期望输出（最后几行）：
```
=== [5/5] 环境变量提示 ===

  在构建脚本中或 ~/.zshrc 中添加：
  export ANDROID_NDK_HOME="/opt/homebrew/opt/android-ndk"

=== 工具链安装完成 ===
```

**Step 3: 验证安装**

```bash
~/.cargo/bin/rustup target list --installed | grep android
```

期望输出（三行，顺序不限）：
```
aarch64-linux-android
armv7-linux-androideabi
x86_64-linux-android
```

**Step 4: 提交**

```bash
git add scripts/install_android_toolchain.sh
git commit -m "feat(android): add Android toolchain installation script"
```

---

## Task 2: 创建 Android 构建脚本

**Files:**
- Create: `scripts/build_android.sh`

**Step 1: 创建构建脚本**

```bash
cat > scripts/build_android.sh << 'SCRIPT'
#!/usr/bin/env bash
# build_android.sh — 用 cargo-ndk 编译 Android librustling.so
#
# 用法:
#   ./scripts/build_android.sh           # 编译三大 ABI release 版
#   ./scripts/build_android.sh --debug   # 编译 debug 版
#   ./scripts/build_android.sh --verify  # 编译后验证 ELF 格式

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

CARGO="$HOME/.cargo/bin/cargo"
CARGO_NDK="$HOME/.cargo/bin/cargo-ndk"

# ── 参数解析 ─────────────────────────────────────────────────────────────
BUILD_TYPE="release"
VERIFY=false
for arg in "$@"; do
    case "$arg" in
        --debug)  BUILD_TYPE="debug" ;;
        --verify) VERIFY=true ;;
        *) echo "未知参数: $arg"; exit 1 ;;
    esac
done

# ── NDK 路径推断 ──────────────────────────────────────────────────────────
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    if brew list android-ndk &>/dev/null; then
        export ANDROID_NDK_HOME="$(brew --prefix android-ndk)"
        echo "提示: ANDROID_NDK_HOME 未设置，自动推断为 $ANDROID_NDK_HOME"
    else
        echo "错误: 请设置 ANDROID_NDK_HOME 环境变量"
        echo "      运行 ./scripts/install_android_toolchain.sh 安装 NDK"
        exit 1
    fi
fi

# ── 检查工具 ──────────────────────────────────────────────────────────────
if [ ! -x "$CARGO_NDK" ]; then
    echo "错误: cargo-ndk 未安装"
    echo "      运行 $CARGO install cargo-ndk"
    exit 1
fi

# ── 编译 ──────────────────────────────────────────────────────────────────
OUTPUT_DIR="$PROJECT_ROOT/android/jniLibs"
echo "=== 编译 Android librustling.so ($BUILD_TYPE) ==="
echo "    NDK: $ANDROID_NDK_HOME"
echo "    输出: $OUTPUT_DIR"
echo ""

BUILD_FLAGS="--release"
[ "$BUILD_TYPE" = "debug" ] && BUILD_FLAGS=""

"$CARGO_NDK" \
    -t arm64-v8a \
    -t armeabi-v7a \
    -t x86_64 \
    -o "$OUTPUT_DIR" \
    build $BUILD_FLAGS --lib

echo ""
echo "=== 产物 ==="
find "$OUTPUT_DIR" -name "*.so" | sort | while read -r f; do
    SIZE=$(du -sh "$f" | cut -f1)
    echo "    $SIZE  $f"
done

# ── 验证 ELF 格式 ─────────────────────────────────────────────────────────
if [ "$VERIFY" = true ]; then
    echo ""
    echo "=== ELF 格式验证 ==="
    OK=true
    file "$OUTPUT_DIR/arm64-v8a/librustling.so" | grep -q "ARM aarch64" \
        && echo "    ✓ arm64-v8a: ARM aarch64" \
        || { echo "    ✗ arm64-v8a: 格式异常"; OK=false; }

    file "$OUTPUT_DIR/armeabi-v7a/librustling.so" | grep -q "ARM" \
        && echo "    ✓ armeabi-v7a: ARM" \
        || { echo "    ✗ armeabi-v7a: 格式异常"; OK=false; }

    file "$OUTPUT_DIR/x86_64/librustling.so" | grep -q "x86-64" \
        && echo "    ✓ x86_64: x86-64" \
        || { echo "    ✗ x86_64: 格式异常"; OK=false; }

    if [ "$OK" = true ]; then
        echo ""
        echo "所有 .so 文件格式正确。"
    else
        echo ""
        echo "验证失败，请检查上方错误信息。"
        exit 1
    fi
fi

echo ""
echo "=== 构建完成 ==="
echo "    将 android/jniLibs/ 整个目录复制到消费方 Android 项目中。"
SCRIPT

chmod +x scripts/build_android.sh
```

**Step 2: 运行构建并验证**

```bash
./scripts/build_android.sh --verify
```

期望输出（关键部分）：
```
=== 产物 ===
    X.XM  android/jniLibs/arm64-v8a/librustling.so
    X.XM  android/jniLibs/armeabi-v7a/librustling.so
    X.XM  android/jniLibs/x86_64/librustling.so

=== ELF 格式验证 ===
    ✓ arm64-v8a: ARM aarch64
    ✓ armeabi-v7a: ARM
    ✓ x86_64: x86-64
```

**Step 3: 提交**

```bash
git add scripts/build_android.sh android/jniLibs/
git commit -m "feat(android): add cargo-ndk build script and compiled Android .so files"
```

---

## Task 3: 创建 Python ctypes 包装层

**Files:**
- Create: `python/rustling/__init__.py`
- Create: `python/rustling/ffi.py`

**背景：** `FfiParseResult` 在 `src/ffi.rs:68-75` 定义为：
```rust
pub struct FfiParseResult {
    pub json: *mut c_char,   // offset 0: char*
    pub count: u32,          // offset 8: uint32
    pub error: *mut c_char,  // offset 16: char*
}
```
Python ctypes 的 `_fields_` 必须与这个内存布局完全对应。

**Step 1: 创建目录**

```bash
mkdir -p python/rustling
```

**Step 2: 创建 `python/rustling/ffi.py`**

```python
# python/rustling/ffi.py
"""
ctypes wrapper around librustling C FFI.

对应 src/ffi.rs 中的 FfiParseResult struct 和导出函数。
不依赖 PyO3，在 Chaquopy 的 Python 环境中直接可用。
"""
import ctypes
import json
import os
import sys


class _FfiParseResult(ctypes.Structure):
    """
    Mirror of FfiParseResult in src/ffi.rs:
        pub json:  *mut c_char   (char*)
        pub count: u32
        pub error: *mut c_char   (char*)
    """
    _fields_ = [
        ("json",  ctypes.c_char_p),   # char* — JSON array (NULL on error)
        ("count", ctypes.c_uint32),   # u32   — number of matches
        ("error", ctypes.c_char_p),   # char* — error string (NULL on success)
    ]


def _load_library() -> ctypes.CDLL:
    """
    Load librustling from the correct path for the current platform.

    Android:  .so is loaded by the JVM via System.loadLibrary("rustling");
              pass None to access global symbol table.
    macOS:    librustling.dylib in the same directory as this file.
    Linux:    librustling.so in the same directory as this file.
    Windows:  rustling.dll in the same directory as this file.
    """
    if "ANDROID_DATA" in os.environ:
        # Android: JVM has already loaded the library into the process
        lib = ctypes.CDLL(None)
    else:
        _dir = os.path.dirname(os.path.abspath(__file__))
        _name = {
            "darwin": "librustling.dylib",
            "win32":  "rustling.dll",
        }.get(sys.platform, "librustling.so")
        _path = os.path.join(_dir, _name)
        if not os.path.exists(_path):
            raise FileNotFoundError(
                f"librustling not found at {_path}. "
                "Run 'cargo build --release --lib' and copy the .so/.dylib here."
            )
        lib = ctypes.CDLL(_path)

    # rustling_parse(text: *const c_char, locale: *const c_char) -> FfiParseResult
    lib.rustling_parse.restype  = _FfiParseResult
    lib.rustling_parse.argtypes = [ctypes.c_char_p, ctypes.c_char_p]

    # rustling_free_result(result: FfiParseResult) -> void
    lib.rustling_free_result.restype  = None
    lib.rustling_free_result.argtypes = [_FfiParseResult]

    # rustling_init() -> void  (optional warm-up)
    lib.rustling_init.restype  = None
    lib.rustling_init.argtypes = []

    lib.rustling_init()
    return lib


_lib: ctypes.CDLL = _load_library()


def parse(text: str, locale: str = "en") -> list:
    """
    解析自然语言文本，返回归一化结果列表。

    Args:
        text:   输入文本，例如 "5 minutes from now"、"明天下午三点"
        locale: BCP-47 语言代码，例如 "en"、"zh"、"fr"

    Returns:
        解析结果列表，每项为 dict，包含：
            value     (dict)  — 解析值，如 {"Integer": 42} / {"Duration": {...}}
            byte_start (int)  — 匹配起始字节偏移
            byte_end   (int)  — 匹配结束字节偏移
            char_start (int)  — 匹配起始字符偏移
            char_end   (int)  — 匹配结束字符偏移

    Raises:
        RuntimeError: locale 不支持或解析失败时抛出，携带 Rust 侧错误信息
    """
    r = _lib.rustling_parse(text.encode("utf-8"), locale.encode("utf-8"))

    if r.error:
        msg = r.error.decode("utf-8", errors="replace")
        _lib.rustling_free_result(r)
        raise RuntimeError(f"rustling_parse failed: {msg}")

    data = json.loads(r.json.decode("utf-8")) if r.json else []
    _lib.rustling_free_result(r)
    return data
```

**Step 3: 创建 `python/rustling/__init__.py`**

```python
# python/rustling/__init__.py
from .ffi import parse

__all__ = ["parse"]
```

**Step 4: 验证文件存在**

```bash
ls python/rustling/
```

期望：
```
__init__.py  ffi.py
```

**Step 5: 提交**

```bash
git add python/rustling/__init__.py python/rustling/ffi.py
git commit -m "feat(python): add ctypes wrapper for rustling C FFI"
```

---

## Task 4: 桌面验证 Python 包装

> 在 Android 上测试之前，先在桌面（macOS）验证 Python wrapper 与本地 .so 对接正确。

**Files:**
- Create: `python/rustling/ffi_test.py`（验证用，不入生产）

**Step 1: 构建桌面版动态库**

```bash
cargo build --release --lib
```

期望（最后一行）：
```
Finished `release` profile [optimized] target(s) in ...
```

**Step 2: 将 .dylib 软链到 Python 包目录**

```bash
ln -sf "$(pwd)/target/release/librustling.dylib" python/rustling/librustling.dylib
```

**Step 3: 创建验证脚本 `python/rustling/ffi_test.py`**

```python
#!/usr/bin/env python3
"""桌面验证：Python ctypes wrapper 能否正确调用 librustling"""
import sys
import os

# 将 python/ 目录加入模块搜索路径
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", ".."))

from rustling import parse

def test_integer():
    result = parse("42", "en")
    assert len(result) > 0, f"期望至少 1 个结果，实际: {result}"
    value = result[0]["value"]
    assert isinstance(value, dict), f"value 应为 dict，实际: {type(value)}"
    print(f"  ✓ integer: {value}")

def test_duration():
    result = parse("5 minutes", "en")
    assert len(result) > 0
    value = result[0]["value"]
    assert "Duration" in value, f"期望 Duration，实际: {value}"
    dur = value["Duration"]
    assert dur["amount"] == 5
    assert dur["unit"] == "Minute"
    print(f"  ✓ duration: {value}")

def test_empty():
    result = parse("", "en")
    assert result == [], f"空文本应返回空列表，实际: {result}"
    print("  ✓ empty text -> []")

def test_unsupported_locale():
    try:
        parse("42", "xx")
        assert False, "期望 RuntimeError"
    except RuntimeError as e:
        print(f"  ✓ unsupported locale -> RuntimeError: {e}")

if __name__ == "__main__":
    print("=== Python ctypes wrapper 验证 ===")
    test_integer()
    test_duration()
    test_empty()
    test_unsupported_locale()
    print("\n所有测试通过。")
```

**Step 4: 运行验证**

```bash
python python/rustling/ffi_test.py
```

期望输出：
```
=== Python ctypes wrapper 验证 ===
  ✓ integer: {'Integer': 42}
  ✓ duration: {'Duration': {'amount': 5, 'unit': 'Minute'}}
  ✓ empty text -> []
  ✓ unsupported locale -> RuntimeError: rustling_parse failed: Unsupported locale: xx

所有测试通过。
```

**Step 5: 提交**

```bash
git add python/rustling/ffi_test.py
git commit -m "test(python): add desktop validation script for ctypes wrapper"
```

---

## Task 5: 验证 Android .so 导出符号

> 确认 Android .so 中包含所有必要的导出符号（不需要真实 Android 设备）。

**Step 1: 检查 arm64 .so 的导出符号**

```bash
nm -D android/jniLibs/arm64-v8a/librustling.so | grep -E "T rustling_"
```

期望（包含这几行，顺序不限）：
```
... T rustling_free_error
... T rustling_free_result
... T rustling_free_string
... T rustling_init
... T rustling_locale_supported
... T rustling_parse
... T rustling_supported_locales
... T rustling_version
```

**Step 2: 确认无 undefined 符号（除标准 Android 库）**

```bash
nm -D android/jniLibs/arm64-v8a/librustling.so | grep " U " | grep -v "libdl\|libm\|libc\|pthread\|android"
```

期望：无输出（所有非系统符号均已解析）。

**Step 3: 提交最终状态（如有新文件）**

```bash
git add android/jniLibs/
git status
```

若 `git status` 显示 `nothing to commit`，则跳过 commit。

---

## 验证清单（完成标准）

- [ ] `scripts/install_android_toolchain.sh` 运行成功，工具链就位
- [ ] `scripts/build_android.sh --verify` 输出三个 ✓
- [ ] `python/rustling/ffi_test.py` 四个测试全部通过
- [ ] `nm -D android/jniLibs/arm64-v8a/librustling.so | grep "T rustling_parse"` 有输出

---

## 后续（不在本计划范围内）

- 在真实 Android 设备上通过 Chaquopy 验证完整 pipeline
- 添加 `python/rustling/ffi_test.py` 到 CI 流程
- 考虑用 GitHub Actions 自动化 Android .so 构建

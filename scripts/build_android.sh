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
    # android-ndk is installed as a Homebrew cask.
    # On macOS the cask wraps the NDK inside a .app bundle:
    #   <caskroom>/android-ndk/<version>/<Name>.app/Contents/NDK/
    NDK_HOME=""
    if brew --caskroom &>/dev/null; then
        CASKROOM="$(brew --caskroom)"
        NDK_VERSIONED="$(ls -d "$CASKROOM/android-ndk/"* 2>/dev/null | tail -1)"
        if [ -d "$NDK_VERSIONED" ]; then
            # Look for a .app bundle containing the NDK
            APP_BUNDLE="$(ls -d "$NDK_VERSIONED"/*.app 2>/dev/null | head -1)"
            if [ -d "$APP_BUNDLE/Contents/NDK" ]; then
                NDK_HOME="$APP_BUNDLE/Contents/NDK"
            else
                NDK_HOME="$NDK_VERSIONED"
            fi
        fi
    fi
    if [ -z "$NDK_HOME" ] || [ ! -d "$NDK_HOME" ]; then
        NDK_HOME="/opt/homebrew/share/android-ndk"
    fi
    export ANDROID_NDK_HOME="$NDK_HOME"
    echo "提示: ANDROID_NDK_HOME 未设置，自动推断为 $ANDROID_NDK_HOME"
fi

# ── 检查工具 ──────────────────────────────────────────────────────────────
if [ ! -x "$CARGO" ]; then
    echo "错误: rustup cargo 未找到（$CARGO）"
    echo "      运行 ./scripts/install_android_toolchain.sh 安装工具链"
    exit 1
fi

if ! "$CARGO" ndk --version &>/dev/null; then
    echo "错误: cargo-ndk 未安装"
    echo "      运行 $CARGO install cargo-ndk"
    exit 1
fi

# ── 编译 ──────────────────────────────────────────────────────────────────
OUTPUT_DIR="$PROJECT_ROOT/android/jniLibs"
mkdir -p "$OUTPUT_DIR"

echo "=== 编译 Android librustling.so ($BUILD_TYPE) ==="
echo "    NDK: $ANDROID_NDK_HOME"
echo "    输出: $OUTPUT_DIR"
echo ""

BUILD_FLAGS=""
[ "$BUILD_TYPE" = "release" ] && BUILD_FLAGS="--release"

"$CARGO" ndk \
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

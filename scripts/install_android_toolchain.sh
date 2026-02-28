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
if brew list --cask android-ndk &>/dev/null; then
    echo "    android-ndk 已安装"
else
    echo "    brew install --cask android-ndk..."
    brew install --cask android-ndk
fi
# android-ndk is a cask; resolve its actual install path
NDK_HOME=""
# Try brew --caskroom (works on some Homebrew versions)
if brew --caskroom &>/dev/null; then
    CASKROOM="$(brew --caskroom)"
    NDK_VERSIONED="$(ls -d "$CASKROOM/android-ndk/"* 2>/dev/null | tail -1)"
    [ -d "$NDK_VERSIONED" ] && NDK_HOME="$NDK_VERSIONED"
fi
# Fallback: well-known Homebrew share path (cask symlink or direct install)
if [ -z "$NDK_HOME" ] || [ ! -d "$NDK_HOME" ]; then
    NDK_HOME="/opt/homebrew/share/android-ndk"
fi
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

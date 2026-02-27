#!/usr/bin/env bash
# build_ffi_example.sh — Build and optionally run the FFI C example
#
# Usage:
#   ./scripts/build_ffi_example.sh          # build only
#   ./scripts/build_ffi_example.sh --run    # build and run
#   ./scripts/build_ffi_example.sh --debug  # build debug library

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# ── Defaults ───────────────────────────────────────────────────────────────
BUILD_TYPE="release"
RUN_AFTER_BUILD=false

# ── Parse arguments ────────────────────────────────────────────────────────
for arg in "$@"; do
    case "$arg" in
        --run)   RUN_AFTER_BUILD=true ;;
        --debug) BUILD_TYPE="debug" ;;
        *) echo "Unknown argument: $arg"; exit 1 ;;
    esac
done

# ── Step 1: Build Rust library ─────────────────────────────────────────────
echo "=== Building Rust library ($BUILD_TYPE) ==="
if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release --lib
    LIB_DIR="$PROJECT_ROOT/target/release"
else
    cargo build --lib
    LIB_DIR="$PROJECT_ROOT/target/debug"
fi
echo ""

# ── Step 2: Detect OS and library extension ────────────────────────────────
OS="$(uname -s)"
case "$OS" in
    Linux*)
        LIB_EXT="so"
        LIB_PREFIX="lib"
        LD_VAR="LD_LIBRARY_PATH"
        ;;
    Darwin*)
        LIB_EXT="dylib"
        LIB_PREFIX="lib"
        LD_VAR="DYLD_LIBRARY_PATH"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        LIB_EXT="dll"
        LIB_PREFIX=""
        LD_VAR="PATH"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

LIB_FILE="$LIB_DIR/${LIB_PREFIX}rustling.$LIB_EXT"
if [ ! -f "$LIB_FILE" ]; then
    echo "ERROR: Library not found at $LIB_FILE"
    echo "       Did the Rust build succeed?"
    exit 1
fi
echo "Library: $LIB_FILE ($(du -sh "$LIB_FILE" | cut -f1))"

# ── Step 3: Compile the C example ─────────────────────────────────────────
OUTPUT_BIN="$PROJECT_ROOT/ffi_example"
echo ""
echo "=== Compiling C example ==="

CFLAGS="-Wall -Wextra -O2"
if [ "$BUILD_TYPE" = "debug" ]; then
    CFLAGS="-Wall -Wextra -g"
fi

gcc $CFLAGS \
    -o "$OUTPUT_BIN" \
    "$PROJECT_ROOT/examples/ffi_example.c" \
    -I"$PROJECT_ROOT/include" \
    -L"$LIB_DIR" \
    -lrustling \
    -lpthread -ldl -lm

echo "Binary:  $OUTPUT_BIN"
echo ""

# ── Step 4: Optionally run ─────────────────────────────────────────────────
if [ "$RUN_AFTER_BUILD" = true ]; then
    echo "=== Running FFI example ==="
    echo ""
    eval "$LD_VAR=$LIB_DIR ${OUTPUT_BIN}"
fi

echo ""
echo "Done.  To run manually:"
echo "  $LD_VAR=$LIB_DIR $OUTPUT_BIN"

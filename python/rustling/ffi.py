# python/rustling/ffi.py
"""
ctypes wrapper around librustling C FFI.

Mirrors FfiParseResult in src/ffi.rs and the exported C functions.
No PyO3 dependency — works in Chaquopy's Python environment on Android.
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
    Parse natural language text and return normalised results.

    Args:
        text:   Input text, e.g. "5 minutes from now" or "明天下午三点"
        locale: BCP-47 language code, e.g. "en", "zh", "fr"

    Returns:
        List of dicts, each containing:
            value      (dict) — parsed value, e.g. {"Integer": 42}
            byte_start (int)  — match start byte offset
            byte_end   (int)  — match end byte offset
            char_start (int)  — match start char offset
            char_end   (int)  — match end char offset

    Raises:
        RuntimeError: if locale is unsupported or parsing fails
    """
    r = _lib.rustling_parse(text.encode("utf-8"), locale.encode("utf-8"))

    if r.error:
        msg = r.error.decode("utf-8", errors="replace")
        _lib.rustling_free_result(r)
        raise RuntimeError(f"rustling_parse failed: {msg}")

    data = json.loads(r.json.decode("utf-8")) if r.json else []
    _lib.rustling_free_result(r)
    return data

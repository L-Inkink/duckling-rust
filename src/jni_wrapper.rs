//! JNI (Java Native Interface) bindings for Android integration
//!
//! This module exports JNI-compatible functions called from Kotlin/Java.
//! It delegates to the high-level [`crate::parse::Parser`] API, which returns
//! fully-typed, serialisable [`crate::parse::ParseOutput`] values.
//!
//! ## Kotlin usage
//!
//! ```kotlin
//! // In your Application or Activity:
//! NLPParser.init()                                    // pre-warm (optional)
//! val results = NLPParser.parse("5 minutes", "en")   // -> List<ParseResult>
//! ```
//!
//! ## Building for Android (arm64-v8a)
//!
//! ```bash
//! cargo build --release \
//!     --target aarch64-linux-android \
//!     --features jni \
//!     --lib
//! # Output: target_user/aarch64-linux-android/release/librustling.so
//! ```
//!
//! Copy the `.so` into your Android project's `app/src/main/jniLibs/arm64-v8a/`.

use std::panic;
use std::sync::OnceLock;

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

use crate::parse::Parser;

// ---------------------------------------------------------------------------
// Global parser instance (initialised on first use, never torn down)
// ---------------------------------------------------------------------------

static PARSER: OnceLock<Parser> = OnceLock::new();

fn get_parser() -> &'static Parser {
    PARSER.get_or_init(Parser::new)
}

// ---------------------------------------------------------------------------
// Helper: create a Java String from a Rust &str, returning null on failure
// ---------------------------------------------------------------------------

fn new_jstring(env: &mut JNIEnv<'_>, s: &str) -> jstring {
    match env.new_string(s) {
        Ok(js) => js.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// ---------------------------------------------------------------------------
// JNI exports
//
// Naming convention: Java_<package_underscored>_<ClassName>_<methodName>
//   package  : com.rustling  →  com_rustling
//   class    : NLPParser
//   methods  : nativeInit, nativeParse, nativeVersion,
//              nativeSupportedLocales, nativeIsLocaleSupported
// ---------------------------------------------------------------------------

/// Pre-warm the parser (load locale data, compile patterns).
///
/// Optional but reduces first-parse latency. Safe to call multiple times.
///
/// Kotlin signature (companion object, @JvmStatic):
/// ```kotlin
/// @JvmStatic private external fun nativeInit()
/// ```
#[no_mangle]
pub extern "system" fn Java_com_rustling_NLPParser_nativeInit(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
) {
    let _ = panic::catch_unwind(|| {
        let _ = get_parser();
    });
}

/// Parse text and return a JSON string matching [`crate::parse::ParseOutput`].
///
/// On success the JSON looks like:
/// ```json
/// {
///   "results": [
///     {"value": {"type":"integer","value":42},
///      "byte_start":0,"byte_end":2,"char_start":0,"char_end":2}
///   ],
///   "count": 1
/// }
/// ```
/// Returns Java `null` only if an internal panic occurred.
///
/// Kotlin signature:
/// ```kotlin
/// @JvmStatic private external fun nativeParse(text: String, locale: String): String?
/// ```
#[no_mangle]
pub extern "system" fn Java_com_rustling_NLPParser_nativeParse<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
    locale: JString<'local>,
) -> jstring {
    // Extract strings from JNI *before* catch_unwind — JNI types must not
    // cross the unwind boundary.
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };
    let locale_str: String = match env.get_string(&locale) {
        Ok(s) => s.into(),
        Err(_) => "en".to_string(),
    };

    let json_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let output = get_parser().parse(&text_str, Some(&locale_str));
        serde_json::to_string(&output).unwrap_or_else(|_| r#"{"results":[],"count":0}"#.to_string())
    }));

    match json_result {
        Ok(json) => new_jstring(&mut env, &json),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Return the library version string (e.g. `"0.10.0"`).
///
/// Kotlin signature:
/// ```kotlin
/// @JvmStatic private external fun nativeVersion(): String?
/// ```
#[no_mangle]
pub extern "system" fn Java_com_rustling_NLPParser_nativeVersion<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    new_jstring(&mut env, env!("CARGO_PKG_VERSION"))
}

/// Return a JSON array of all supported locale codes, e.g. `["en","fr","zh"]`.
///
/// Kotlin signature:
/// ```kotlin
/// @JvmStatic private external fun nativeSupportedLocales(): String?
/// ```
#[no_mangle]
pub extern "system" fn Java_com_rustling_NLPParser_nativeSupportedLocales<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let json_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let locales = get_parser().supported_locales();
        serde_json::to_string(&locales).unwrap_or_else(|_| "[]".to_string())
    }));

    match json_result {
        Ok(json) => new_jstring(&mut env, &json),
        Err(_) => new_jstring(&mut env, "[]"),
    }
}

/// Return `true` if the given locale code is supported.
///
/// Kotlin signature:
/// ```kotlin
/// @JvmStatic private external fun nativeIsLocaleSupported(locale: String): Boolean
/// ```
#[no_mangle]
pub extern "system" fn Java_com_rustling_NLPParser_nativeIsLocaleSupported<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    locale: JString<'local>,
) -> jboolean {
    let locale_str: String = match env.get_string(&locale) {
        Ok(s) => s.into(),
        Err(_) => return JNI_FALSE,
    };

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        get_parser().is_locale_supported(&locale_str)
    }));

    match result {
        Ok(true) => JNI_TRUE,
        _ => JNI_FALSE,
    }
}

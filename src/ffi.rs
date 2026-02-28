//! FFI (Foreign Function Interface) module for rustling
//!
//! This module provides C-compatible functions that can be called from
//! other programming languages (C, C++, Python, Java, etc.).
//!
//! ## Usage
//!
//! ```c
//! #include <stdio.h>
//! #include <stdlib.h>
//!
//! // Declare the functions
//! extern FfiParseResult rustling_parse(const char* text, const char* locale);
//! extern void rustling_free_result(FfiParseResult result);
//! extern void rustling_free_string(char* s);
//!
//! int main() {
//!     FfiParseResult r = rustling_parse("5 minutes", "en");
//!     printf("Result: %s\n", r.json);
//!     rustling_free_result(r);
//!     return 0;
//! }
//! ```
//!
//! ## Building
//!
//! ```bash
//! cargo build --release --lib
//! ```
//!
//! This produces:
//! - `target/release/librustling.so` (Linux)
//! - `target/release/librustling.dylib` (macOS)
//! - `target/release/librustling.a` (static library)
//! - `target/release/rustling.dll` (Windows)

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use std::ptr;
use std::sync::OnceLock;

use crate::fuzzy::PatternNormalizer;
use crate::locale::LocaleRegistry;

static LOCALE_REGISTRY: OnceLock<LocaleRegistry> = OnceLock::new();
static PATTERN_NORMALIZER: OnceLock<PatternNormalizer> = OnceLock::new();

/// Initialize the global locale registry
fn get_locale_registry() -> &'static LocaleRegistry {
    LOCALE_REGISTRY.get_or_init(LocaleRegistry::build_all)
}

/// Initialize the global pattern normalizer
fn get_pattern_normalizer() -> &'static PatternNormalizer {
    PATTERN_NORMALIZER.get_or_init(PatternNormalizer::new)
}

/// Allocate a C string from a Rust `String`. Returns null on internal error.
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s)
        .unwrap_or_else(|_| CString::new("<encoding error>").unwrap())
        .into_raw()
}

/// Parse result structure (C-compatible)
#[repr(C)]
pub struct FfiParseResult {
    /// JSON string containing the parse results (null on error)
    pub json: *mut c_char,
    /// Number of results
    pub count: u32,
    /// Error message (null on success)
    pub error: *mut c_char,
}

/// Parse a text string and return results as JSON.
///
/// # Arguments
/// * `text`   - The text to parse (UTF-8, null-terminated)
/// * `locale` - The locale code (e.g. "en", "fr", "zh"); defaults to "en" when null
///
/// # Returns
/// `FfiParseResult` with:
/// - `json`  : JSON array of match objects (caller must free with `rustling_free_result`)
/// - `count` : number of matches
/// - `error` : error description (non-null only on failure)
///
/// # Safety
/// `text` and `locale` must be valid null-terminated UTF-8 strings or null pointers.
#[no_mangle]
pub extern "C" fn rustling_parse(text: *const c_char, locale: *const c_char) -> FfiParseResult {
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        rustling_parse_inner(text, locale)
    }));

    match result {
        Ok(r) => r,
        Err(_) => FfiParseResult {
            json: ptr::null_mut(),
            count: 0,
            error: to_c_string("internal panic in rustling_parse".to_string()),
        },
    }
}

fn rustling_parse_inner(text: *const c_char, locale: *const c_char) -> FfiParseResult {
    if text.is_null() {
        return FfiParseResult {
            json: ptr::null_mut(),
            count: 0,
            error: to_c_string("text cannot be null".to_string()),
        };
    }

    let locale_str = if locale.is_null() {
        "en".to_string()
    } else {
        match unsafe { CStr::from_ptr(locale).to_str() } {
            Ok(s) => s.to_string(),
            Err(_) => {
                return FfiParseResult {
                    json: ptr::null_mut(),
                    count: 0,
                    error: to_c_string("locale is not valid UTF-8".to_string()),
                };
            }
        }
    };

    let text_str = match unsafe { CStr::from_ptr(text).to_str() } {
        Ok(s) => s,
        Err(_) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: to_c_string("text is not valid UTF-8".to_string()),
            };
        }
    };

    let registry = get_locale_registry();
    let normalizer = get_pattern_normalizer();

    let rule_set = match registry.get(&locale_str) {
        Some(rs) => rs,
        None => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: to_c_string(format!("Unsupported locale: {}", locale_str)),
            };
        }
    };

    let normalized = normalizer.normalize(text_str);

    let nodes = match rule_set.apply_all(&normalized) {
        Ok(nodes) => nodes,
        Err(e) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: to_c_string(format!("Parse error: {:?}", e)),
            };
        }
    };

    // Serialize value as a structured JSON object (Value derives Serialize)
    let results: Vec<serde_json::Value> = nodes
        .iter()
        .map(|n| {
            let byte_range = n.root_node.byte_range;
            let char_range = byte_range.char_range(text_str);
            let value_json = serde_json::to_value(&n.value)
                .unwrap_or(serde_json::Value::Null);
            serde_json::json!({
                "value": value_json,
                "byte_start": byte_range.0,
                "byte_end": byte_range.1,
                "char_start": char_range.0,
                "char_end": char_range.1,
            })
        })
        .collect();

    let count = results.len() as u32;

    let json = match serde_json::to_string(&results) {
        Ok(s) => s,
        Err(e) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: to_c_string(format!("JSON serialization error: {}", e)),
            };
        }
    };

    FfiParseResult {
        json: to_c_string(json),
        count,
        error: ptr::null_mut(),
    }
}

/// Free a string allocated by rustling (e.g. from `rustling_version` or
/// `rustling_supported_locales`).
///
/// # Safety
/// Must only be called with a pointer previously returned by a rustling function.
#[no_mangle]
pub unsafe extern "C" fn rustling_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

/// Free all memory owned by an `FfiParseResult`.
///
/// Call this instead of manually freeing `json` and `error` individually.
///
/// # Safety
/// Must only be called with a result returned by `rustling_parse`.
/// Must be called exactly once per result.
#[no_mangle]
pub unsafe extern "C" fn rustling_free_result(result: FfiParseResult) {
    if !result.json.is_null() {
        rustling_free_string(result.json);
    }
    if !result.error.is_null() {
        rustling_free_string(result.error);
    }
}

/// Free an error string allocated by rustling.
///
/// # Safety
/// Must only be called with an error pointer returned by a rustling function.
#[no_mangle]
pub unsafe extern "C" fn rustling_free_error(ptr: *mut c_char) {
    rustling_free_string(ptr);
}

/// Return the rustling version string (e.g. "0.10.0").
///
/// The caller must free the result with `rustling_free_string`.
#[no_mangle]
pub extern "C" fn rustling_version() -> *mut c_char {
    let result = panic::catch_unwind(|| {
        to_c_string(env!("CARGO_PKG_VERSION").to_string())
    });
    result.unwrap_or(ptr::null_mut())
}

/// Return supported locales as a JSON array string (e.g. `["en","fr","zh"]`).
///
/// The caller must free the result with `rustling_free_string`.
#[no_mangle]
pub extern "C" fn rustling_supported_locales() -> *mut c_char {
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let registry = get_locale_registry();
        let locales_refs: Vec<&str> = registry.supported_locales();
        let locales: Vec<String> = locales_refs.into_iter().map(|s| s.to_string()).collect();
        let json = serde_json::to_string(&locales).unwrap_or_else(|_| "[]".to_string());
        // Use CString::into_raw so rustling_free_string (CString::from_raw) is safe to call
        to_c_string(json)
    }));
    result.unwrap_or(ptr::null_mut())
}

/// Return 1 if the given locale is supported, 0 otherwise.
#[no_mangle]
pub extern "C" fn rustling_locale_supported(locale: *const c_char) -> u32 {
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        if locale.is_null() {
            return 0u32;
        }
        let locale_str = match unsafe { CStr::from_ptr(locale).to_str() } {
            Ok(s) => s,
            Err(_) => return 0,
        };
        let registry = get_locale_registry();
        if registry.get(locale_str).is_some() { 1 } else { 0 }
    }));
    result.unwrap_or(0)
}

/// Pre-warm the library to reduce first-call latency. Optional but recommended.
#[no_mangle]
pub extern "C" fn rustling_init() {
    let _ = panic::catch_unwind(|| {
        let _ = get_locale_registry();
        let _ = get_pattern_normalizer();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version_ptr = rustling_version();
        assert!(!version_ptr.is_null());
        let version = unsafe { CStr::from_ptr(version_ptr) }.to_str().unwrap();
        assert!(!version.is_empty());
        unsafe { rustling_free_string(version_ptr); }
    }

    #[test]
    fn test_supported_locales() {
        rustling_init();
        let locales_ptr = rustling_supported_locales();
        assert!(!locales_ptr.is_null());
        let locales_json = unsafe { CStr::from_ptr(locales_ptr) }.to_str().unwrap();
        let locales: Vec<String> = serde_json::from_str(locales_json).unwrap();
        assert!(locales.contains(&"en".to_string()));
        assert!(locales.contains(&"fr".to_string()));
        unsafe { rustling_free_string(locales_ptr); }
    }

    #[test]
    fn test_locale_supported() {
        assert_eq!(rustling_locale_supported(std::ptr::null()), 0);

        let en = CString::new("en").unwrap();
        let xx = CString::new("xx").unwrap();

        assert_eq!(rustling_locale_supported(en.as_ptr()), 1);
        assert_eq!(rustling_locale_supported(xx.as_ptr()), 0);
    }

    #[test]
    fn test_parse_null_text() {
        let result = rustling_parse(std::ptr::null(), std::ptr::null());
        assert!(result.json.is_null());
        assert!(!result.error.is_null());
        assert_eq!(result.count, 0);
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_parse_empty_text() {
        let empty = CString::new("").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(empty.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert_eq!(result.count, 0);
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_parse_integer() {
        let text = CString::new("42").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert!(result.count > 0);
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_parse_duration() {
        let text = CString::new("5 minutes").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert!(result.count > 0);

        let json_str = unsafe { CStr::from_ptr(result.json) }.to_str().unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap();
        assert!(!parsed.is_empty());
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_parse_unsupported_locale() {
        let text = CString::new("42").unwrap();
        let xx = CString::new("xx").unwrap();

        let result = rustling_parse(text.as_ptr(), xx.as_ptr());
        assert!(result.json.is_null());
        assert!(!result.error.is_null());
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_init() {
        rustling_init();
    }

    // --- Tests for bugs to be fixed ---

    /// rustling_free_result must safely release both json and error fields
    #[test]
    fn test_free_result_on_success() {
        let text = CString::new("42").unwrap();
        let en = CString::new("en").unwrap();
        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        // Must not crash or leak
        unsafe { rustling_free_result(result); }
    }

    #[test]
    fn test_free_result_on_error() {
        let result = rustling_parse(std::ptr::null(), std::ptr::null());
        assert!(result.json.is_null());
        assert!(!result.error.is_null());
        // Must not crash or leak
        unsafe { rustling_free_result(result); }
    }

    /// value field in parse results must be a structured JSON object, not a Debug string
    #[test]
    fn test_parse_value_is_structured_json_object() {
        let text = CString::new("42").unwrap();
        let en = CString::new("en").unwrap();
        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());

        let json_str = unsafe { CStr::from_ptr(result.json) }.to_str().unwrap();
        let items: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap();
        assert!(!items.is_empty());

        // value must be a JSON object, not a raw Debug string like "Integer(42)"
        let value = &items[0]["value"];
        assert!(
            value.is_object(),
            "expected value to be a JSON object but got: {:?}",
            value
        );

        unsafe { rustling_free_result(result); }
    }

    /// rustling_supported_locales result must be safely freeable with rustling_free_string
    #[test]
    fn test_supported_locales_freeable() {
        rustling_init();
        let ptr = rustling_supported_locales();
        assert!(!ptr.is_null());
        let json = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
        let locales: Vec<String> = serde_json::from_str(json).unwrap();
        assert!(locales.contains(&"en".to_string()));
        // This must NOT be UB - requires CString allocation, not Vec::as_ptr
        unsafe { rustling_free_string(ptr); }
    }
}

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
//! extern char* rustling_parse(const char* text, const char* locale);
//! extern void rustling_free(char* result);
//!
//! int main() {
//!     char* result = rustling_parse("5 minutes", "en");
//!     printf("Result: %s\n", result);
//!     rustling_free(result);
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
//! - `target/release/librustling.so` (Linux/macOS)
//! - `target/release/librustling.a` (static library)
//! - `target/release/librustling.dll` (Windows)

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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

/// Parse result structure (C-compatible)
#[repr(C)]
pub struct FfiParseResult {
    /// JSON string containing the parse results
    pub json: *mut c_char,
    /// Number of results
    pub count: u32,
    /// Error message (null if no error)
    pub error: *mut c_char,
}

/// Parse a text string and return results as JSON
///
/// # Arguments
/// * `text` - The text to parse (UTF-8 encoded)
/// * `locale` - The locale code (e.g., "en", "fr", "zh")
///
/// # Returns
/// FfiParseResult containing:
/// - json: JSON string with parse results (must be freed with rustling_free)
/// - count: Number of results
/// - error: Error message if any (null if successful)
#[no_mangle]
pub extern "C" fn rustling_parse(text: *const c_char, locale: *const c_char) -> FfiParseResult {
    // Check for null pointers
    if text.is_null() {
        return FfiParseResult {
            json: ptr::null_mut(),
            count: 0,
            error: CString::new("text cannot be null").unwrap().into_raw(),
        };
    }

    let locale_str = if locale.is_null() {
        "en".to_string()
    } else {
        // Safety: we trust the caller to provide valid UTF-8
        match unsafe { CStr::from_ptr(locale).to_str() } {
            Ok(s) => s.to_string(),
            Err(_) => {
                return FfiParseResult {
                    json: ptr::null_mut(),
                    count: 0,
                    error: CString::new("locale is not valid UTF-8").unwrap().into_raw(),
                };
            }
        }
    };

    // Safety: we trust the caller to provide valid UTF-8
    let text_str = match unsafe { CStr::from_ptr(text).to_str() } {
        Ok(s) => s,
        Err(_) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: CString::new("text is not valid UTF-8").unwrap().into_raw(),
            };
        }
    };

    // Get the locale registry and pattern normalizer
    let registry = get_locale_registry();
    let normalizer = get_pattern_normalizer();

    // Get the rule set for the locale
    let rule_set = match registry.get(&locale_str) {
        Some(rs) => rs,
        None => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: CString::new(format!("Unsupported locale: {}", locale_str))
                    .unwrap()
                    .into_raw(),
            };
        }
    };

    // Normalize the text
    let normalized = normalizer.normalize(text_str);

    // Parse the text
    let nodes = match rule_set.apply_all(&normalized) {
        Ok(nodes) => nodes,
        Err(e) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: CString::new(format!("Parse error: {:?}", e)).unwrap().into_raw(),
            };
        }
    };

    // Convert results to JSON
    let results: Vec<serde_json::Value> = nodes
        .iter()
        .map(|n| {
            let byte_range = n.root_node.byte_range;
            let char_range = byte_range.char_range(text_str);
            serde_json::json!({
                "value": format!("{:?}", n.value),
                "byte_start": byte_range.0,
                "byte_end": byte_range.1,
                "char_start": char_range.0,
                "char_end": char_range.1,
            })
        })
        .collect();

    let count = results.len() as u32;

    // Serialize to JSON
    let json = match serde_json::to_string(&results) {
        Ok(s) => s,
        Err(e) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: CString::new(format!("JSON serialization error: {}", e))
                    .unwrap()
                    .into_raw(),
            };
        }
    };

    // Convert to C string
    let json_cstring = match CString::new(json) {
        Ok(s) => s,
        Err(e) => {
            return FfiParseResult {
                json: ptr::null_mut(),
                count: 0,
                error: CString::new(format!("CString conversion error: {}", e))
                    .unwrap()
                    .into_raw(),
            };
        }
    };

    FfiParseResult {
        json: json_cstring.into_raw(),
        count,
        error: ptr::null_mut(),
    }
}

/// Free a string allocated by rustling
///
/// # Arguments
/// * `ptr` - Pointer to the string to free
///
/// # Safety
/// This function must only be called with pointers returned by rustling functions.
#[no_mangle]
pub unsafe extern "C" fn rustling_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        // Safety: we own this pointer
        let _ = CString::from_raw(ptr);
    }
}

/// Free an error message allocated by rustling
///
/// # Arguments
/// * `ptr` - Pointer to the error string to free
///
/// # Safety
/// This function must only be called with error pointers returned by rustling functions.
#[no_mangle]
pub unsafe extern "C" fn rustling_free_error(ptr: *mut c_char) {
    rustling_free_string(ptr);
}

/// Get the version of rustling
///
/// # Returns
/// Version string (e.g., "0.10.0")
#[no_mangle]
pub extern "C" fn rustling_version() -> *mut c_char {
    CString::new(env!("CARGO_PKG_VERSION"))
        .expect("Invalid version string")
        .into_raw()
}

/// Get supported locales as JSON
///
/// # Returns
/// JSON array of supported locale codes
#[no_mangle]
pub extern "C" fn rustling_supported_locales() -> *mut c_char {
    let registry = get_locale_registry();
    let locales_refs: Vec<&str> = registry.supported_locales();
    let locales: Vec<String> = locales_refs.into_iter().map(|s| s.to_string()).collect();
    let json = serde_json::to_string(&locales).unwrap_or_else(|_| "[]".to_string());

    // Create a Vec with the string plus null terminator
    let mut bytes = json.into_bytes();
    bytes.push(0); // null terminator

    // Get pointer before leaking
    let ptr = bytes.as_ptr() as *mut c_char;

    // Leak the Vec so it doesn't get freed
    std::mem::forget(bytes);

    ptr
}

/// Check if a locale is supported
///
/// # Arguments
/// * `locale` - The locale code to check
///
/// # Returns
/// 1 if supported, 0 otherwise
#[no_mangle]
pub extern "C" fn rustling_locale_supported(locale: *const c_char) -> u32 {
    if locale.is_null() {
        return 0;
    }

    let locale_str = match unsafe { CStr::from_ptr(locale).to_str() } {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let registry = get_locale_registry();
    if registry.get(locale_str).is_some() {
        1
    } else {
        0
    }
}

/// Initialize the FFI library
///
/// This can be called to pre-warm the library before any parse calls.
/// It's optional but can improve first-call latency.
#[no_mangle]
pub extern "C" fn rustling_init() {
    let _ = get_locale_registry();
    let _ = get_pattern_normalizer();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = unsafe { CStr::from_ptr(rustling_version()) }
            .to_str()
            .unwrap();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_supported_locales() {
        // Initialize the FFI library first
        rustling_init();

        let locales_ptr = rustling_supported_locales();

        // Read the string BEFORE freeing
        let locales_json = unsafe { CStr::from_ptr(locales_ptr) }
            .to_str()
            .unwrap();

        // Parse JSON BEFORE freeing
        let locales: Vec<String> = serde_json::from_str(locales_json).unwrap();
        assert!(locales.contains(&"en".to_string()));
        assert!(locales.contains(&"fr".to_string()));

        // Free AFTER we're done using the string
        unsafe { rustling_free_string(locales_ptr); }
    }

    #[test]
    fn test_locale_supported() {
        assert_eq!(rustling_locale_supported(std::ptr::null()), 0);
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

        // Free the error
        unsafe { rustling_free_error(result.error); }
    }

    #[test]
    fn test_parse_empty_text() {
        let empty = CString::new("").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(empty.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert_eq!(result.count, 0);

        unsafe { rustling_free_string(result.json); }
    }

    #[test]
    fn test_parse_integer() {
        let text = CString::new("42").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert!(result.count > 0);

        unsafe { rustling_free_string(result.json); }
    }

    #[test]
    fn test_parse_duration() {
        let text = CString::new("5 minutes").unwrap();
        let en = CString::new("en").unwrap();

        let result = rustling_parse(text.as_ptr(), en.as_ptr());
        assert!(!result.json.is_null());
        assert!(result.error.is_null());
        assert!(result.count > 0);

        // Parse the JSON to verify content
        let json_str = unsafe { CStr::from_ptr(result.json) }
            .to_str()
            .unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap();
        assert!(!parsed.is_empty());

        unsafe { rustling_free_string(result.json); }
    }

    #[test]
    fn test_parse_unsupported_locale() {
        let text = CString::new("42").unwrap();
        let xx = CString::new("xx").unwrap();

        let result = rustling_parse(text.as_ptr(), xx.as_ptr());
        assert!(result.json.is_null());
        assert!(!result.error.is_null());

        unsafe { rustling_free_error(result.error); }
    }

    #[test]
    fn test_init() {
        rustling_init();
    }
}

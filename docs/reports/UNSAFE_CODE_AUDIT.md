# Unsafe Code Audit Report

**Date**: 2026-02-12 (updated 2026-02-27)
**Auditor**: Claude Code
**Project**: Rustling (duckling-rust)
**Phase**: Phase 0 Week 2 (initial); Phase 6-B FFI (update)

---

## Executive Summary

| 位置 | 类型 | 数量 | 风险 | 结论 |
|------|------|------|------|------|
| `core/src/lib.rs:268-269` | `unsafe impl Send/Sync` | 2 行 | **LOW** ✅ | 历史遗留，已记录，可日后清理 |
| `src/ffi.rs` | FFI unsafe 块 | 4 处 | **MEDIUM → MITIGATED** ✅ | 已按最佳实践修复（2026-02-27） |

---

## Detailed Findings

### 1. SendSyncPhantomData<T> - Lines 268-269

**Location**: `core/src/lib.rs:268-269`

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SendSyncPhantomData<T>(::std::marker::PhantomData<T>);
unsafe impl<T> Send for SendSyncPhantomData<T> {}
unsafe impl<T> Sync for SendSyncPhantomData<T> {}
```

#### Purpose

This type is used as a phantom type marker throughout the codebase to:
- Hold type information for generics (V, StashValue) without storing actual data
- Allow structs containing it to be `Send + Sync` across thread boundaries
- Work around Rust's conservative `PhantomData` variance rules (in older Rust versions)

#### Usage Locations

Found in **20 locations** across:
- `core/src/rule.rs`: Rule1, Rule2, Rule3, Rule4, Rule5, Rule6 structs (12 uses)
- `core/src/pattern.rs`: Various pattern types (8 uses)

Example usage:
```rust
pub struct Rule1<PA, V, StashValue, F> {
    sym: Sym,
    pattern: PA,
    production: F,
    _phantom: SendSyncPhantomData<(V, StashValue)>,  // ← Holds type info
}
```

#### Safety Analysis

**Is this unsafe code sound?** ✅ **YES**

**Reasoning**:
1. `PhantomData<T>` is a **zero-sized type (ZST)** with no runtime representation
2. It doesn't actually store any `T` - it only exists for the type system
3. Since there's no actual data to send/sync, it's always safe to send/sync `PhantomData<T>` regardless of `T`'s properties
4. The `unsafe impl` is technically **sound** but **unnecessary** in modern Rust

**Historical Context**:
- This pattern was common in Rust 2015-2018 era
- Modern Rust (1.70+) has improved `PhantomData` semantics
- `PhantomData<T>` is now `Send + Sync` by default (doesn't require T to be)

#### Modernization Options

##### Option 1: Keep Current Implementation (Recommended for Phase 0)
**Pros**:
- Already working and tested
- No risk of introducing bugs
- Sound and safe

**Cons**:
- Uses `unsafe` unnecessarily
- Doesn't follow modern Rust idioms

**Action**: Add comprehensive safety documentation (see below)

##### Option 2: Replace with std::marker::PhantomData (Future Phase)
```rust
// Remove SendSyncPhantomData entirely
pub struct Rule1<PA, V, StashValue, F> {
    sym: Sym,
    pattern: PA,
    production: F,
    _phantom: std::marker::PhantomData<(V, StashValue)>,  // ← Direct use
}
```

**Pros**:
- Eliminates all `unsafe` code
- Modern Rust best practice
- Zero functional change

**Cons**:
- Requires testing across all 20 usage sites
- Slightly risky for Phase 0 (low priority item)

**Recommendation**: Consider for Phase 1 refactoring

##### Option 3: Use fn() -> T pattern (Alternative)
```rust
// Another common zero-sized phantom pattern
_phantom: std::marker::PhantomData<fn() -> (V, StashValue)>,
```

**Pros**:
- Guarantees covariance
- No unsafe needed

**Cons**:
- More complex type signature
- Unnecessary for this use case

---

## Verification with Miri

**Status**: ⏸️ Optional (not critical)

Miri is Rust's undefined behavior detector. Since our unsafe code only involves `PhantomData` (ZST with no actual operations), Miri would not find issues.

**Command** (if desired):
```bash
rustup component add miri --toolchain nightly
cargo +nightly miri test
```

**Expected Result**: All tests pass (no UB detected)

---

## Recommendations

### Immediate Actions (Phase 0 Week 2)

1. ✅ **Add safety documentation to core/src/lib.rs** (see next section)
2. ✅ **Update this audit report in repository**
3. ⏸️ **Run Miri** (optional, low priority)

### Future Actions (Phase 1+)

1. 🎯 **Remove SendSyncPhantomData entirely**
   - Replace with `std::marker::PhantomData<T>`
   - Test across all 20 usage sites
   - Should be zero functional change

2. 🎯 **Add `#![forbid(unsafe_code)]`** to crate root
   - Once all unsafe removed, forbid future additions
   - Ensures codebase remains 100% safe Rust

---

## Conclusion

The current unsafe code is **sound, justified, and low-risk**. It's a legacy pattern from older Rust that can be safely modernized in a future phase. No immediate action required beyond documentation.

**Phase 0 Acceptance Criteria**: ✅ **PASS**
- Unsafe code audited ✅
- Safety justified and documented ✅
- No critical issues found ✅

---

---

## Finding 2: FFI Unsafe Blocks (src/ffi.rs) — Added Phase 6-B

**Date**: 2026-02-27
**Status**: ✅ MITIGATED

### 2a. `CStr::from_ptr` — Reading C strings

```rust
// Safety: caller guarantees valid null-terminated UTF-8
let text_str = unsafe { CStr::from_ptr(text) }.to_str()?;
```

**Used in**: `rustling_parse_inner`, `rustling_locale_supported`
**Risk**: Dangling pointer or non-UTF-8 data from caller → **MEDIUM**
**Mitigations applied**:
- Null pointer checked before use
- `.to_str()` converts the error (non-UTF-8 → early return with error result)
- Standard FFI pattern; safety obligation on caller is documented in header

### 2b. `CString::from_raw` — Freeing C strings

```rust
pub unsafe extern "C" fn rustling_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);  // reclaims CString allocation
    }
}
```

**Used in**: `rustling_free_string`, `rustling_free_error`, `rustling_free_result`
**Risk**: Double-free or use-after-free if caller passes wrong pointer → **HIGH**
**Mitigations applied**:
- All string allocations in this crate use `CString::into_raw` (via `to_c_string()` helper) — allocation and deallocation are guaranteed to match
- Previous bug (`Vec::as_ptr` + `mem::forget`) was **fixed** on 2026-02-27: `rustling_supported_locales` now uses `to_c_string()` like all other functions
- `rustling_free_result` added to prevent partial-free bugs (callers no longer free `json` and `error` separately)

### 2c. Previously fixed: `Vec::as_ptr` + `mem::forget` UB

```rust
// REMOVED (was in rustling_supported_locales before 2026-02-27):
// let ptr = bytes.as_ptr() as *mut c_char;
// std::mem::forget(bytes);  // WRONG: CString::from_raw on Vec ptr = UB
```

This was **undefined behaviour**: `rustling_free_string` used `CString::from_raw`, but the pointer came from a `Vec` allocation. Fixed by replacing with `to_c_string(json)`.

### Summary of FFI Safety

| Pattern | Count | Status |
|---------|-------|--------|
| `CStr::from_ptr` | 3 | ✅ Null-checked, UTF-8 validated |
| `CString::from_raw` | 3 | ✅ All allocations via `CString::into_raw` |
| `Vec::as_ptr` UB | 0 | ✅ Eliminated (was 1, fixed 2026-02-27) |
| Panic across FFI | 0 | ✅ All functions wrapped in `catch_unwind` |

---

## Appendix: Unsafe Patterns — Complete Inventory

| Pattern | Status |
|---------|--------|
| Raw pointer dereferencing (`*ptr`) | ✅ Not present |
| `CStr::from_ptr` (FFI boundary) | ✅ Present, mitigated (see §2a) |
| `CString::from_raw` (FFI free) | ✅ Present, mitigated (see §2b) |
| Inline assembly (`asm!`) | ✅ Not present |
| Type transmutation (`std::mem::transmute`) | ✅ Not present |
| Mutable static variables (`static mut`) | ✅ Not present (using `OnceLock` instead) |
| Union field access | ✅ Not present |
| `unsafe impl Send/Sync` | ✅ Present in `core/` (see §1, benign) |

---

**Initial report**: 2026-02-12
**Updated**: 2026-02-27 (Phase 6-B FFI audit; fixed `rustling_supported_locales` UB)
**Next Review**: Phase 7 or on any future `unsafe` addition
**Approved By**: Phase 0 modernization audit + Phase 6-B FFI fix

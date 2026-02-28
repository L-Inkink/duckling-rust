/**
 * rustling.h — Public C API for the Rustling NLP parser
 *
 * Build the shared library first:
 *   cargo build --release --lib
 *
 * Then compile your program:
 *   gcc -o my_program my_program.c -I./include \
 *       -L./target/release -lrustling \
 *       -lpthread -ldl -lm
 *
 * On Linux, set the runtime path:
 *   LD_LIBRARY_PATH=./target/release ./my_program
 *
 * On macOS:
 *   DYLD_LIBRARY_PATH=./target/release ./my_program
 */

#ifndef RUSTLING_H
#define RUSTLING_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Types ─────────────────────────────────────────────────────────────── */

/**
 * Result of a rustling_parse() call.
 *
 * On success:  json != NULL, error == NULL, count >= 0
 * On failure:  json == NULL, error != NULL, count == 0
 *
 * Always free non-NULL pointers with rustling_free_string() /
 * rustling_free_error() when you are done with them.
 */
typedef struct {
    /** JSON-encoded array of parse results (UTF-8).  May be NULL on error. */
    char    *json;
    /** Number of items in the JSON array. */
    uint32_t count;
    /** Human-readable error message.  NULL if the call succeeded. */
    char    *error;
} RustlingParseResult;

/* ── Lifecycle ─────────────────────────────────────────────────────────── */

/**
 * Pre-warm the library (load locale data, compile patterns, …).
 *
 * Optional but recommended before the first parse call to reduce
 * first-call latency.  Safe to call more than once.
 */
void rustling_init(void);

/* ── Parsing ───────────────────────────────────────────────────────────── */

/**
 * Parse a single text and return all recognised values as JSON.
 *
 * @param text    NUL-terminated UTF-8 input string.  Must not be NULL.
 * @param locale  BCP-47 locale code ("en", "fr", "zh", …).
 *                Pass NULL or "" to use the default locale ("en").
 *
 * @return A RustlingParseResult whose json field points to a
 *         NUL-terminated UTF-8 JSON string like:
 *         [{"value":{"Integer":42},"byte_start":0,"byte_end":2,
 *           "char_start":0,"char_end":2}, …]
 *
 *         The "value" field is a structured JSON object, not a debug string:
 *           Integer:  {"Integer": 42}
 *           Float:    {"Float": 3.14}
 *           Duration: {"Duration": {"amount": 5, "unit": "Minute"}}
 *           Time:     {"Time": {...}}
 *
 * Prefer freeing the result with rustling_free_result() rather than
 * manually freeing each field.
 */
RustlingParseResult rustling_parse(const char *text, const char *locale);

/* ── Memory management ─────────────────────────────────────────────────── */

/**
 * Free all memory owned by a RustlingParseResult (both json and error).
 *
 * This is the preferred way to release a parse result.  It handles the
 * NULL checks for you and is safe to call on both success and error results.
 *
 * @param result  The result returned by rustling_parse().
 *                Must be called exactly once per result.
 */
void rustling_free_result(RustlingParseResult result);

/**
 * Free a string allocated by rustling (return value of rustling_version()
 * or rustling_supported_locales()).
 *
 * @param ptr  Pointer previously returned by a rustling function.
 *             Passing NULL is safe and has no effect.
 */
void rustling_free_string(char *ptr);

/**
 * Free an error string allocated by rustling.  Equivalent to
 * rustling_free_string(); provided for semantic clarity.
 *
 * @param ptr  Pointer previously returned as an error.
 *             Passing NULL is safe and has no effect.
 */
void rustling_free_error(char *ptr);

/* ── Metadata ──────────────────────────────────────────────────────────── */

/**
 * Return the library version string (e.g. "0.10.0").
 *
 * The caller must free the returned string with rustling_free_string().
 */
char *rustling_version(void);

/**
 * Return a JSON array of all supported locale codes.
 * Example: ["en","fr","de","zh","ja",…]
 *
 * The caller must free the returned string with rustling_free_string().
 */
char *rustling_supported_locales(void);

/**
 * Check whether a locale code is supported.
 *
 * @param locale  NUL-terminated locale code.  Must not be NULL.
 * @return 1 if supported, 0 otherwise.
 */
uint32_t rustling_locale_supported(const char *locale);

#ifdef __cplusplus
}
#endif

#endif /* RUSTLING_H */

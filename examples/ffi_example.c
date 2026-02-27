/**
 * ffi_example.c — Rustling FFI usage example
 *
 * Demonstrates the complete C API for the Rustling NLP parser library.
 *
 * Build:
 *   # 1. Compile the Rust library
 *   cargo build --release --lib
 *
 *   # 2. Compile this example
 *   gcc -o ffi_example examples/ffi_example.c \
 *       -I./include -L./target/release -lrustling \
 *       -lpthread -ldl -lm
 *
 *   # 3. Run (Linux)
 *   LD_LIBRARY_PATH=./target/release ./ffi_example
 *
 *   # 3. Run (macOS)
 *   DYLD_LIBRARY_PATH=./target/release ./ffi_example
 *
 * Or use the provided helper script:
 *   ./scripts/build_ffi_example.sh
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "rustling.h"

/* ── Helpers ────────────────────────────────────────────────────────────── */

static void print_separator(void) {
    printf("────────────────────────────────────────\n");
}

/**
 * Parse `text` with `locale`, print results, and free all memory.
 * Returns 1 on success, 0 on error.
 */
static int demo_parse(const char *text, const char *locale) {
    printf("  Input : \"%s\"  (locale: %s)\n", text, locale ? locale : "(default)");

    RustlingParseResult result = rustling_parse(text, locale);

    if (result.error != NULL) {
        fprintf(stderr, "  ERROR : %s\n", result.error);
        rustling_free_error(result.error);
        return 0;
    }

    printf("  Count : %u\n", result.count);
    printf("  JSON  : %s\n", result.json ? result.json : "(null)");

    rustling_free_string(result.json);
    return 1;
}

/* ── Main ───────────────────────────────────────────────────────────────── */

int main(void) {
    int failed = 0;

    /* ── 1. Initialise the library (optional warm-up) ─────────────────── */
    printf("=== Rustling FFI Example ===\n\n");
    printf("[1] Initialising library…\n");
    rustling_init();
    printf("    Done.\n\n");

    /* ── 2. Version ───────────────────────────────────────────────────── */
    printf("[2] Library version\n");
    char *version = rustling_version();
    printf("    Version: %s\n\n", version);
    rustling_free_string(version);

    /* ── 3. Supported locales ─────────────────────────────────────────── */
    printf("[3] Supported locales\n");
    char *locales = rustling_supported_locales();
    printf("    %s\n\n", locales);
    rustling_free_string(locales);

    /* ── 4. Locale check ──────────────────────────────────────────────── */
    printf("[4] Locale support checks\n");
    const char *test_locales[] = {"en", "fr", "zh", "xx", NULL};
    for (int i = 0; test_locales[i] != NULL; i++) {
        uint32_t supported = rustling_locale_supported(test_locales[i]);
        printf("    \"%s\" → %s\n", test_locales[i], supported ? "supported" : "NOT supported");
    }
    printf("\n");

    /* ── 5. Integer parsing ───────────────────────────────────────────── */
    print_separator();
    printf("[5] Integer parsing (English)\n");
    if (!demo_parse("42", "en")) failed++;
    printf("\n");

    if (!demo_parse("one hundred and twenty three", "en")) failed++;
    printf("\n");

    /* ── 6. Duration parsing ──────────────────────────────────────────── */
    print_separator();
    printf("[6] Duration parsing (English)\n");
    if (!demo_parse("5 minutes", "en")) failed++;
    printf("\n");

    if (!demo_parse("2 hours and 30 minutes", "en")) failed++;
    printf("\n");

    /* ── 7. Multi-language parsing ────────────────────────────────────── */
    print_separator();
    printf("[7] Multi-language parsing\n");
    if (!demo_parse("quarante-deux", "fr")) failed++;  /* French: forty-two */
    printf("\n");

    if (!demo_parse("vierzig", "de")) failed++;        /* German: forty */
    printf("\n");

    /* ── 8. Empty text ────────────────────────────────────────────────── */
    print_separator();
    printf("[8] Edge cases\n");
    if (!demo_parse("", "en")) failed++;
    printf("\n");

    /* ── 9. Unsupported locale (returns error) ────────────────────────── */
    printf("[9] Unsupported locale (expected error)\n");
    {
        RustlingParseResult result = rustling_parse("42", "xx");
        if (result.error != NULL) {
            printf("  OK  : Got expected error: %s\n", result.error);
            rustling_free_error(result.error);
        } else {
            /* Some builds return empty results instead of an error */
            printf("  OK  : count=%u (empty result for unknown locale)\n",
                   result.count);
            rustling_free_string(result.json);
        }
    }
    printf("\n");

    /* ── 10. NULL text (expected error) ──────────────────────────────── */
    printf("[10] NULL text (expected error)\n");
    {
        RustlingParseResult result = rustling_parse(NULL, "en");
        if (result.error != NULL) {
            printf("  OK  : Got expected error: %s\n", result.error);
            rustling_free_error(result.error);
        } else {
            fprintf(stderr, "  FAIL: Expected an error for NULL text\n");
            rustling_free_string(result.json);
            failed++;
        }
    }
    printf("\n");

    /* ── Summary ──────────────────────────────────────────────────────── */
    print_separator();
    if (failed == 0) {
        printf("All demos completed successfully.\n");
    } else {
        fprintf(stderr, "%d demo(s) failed.\n", failed);
        return 1;
    }

    return 0;
}

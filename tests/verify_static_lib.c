/**
 * verify_static_lib.c — 验证 librustling.a 可正常调用
 *
 * 编译:
 *   gcc -o /tmp/verify_rustling tests/verify_static_lib.c \
 *       -I./include \
 *       -L./target_user/x86_64-unknown-linux-gnu/release \
 *       -Wl,--whole-archive -lrustling -Wl,--no-whole-archive \
 *       -lpthread -ldl -lm
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rustling.h"

/* ANSI colors */
#define GRN "\033[32m"
#define RED "\033[31m"
#define RST "\033[0m"

static int g_pass = 0, g_fail = 0;

static void check(const char *label, int ok) {
    if (ok) {
        printf("  " GRN "✓" RST " %s\n", label);
        g_pass++;
    } else {
        printf("  " RED "✗" RST " %s\n", label);
        g_fail++;
    }
}

static void test_parse(const char *text, const char *locale,
                       int expect_count, const char *expect_substr) {
    printf("\n  parse(\"%s\", \"%s\")\n", text, locale);
    RustlingParseResult r = rustling_parse(text, locale);

    check("no error", r.error == NULL);
    check("json not NULL", r.json != NULL);

    if (r.json) {
        printf("    json  = %s\n", r.json);
        printf("    count = %u\n", r.count);
        check("count matches", (int)r.count == expect_count);
        if (expect_substr) {
            check("expected substr found",
                  strstr(r.json, expect_substr) != NULL);
        }
    }
    rustling_free_result(r);
}

int main(void) {
    printf("=== rustling static library verification ===\n");

    /* version */
    char *ver = rustling_version();
    printf("\nversion: %s\n", ver ? ver : "(null)");
    check("version not NULL", ver != NULL);
    rustling_free_string(ver);

    /* locales */
    char *locales = rustling_supported_locales();
    check("supported_locales not NULL", locales != NULL);
    check("contains \"en\"", locales && strstr(locales, "\"en\"") != NULL);
    check("contains \"zh\"", locales && strstr(locales, "\"zh\"") != NULL);
    if (locales) printf("locales: %s\n", locales);
    rustling_free_string(locales);

    /* locale_supported */
    check("en supported",  rustling_locale_supported("en")  == 1);
    check("zh supported",  rustling_locale_supported("zh")  == 1);
    check("xx unsupported",rustling_locale_supported("xx")  == 0);

    /* init (warm-up) */
    rustling_init();
    check("init completed", 1);

    /* --- parse tests --- */
    test_parse("42",            "en", 1, "Integer");
    /* "five" is not parsed as a number without context */
    test_parse("five",          "en", 0, NULL);
    /* "5 minutes" matches both Integer("5") and Duration("5 minutes") → count=2 */
    test_parse("5 minutes",     "en", 2, "Duration");
    test_parse("tomorrow",      "en", 1, "Time");
    /* "3.14" is parsed as two Integers (3 and 14), no Float type */
    test_parse("3.14",          "en", 2, "Integer");
    /* zh time parsing requires a reference time; plain text returns empty */
    test_parse("明天下午三点",   "zh", 0, NULL);

    /* empty input → 0 results */
    {
        printf("\n  parse(\"\", \"en\")\n");
        RustlingParseResult r = rustling_parse("", "en");
        check("no error on empty", r.error == NULL);
        check("count == 0",        r.count == 0);
        rustling_free_result(r);
    }

    /* --- summary --- */
    printf("\n=== %d passed, %d failed ===\n", g_pass, g_fail);
    return g_fail > 0 ? 1 : 0;
}

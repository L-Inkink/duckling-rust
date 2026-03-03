package com.rustling

/**
 * High-level convenience client built on top of [NLPParser].
 *
 * [DucklingClient] wraps the raw parser with typed extraction helpers so
 * callers can ask for a specific kind of value without manually filtering
 * the result list.
 *
 * ## Usage
 *
 * ```kotlin
 * val client = DucklingClient(defaultLocale = "en")
 *
 * // Extract only durations from the text
 * val durations = client.extractDurations("Set a timer for 20 minutes")
 * durations.forEach { d -> println("${d.amount} ${d.unit}") }
 *
 * // Extract everything
 * val all = client.extractAll("Call me at 3pm for 30 minutes")
 * all.forEach { r -> println(r.value) }
 * ```
 *
 * @param defaultLocale BCP-47 locale used when [parse] is called without an
 *                      explicit locale. Defaults to `"en"`.
 */
class DucklingClient(val defaultLocale: String = "en") {

    // ── Generic extraction ────────────────────────────────────────────────

    /**
     * Parse [text] and return all recognised values in [locale].
     */
    fun extractAll(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale)

    // ── Typed extraction helpers ──────────────────────────────────────────

    /**
     * Return only the integer values found in [text].
     *
     * Example: `"I have 3 cats and 2 dogs"` → two [ParsedValue.Integer] results.
     */
    fun extractIntegers(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale).filter { it.value is ParsedValue.Integer }

    /**
     * Return only the float values found in [text].
     */
    fun extractFloats(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale).filter { it.value is ParsedValue.Float }

    /**
     * Return only the duration values found in [text].
     *
     * Example: `"Wait 5 minutes then retry"` → one [ParsedValue.Duration] result.
     */
    fun extractDurations(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale).filter { it.value is ParsedValue.Duration }

    /**
     * Return only the point-in-time values found in [text].
     *
     * Example: `"Meeting on Monday at 9am"` → one [ParsedValue.Time] result.
     */
    fun extractTimes(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale).filter { it.value is ParsedValue.Time }

    /**
     * Return only the time-interval values found in [text].
     *
     * Example: `"The event runs from Monday to Wednesday"` →
     * one [ParsedValue.Interval] result.
     */
    fun extractIntervals(text: String, locale: String = defaultLocale): List<ParseResult> =
        NLPParser.parse(text, locale).filter { it.value is ParsedValue.Interval }

    // ── Convenience scalar extractors ─────────────────────────────────────

    /**
     * Return the first integer value in [text], or `null` if none found.
     */
    fun firstInteger(text: String, locale: String = defaultLocale): Long? =
        (extractIntegers(text, locale).firstOrNull()?.value as? ParsedValue.Integer)?.value

    /**
     * Return the first duration in [text], or `null` if none found.
     */
    fun firstDuration(text: String, locale: String = defaultLocale): ParsedValue.Duration? =
        extractDurations(text, locale).firstOrNull()?.value as? ParsedValue.Duration

    /**
     * Return the first point-in-time in [text], or `null` if none found.
     */
    fun firstTime(text: String, locale: String = defaultLocale): ParsedValue.Time? =
        extractTimes(text, locale).firstOrNull()?.value as? ParsedValue.Time

    // ── Metadata ──────────────────────────────────────────────────────────

    /** The native library version string (e.g. `"0.10.0"`). */
    val libraryVersion: String get() = NLPParser.version()

    /** All locale codes supported by this build. */
    val supportedLocales: List<String> get() = NLPParser.supportedLocales()

    /** Return `true` if [locale] is supported by this build. */
    fun isLocaleSupported(locale: String): Boolean = NLPParser.isLocaleSupported(locale)
}

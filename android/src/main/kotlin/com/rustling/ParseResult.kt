package com.rustling

/**
 * A typed value parsed from natural language text.
 *
 * The Rust library recognises five value kinds; each maps to a subclass.
 * Use Kotlin's `when` expression to handle them:
 *
 * ```kotlin
 * when (val v = result.value) {
 *     is ParsedValue.Integer  -> println("integer: ${v.value}")
 *     is ParsedValue.Float    -> println("float: ${v.value}")
 *     is ParsedValue.Duration -> println("${v.amount} ${v.unit} = ${v.seconds}s")
 *     is ParsedValue.Time     -> println("time: ${v.isoValue} (${v.grain})")
 *     is ParsedValue.Interval -> println("from ${v.from} to ${v.to}")
 *     is ParsedValue.Unknown  -> println("raw JSON: ${v.rawJson}")
 * }
 * ```
 */
sealed class ParsedValue {
    /** A whole number, e.g. "forty-two" → Integer(42). */
    data class Integer(val value: Long) : ParsedValue()

    /** A decimal number, e.g. "3.14" → Float(3.14). */
    data class Float(val value: Double) : ParsedValue()

    /**
     * A duration, e.g. "5 minutes" → Duration(amount=5, unit="minute", seconds=300).
     * [unit] is one of: second, minute, hour, day, week.
     */
    data class Duration(
        val seconds: Long,
        val unit: String,
        val amount: Long,
    ) : ParsedValue()

    /**
     * A point in time, e.g. "next Monday" → Time(isoValue="2024-01-08T00:00:00+00:00", grain="day").
     * [isoValue] is an ISO-8601 datetime string.
     * [grain] is one of: second, minute, hour, day, week, month, quarter, year.
     * [latent] is true when context is needed to resolve the time (e.g. bare "Monday").
     */
    data class Time(
        val isoValue: String,
        val grain: String,
        val latent: Boolean,
        val holiday: String? = null,
        val form: String? = null,
    ) : ParsedValue()

    /**
     * A time interval, e.g. "from Monday to Wednesday".
     * [from] and [to] are ISO-8601 datetime strings.
     */
    data class Interval(
        val from: String,
        val to: String,
        val grain: String,
    ) : ParsedValue()

    /** Fallback for future value types not yet handled in this SDK version. */
    data class Unknown(val rawJson: String) : ParsedValue()
}

/**
 * A single match returned by [NLPParser.parse].
 *
 * @property value      The typed parsed value.
 * @property byteStart  Byte offset of the match start in the original text.
 * @property byteEnd    Byte offset of the match end (exclusive).
 * @property charStart  Character (Unicode code-point) offset of the match start.
 * @property charEnd    Character offset of the match end (exclusive).
 */
data class ParseResult(
    val value: ParsedValue,
    val byteStart: Int,
    val byteEnd: Int,
    val charStart: Int,
    val charEnd: Int,
)

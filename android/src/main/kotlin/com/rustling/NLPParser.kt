package com.rustling

import org.json.JSONArray
import org.json.JSONObject

/**
 * Bridge to the Rustling NLP parser native library.
 *
 * All public methods on the companion object are thread-safe — the underlying
 * Rust parser uses a global `OnceLock<Parser>` and `LocaleRegistry` that are
 * initialised once and never mutated.
 *
 * ## Setup
 *
 * 1. Copy `librustling.so` to `app/src/main/jniLibs/<abi>/`.
 * 2. Call `NLPParser.init()` once at application start (optional but reduces
 *    first-parse latency by pre-warming the locale registry).
 *
 * ## Quick start
 *
 * ```kotlin
 * class MyApp : Application() {
 *     override fun onCreate() {
 *         super.onCreate()
 *         NLPParser.init()
 *     }
 * }
 *
 * // Somewhere in your code:
 * val results = NLPParser.parse("I need 5 minutes", locale = "en")
 * results.forEach { r ->
 *     when (val v = r.value) {
 *         is ParsedValue.Duration -> Log.d("NLP", "${v.amount} ${v.unit}")
 *         else -> {}
 *     }
 * }
 * ```
 */
class NLPParser private constructor() {

    companion object {

        init {
            System.loadLibrary("rustling")
        }

        // ── JNI declarations ──────────────────────────────────────────────
        // These map directly to Rust functions in src/jni_wrapper.rs.

        @JvmStatic private external fun nativeInit()
        @JvmStatic private external fun nativeParse(text: String, locale: String): String?
        @JvmStatic private external fun nativeVersion(): String?
        @JvmStatic private external fun nativeSupportedLocales(): String?
        @JvmStatic private external fun nativeIsLocaleSupported(locale: String): Boolean

        // ── Public API ────────────────────────────────────────────────────

        /**
         * Pre-warm the parser (loads locale data, compiles patterns).
         *
         * Optional but recommended in `Application.onCreate()` to avoid a
         * cold-start delay on the first [parse] call.
         */
        @JvmStatic
        fun init() {
            nativeInit()
        }

        /**
         * Return the native library version string (e.g. `"0.10.0"`).
         */
        @JvmStatic
        fun version(): String = nativeVersion() ?: "unknown"

        /**
         * Return all locale codes supported by this build of the library.
         *
         * Example: `["en", "fr", "de", "zh", "ja", ...]`
         */
        @JvmStatic
        fun supportedLocales(): List<String> {
            val json = nativeSupportedLocales() ?: return emptyList()
            return try {
                val arr = JSONArray(json)
                (0 until arr.length()).map { arr.getString(it) }
            } catch (_: Exception) {
                emptyList()
            }
        }

        /**
         * Return `true` if [locale] (e.g. `"en"`, `"fr"`) is supported.
         */
        @JvmStatic
        fun isLocaleSupported(locale: String): Boolean = nativeIsLocaleSupported(locale)

        /**
         * Parse [text] in the given [locale] and return all recognised values.
         *
         * @param text   Natural-language input string.
         * @param locale BCP-47 locale code. Defaults to `"en"`.
         * @return       Ordered list of [ParseResult] objects (may be empty).
         */
        @JvmStatic
        @JvmOverloads
        fun parse(text: String, locale: String = "en"): List<ParseResult> {
            val json = nativeParse(text, locale) ?: return emptyList()
            return try {
                parseOutput(JSONObject(json))
            } catch (_: Exception) {
                emptyList()
            }
        }

        // ── JSON → ParseResult conversion ─────────────────────────────────

        private fun parseOutput(root: JSONObject): List<ParseResult> {
            val arr = root.optJSONArray("results") ?: return emptyList()
            return (0 until arr.length()).mapNotNull { i ->
                try {
                    parseResult(arr.getJSONObject(i))
                } catch (_: Exception) {
                    null
                }
            }
        }

        private fun parseResult(obj: JSONObject): ParseResult {
            val value = parsedValue(obj.getJSONObject("value"))
            return ParseResult(
                value     = value,
                byteStart = obj.getInt("byte_start"),
                byteEnd   = obj.getInt("byte_end"),
                charStart = obj.getInt("char_start"),
                charEnd   = obj.getInt("char_end"),
            )
        }

        private fun parsedValue(v: JSONObject): ParsedValue {
            return when (v.optString("type")) {
                "integer"  -> ParsedValue.Integer(v.getLong("value"))
                "float"    -> ParsedValue.Float(v.getDouble("value"))
                "duration" -> ParsedValue.Duration(
                    seconds = v.getLong("seconds"),
                    unit    = v.getString("unit"),
                    amount  = v.getLong("amount"),
                )
                "time"     -> ParsedValue.Time(
                    isoValue = v.getString("value"),
                    grain    = v.getString("grain"),
                    latent   = v.getBoolean("latent"),
                    holiday  = v.optString("holiday").takeIf { it.isNotEmpty() },
                    form     = v.optString("form").takeIf { it.isNotEmpty() },
                )
                "interval" -> ParsedValue.Interval(
                    from  = v.getString("from"),
                    to    = v.getString("to"),
                    grain = v.getString("grain"),
                )
                else       -> ParsedValue.Unknown(v.toString())
            }
        }
    }
}

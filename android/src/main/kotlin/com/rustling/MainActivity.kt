package com.rustling

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.rustling.ParsedValue
import com.rustling.ParseResult

/**
 * Main Activity demonstrating Duckling NLP parsing on Android.
 *
 * This activity provides a simple UI to:
 * 1. Pre-warm the parser on startup
 * 2. Enter text and parse it with a selected locale
 * 3. Display the parsed results with type information
 *
 * The native library (librustling.so) must be placed in:
 * app/src/main/jniLibs/<abi>/
 *
 * Supported ABIs: arm64-v8a, armeabi-v7a, x86_64
 */
class MainActivity : AppCompatActivity() {

    private lateinit var inputText: EditText
    private lateinit var parseButton: Button
    private lateinit var clearButton: Button
    private lateinit var resultsView: TextView
    private lateinit var statusView: TextView

    // DucklingClient provides high-level typed extraction
    private val ducklingClient = DucklingClient(defaultLocale = "en")

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Initialize views
        inputText = findViewById(R.id.input_text)
        parseButton = findViewById(R.id.btn_parse)
        clearButton = findViewById(R.id.btn_clear)
        resultsView = findViewById(R.id.results_view)
        statusView = findViewById(R.id.status_view)

        // Pre-warm the parser on startup
        initializeParser()

        // Set up click listeners
        parseButton.setOnClickListener { performParse() }
        clearButton.setOnClickListener { clearResults() }
    }

    /**
     * Initialize the native parser.
     * This pre-warms the parser and loads locale data.
     */
    private fun initializeParser() {
        try {
            // Call init() to pre-warm the parser (optional but recommended)
            NLPParser.init()

            // Display library info
            val version = NLPParser.version()
            val locales = NLPParser.supportedLocales()
            val localeCount = locales.size

            statusView.text = getString(
                R.string.status_initialized,
                version,
                localeCount
            )

        } catch (e: UnsatisfiedLinkError) {
            statusView.text = getString(R.string.error_library_not_found, e.message)
            parseButton.isEnabled = false
        } catch (e: Exception) {
            statusView.text = getString(R.string.error_init_failed, e.message)
        }
    }

    /**
     * Parse the input text using Duckling NLP.
     */
    private fun performParse() {
        val text = inputText.text.toString().trim()

        if (text.isEmpty()) {
            resultsView.text = getString(R.string.error_empty_input)
            return
        }

        try {
            // Use DucklingClient for typed extraction
            val results = ducklingClient.extractAll(text)

            if (results.isEmpty()) {
                resultsView.text = getString(R.string.no_results)
                return
            }

            // Format and display results
            val formattedResults = formatResults(results)
            resultsView.text = formattedResults

        } catch (e: Exception) {
            resultsView.text = getString(R.string.error_parse_failed, e.message)
        }
    }

    /**
     * Format parse results for display.
     */
    private fun formatResults(results: List<ParseResult>): String {
        val builder = StringBuilder()
        builder.append("Found ${results.size} result(s):\n\n")

        results.forEachIndexed { index, result ->
            builder.append("${index + 1}. [${result.charStart}:${result.charEnd}] ")

            when (val value = result.value) {
                is ParsedValue.Integer -> {
                    builder.append("Integer: ${value.value}\n")
                }
                is ParsedValue.Float -> {
                    builder.append("Float: ${value.value}\n")
                }
                is ParsedValue.Duration -> {
                    builder.append("Duration: ${value.amount} ${value.unit} (${value.seconds}s)\n")
                }
                is ParsedValue.Time -> {
                    builder.append("Time: ${value.isoValue} (grain: ${value.grain})")
                    if (value.latent) builder.append(" [latent]")
                    value.holiday?.let { builder.append(", holiday: $it") }
                    builder.append("\n")
                }
                is ParsedValue.Interval -> {
                    builder.append("Interval: ${value.from} → ${value.to} (${value.grain})\n")
                }
                is ParsedValue.Unknown -> {
                    builder.append("Unknown: ${value.rawJson}\n")
                }
            }
        }

        return builder.toString()
    }

    /**
     * Clear input and results.
     */
    private fun clearResults() {
        inputText.text.clear()
        resultsView.text = ""
    }
}

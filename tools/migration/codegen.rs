#!/usr/bin/env rustc --edition 2021
//! Duckling → Rustling Code Generator
//!
//! Generates Rust source code from extracted JSON rule definitions.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
struct RuleFile {
    dimension: String,
    locale: String,
    source_file: String,
    rules: Vec<Rule>,
    metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Rule {
    name: String,
    rule_type: String,
    pattern: serde_json::Value,
    production: Production,
    examples: Option<Vec<String>>,
    priority: Option<i32>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Production {
    value_extractor: String,
    function: Option<String>,
    arguments: Option<Vec<String>>,
    custom_logic: Option<String>,
    confidence: Option<f64>,
    latent: Option<bool>,
    grain: Option<String>,
    dictionary_ref: Option<String>,
    constant_value: Option<i64>,  // For constant value rules
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    extracted_at: String,
    extractor_version: String,
    extraction_method: String,
    rules_needing_review: Option<usize>,
    notes: Option<String>,
}

struct CodeGenerator {
    tera: Tera,
    output_dir: PathBuf,
}

impl CodeGenerator {
    fn new(template_dir: &Path, output_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let template_path = format!("{}/**/*.tera", template_dir.display());
        let mut tera = match Tera::new(&template_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Warning: Could not load templates from {}: {}", template_path, e);
                eprintln!("Creating empty Tera instance");
                Tera::default()
            }
        };

        // Add templates inline as fallback
        let numeral_template = include_str!("../../templates/numeral_rules_v3.rs.tera");
        let generic_template = include_str!("../../templates/generic_rules.rs.tera");

        tera.add_raw_template("numeral_rules.rs.tera", numeral_template)?;
        tera.add_raw_template("generic_rules.rs.tera", generic_template)?;
        tera.add_raw_template("time_rules.rs.tera", generic_template)?; // Use generic for now
        tera.add_raw_template("duration_rules.rs.tera", generic_template)?;

        tera.autoescape_on(vec![]);  // Disable HTML escaping for code generation

        Ok(Self {
            tera,
            output_dir: output_dir.to_path_buf(),
        })
    }

    fn generate(&self, rule_file: &RuleFile) -> Result<(), Box<dyn std::error::Error>> {
        let locale = &rule_file.locale;
        let dimension = rule_file.dimension.to_lowercase();

        // Create output directory
        let locale_dir = self.output_dir.join("languages").join(locale);
        fs::create_dir_all(&locale_dir)?;

        // Generate dimension-specific Rust file
        let output_file = locale_dir.join(format!("{}.rs", dimension));

        let mut context = Context::new();
        context.insert("locale", locale);
        context.insert("dimension", &rule_file.dimension);

        // Build dictionary type map for reference resolution
        let mut dict_type_map: HashMap<String, String> = HashMap::new();
        for rule in &rule_file.rules {
            if rule.rule_type == "dictionary" {
                if let Some(entries) = rule.pattern.get("entries").and_then(|v| v.as_object()) {
                    let value_type = infer_dict_value_type(entries);
                    // Map exact rule name
                    dict_type_map.insert(rule.name.clone(), value_type.clone());

                    // If rule name ends with "_dictionary", also map the base name + "Map"
                    if let Some(base_name) = rule.name.strip_suffix("_dictionary") {
                        dict_type_map.insert(format!("{}Map", base_name), value_type);
                    }
                }
            }
        }

        // Enrich rules with type information
        let enriched_rules: Vec<_> = rule_file.rules.iter().map(|rule| {
            let mut enriched = rule.clone();

            // Add inferred value type for dictionary rules
            if rule.rule_type == "dictionary" {
                if let Some(entries) = rule.pattern.get("entries").and_then(|v| v.as_object()) {
                    let value_type = infer_dict_value_type(entries);
                    if enriched.metadata.is_none() {
                        enriched.metadata = Some(serde_json::json!({}));
                    }
                    enriched.metadata.as_mut().unwrap().as_object_mut().unwrap()
                        .insert("value_type".to_string(), serde_json::Value::String(value_type));
                }
            }

            // Add referenced dictionary type for dict_ref rules
            if rule.rule_type == "regex" {
                if let Some(dict_ref) = rule.production.dictionary_ref.as_ref() {
                    if let Some(value_type) = dict_type_map.get(dict_ref) {
                        if enriched.metadata.is_none() {
                            enriched.metadata = Some(serde_json::json!({}));
                        }
                        enriched.metadata.as_mut().unwrap().as_object_mut().unwrap()
                            .insert("ref_value_type".to_string(), serde_json::Value::String(value_type.clone()));
                    }
                }
            }

            enriched
        }).collect();

        context.insert("rules", &enriched_rules);
        context.insert("source_file", &rule_file.source_file);

        // Choose template based on dimension
        let template_name = match dimension.as_str() {
            "numeral" => "numeral_rules.rs.tera",
            "time" => "time_rules.rs.tera",
            "duration" => "duration_rules.rs.tera",
            _ => "generic_rules.rs.tera",
        };

        let rendered = self.tera.render(template_name, &context)?;

        fs::write(&output_file, rendered)?;

        println!("✓ Generated {} ({} rules)", output_file.display(), rule_file.rules.len());

        // Update mod.rs
        self.update_mod_rs(locale, &dimension)?;

        Ok(())
    }

    fn update_mod_rs(&self, locale: &str, dimension: &str) -> Result<(), Box<dyn std::error::Error>> {
        let locale_dir = self.output_dir.join("languages").join(locale);
        let mod_file = locale_dir.join("mod.rs");

        let module_decl = format!("pub mod {};\n", dimension);

        if mod_file.exists() {
            let content = fs::read_to_string(&mod_file)?;
            if !content.contains(&module_decl) {
                fs::write(&mod_file, format!("{}{}", content, module_decl))?;
            }
        } else {
            fs::write(&mod_file, &module_decl)?;
        }

        Ok(())
    }

    fn update_root_languages_mod(&self, locales: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mod_file = self.output_dir.join("languages").join("mod.rs");
        fs::create_dir_all(self.output_dir.join("languages"))?;

        let mut content = String::from("// Auto-generated language modules\n");
        content.push_str("// Generated by tools/migration/codegen.rs\n\n");

        for locale in locales {
            content.push_str(&format!("pub mod {};\n", locale));
        }

        fs::write(&mod_file, content)?;

        Ok(())
    }
}

fn infer_dict_value_type(entries: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut has_float = false;
    let mut has_int = false;

    for value_obj in entries.values() {
        if let Some(value) = value_obj.get("value") {
            match value {
                serde_json::Value::Number(n) => {
                    if n.as_f64().map(|f| f.fract() != 0.0).unwrap_or(false) {
                        has_float = true;
                    } else {
                        has_int = true;
                    }
                },
                _ => {}
            }
        }
    }

    if has_float {
        "f64".to_string()
    } else {
        "i64".to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: codegen [input_files...] --output <output_dir>");
        eprintln!("");
        eprintln!("Example:");
        eprintln!("  cargo run --bin codegen --features migration-tools -- \\");
        eprintln!("    extracted/numeral/*.json --output .");
        std::process::exit(1);
    }

    let template_dir = Path::new("templates");
    let mut output_dir = PathBuf::from(".");

    // Parse arguments
    let mut input_files = Vec::new();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "--output" || args[i] == "-o" {
            if i + 1 < args.len() {
                output_dir = PathBuf::from(&args[i + 1]);
                i += 2;
            } else {
                eprintln!("Error: --output requires a directory argument");
                std::process::exit(1);
            }
        } else {
            // Expand glob patterns if needed
            if args[i].contains('*') {
                // Simple glob expansion for shell
                match glob::glob(&args[i]) {
                    Ok(paths) => {
                        for path in paths.filter_map(Result::ok) {
                            input_files.push(path.to_string_lossy().to_string());
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to expand glob pattern {}: {}", args[i], e);
                        input_files.push(args[i].clone());
                    }
                }
            } else {
                input_files.push(args[i].clone());
            }
            i += 1;
        }
    }

    if input_files.is_empty() {
        eprintln!("Error: No input files specified");
        std::process::exit(1);
    }

    println!("Codegen starting...");
    println!("  Input files: {}", input_files.len());
    println!("  Output directory: {}", output_dir.display());
    println!("");

    let generator = CodeGenerator::new(template_dir, &output_dir)?;

    let mut processed_locales = HashSet::new();
    let mut total_rules = 0;

    for input_file in &input_files {
        let content = fs::read_to_string(input_file)
            .map_err(|e| format!("Failed to read {}: {}", input_file, e))?;

        let rule_file: RuleFile = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON in {}: {}", input_file, e))?;

        total_rules += rule_file.rules.len();

        generator.generate(&rule_file)?;

        processed_locales.insert(rule_file.locale.clone());
    }

    // Update root languages/mod.rs
    let mut locales: Vec<String> = processed_locales.into_iter().collect();
    locales.sort();
    generator.update_root_languages_mod(&locales)?;

    println!("");
    println!("✓ Code generation complete!");
    println!("  Locales: {}", locales.len());
    println!("  Total rules: {}", total_rules);

    Ok(())
}

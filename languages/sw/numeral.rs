// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/SW/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: sw
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kumi", 10);
        
        map.insert("mbili", 2);
        
        map.insert("moja", 1);
        
        map.insert("nane", 8);
        
        map.insert("nne", 4);
        
        map.insert("saba", 7);
        
        map.insert("sita", 6);
        
        map.insert("sufuri", 0);
        
        map.insert("tano", 5);
        
        map.insert("tatu", 3);
        
        map.insert("tisa", 9);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("arobaini", 40);
        
        map.insert("arubaini", 40);
        
        map.insert("hamsini", 50);
        
        map.insert("ishirini", 20);
        
        map.insert("sabini", 70);
        
        map.insert("sitini", 60);
        
        map.insert("thelathini", 30);
        
        map.insert("themanini", 80);
        
        map.insert("tisini", 90);
        
        map
    };
}


/// Build Numeral rules for sw locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 1 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: sufuri, zero, moja, mbili, tatu
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "sw:ruleNumeral_dictionary",
            b.reg(r"(?i)kumi|mbili|moja|nane|nne|saba|sita|sufuri|tano|tatu|tisa|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ishirini, thelathini, arubaini, arobaini, hamsini
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "sw:tens_dictionary",
            b.reg(r"(?i)arobaini|arubaini|hamsini|ishirini|sabini|sitini|thelathini|themanini|tisini").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (2)
    // ========================================
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "sw:Numeral",
            b.reg(r"(?i)(sufuri|zero|moja|mbili|tatu|nne|tano|sita|saba|nane|tisa|kumi)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Tens (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "sw:Tens",
            b.reg(r"(?i)(ishirini|thelathini|arubaini|arobaini|hamsini|sitini|sabini|themanini|tisini)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (1)
    // ========================================
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 11..19 21..29 .. 91..99
    
    // Manual implementation required
    

    
    eprintln!("⚠️  sw/numeral has 1 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_sw_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_sw_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_sw_numeral_stats() {
        // Generation statistics
        let total_rules = 5;
        let auto_generated = 4;
        let manual_needed = 1;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("sw/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

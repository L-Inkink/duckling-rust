// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/CS/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: cs
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("deset", 10);
        
        map.insert("dev\x0115t", 9);
        
        map.insert("dv\x0115", 2);
        
        map.insert("dva", 2);
        
        map.insert("jeden", 1);
        
        map.insert("jedna", 1);
        
        map.insert("jedno", 1);
        
        map.insert("nula", 0);
        
        map.insert("osm", 8);
        
        map.insert("p\x0115t", 5);
        
        map.insert("sedm", 7);
        
        map.insert("t\x0159i", 3);
        
        map.insert("čty\x0159i", 4);
        
        map.insert("šest", 6);
        
        map
    };
}


/// Build Numeral rules for cs locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: nula, jeden, jedna, jedno, dva
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "cs:ruleNumeral_dictionary",
            b.reg(r"(?i)deset|dev\x0115t|dv\x0115|dva|jeden|jedna|jedno|nula|osm|p\x0115t|sedm|t\x0159i|čty\x0159i|šest").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (1)
    // ========================================
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "cs:Numeral",
            b.reg(r"(?i)(nula|jed(en|n[ao])|dv(a|\x0115)|t(\x0159)i|(č)ty(\x0159)i|p(\x0115)t|(š)est|sedm|osm|dev(\x0115)t|deset)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_cs_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_cs_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_cs_numeral_stats() {
        // Generation statistics
        let total_rules = 2;
        let auto_generated = 2;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("cs/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

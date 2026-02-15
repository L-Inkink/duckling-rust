// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/IS/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: is
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  




// Dictionary: zeroToTwenty_dictionary
lazy_static! {
    static ref ZEROTOTWENTY_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("einn", 1);
        
        map.insert("ellefu", 11);
        
        map.insert("fimm", 5);
        
        map.insert("fimmtán", 15);
        
        map.insert("fjórir", 4);
        
        map.insert("fjórtán", 14);
        
        map.insert("null", 0);
        
        map.insert("nítján", 19);
        
        map.insert("níu", 9);
        
        map.insert("núll", 0);
        
        map.insert("sautján", 17);
        
        map.insert("sex", 6);
        
        map.insert("sextán", 16);
        
        map.insert("sjö", 7);
        
        map.insert("tuttugu", 20);
        
        map.insert("tveir", 2);
        
        map.insert("tíu", 10);
        
        map.insert("tólf", 12);
        
        map.insert("átján", 18);
        
        map.insert("átta", 8);
        
        map.insert("þrettán", 13);
        
        map.insert("þrír", 3);
        
        map
    };
}


/// Build Numeral rules for is locale
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
    

    // Rule: zeroToTwenty_dictionary
    // Examples: núll, null, einn, tveir, þrír
    {
        let dict = &*ZEROTOTWENTY_DICTIONARY;
        b.rule_1_terminal(
            "is:zeroToTwenty_dictionary",
            b.reg(r"(?i)einn|ellefu|fimm|fimmtán|fjórir|fjórtán|null|nítján|níu|núll|sautján|sex|sextán|sjö|tuttugu|tveir|tíu|tólf|átján|átta|þrettán|þrír").unwrap(),
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
    

    // Rule: ZeroToTwenty (refs: zeroToTwentyMap)
    {
        
        
        let dict = &*ZEROTOTWENTY_DICTIONARY;
        b.rule_1_terminal(
            "is:ZeroToTwenty",
            b.reg(r"(?i)(n[úu]ll|einn|tveir|þrír|fjórir|fimm(tán)?|sex(tán)?|sjö|átta|níu|tíu|ellefu|tólf|þrettán|fjórtán|sautján|átján|nítján|tuttugu)").unwrap(),
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
    fn test_is_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_is_numeral_dictionaries() {
        
        assert!(ZEROTOTWENTY_DICTIONARY.len() > 0, "zeroToTwenty_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_is_numeral_stats() {
        // Generation statistics
        let total_rules = 2;
        let auto_generated = 2;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("is/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/ML/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ml
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: zeroToNine_dictionary
lazy_static! {
    static ref ZEROTONINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("അഞ്ച്", 5);
        
        map.insert("ആറ്", 6);
        
        map.insert("എട്ട്", 8);
        
        map.insert("ഏഴ്", 7);
        
        map.insert("ഒന്ന്", 1);
        
        map.insert("ഒൻപത്", 9);
        
        map.insert("നാല്", 4);
        
        map.insert("പൂജ്യം", 0);
        
        map.insert("മുന്ന്", 3);
        
        map.insert("രണ്ട്", 2);
        
        map
    };
}

// Dictionary: tenToNineteen_dictionary
lazy_static! {
    static ref TENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("പതിനഞ്ച്", 15);
        
        map.insert("പതിനാറ്", 16);
        
        map.insert("പതിനാല്", 14);
        
        map.insert("പതിനെട്ട്", 18);
        
        map.insert("പതിനേഴ്", 17);
        
        map.insert("പതിനൊന്ന്", 11);
        
        map.insert("പതിമൂന്ന്", 13);
        
        map.insert("പത്തൊമ്പത്", 19);
        
        map.insert("പത്ത്", 10);
        
        map.insert("പന്ത്രണ്ട്", 12);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("അമ്പത്", 50);
        
        map.insert("അമ്പത്തി", 50);
        
        map.insert("അറുപത്", 60);
        
        map.insert("അറുപത്തി", 60);
        
        map.insert("ഇരുപത്", 20);
        
        map.insert("ഇരുപത്തി", 20);
        
        map.insert("എഴുപത്", 70);
        
        map.insert("എഴുപത്തി", 70);
        
        map.insert("എൺപത്", 80);
        
        map.insert("എൺപത്തി", 80);
        
        map.insert("തൊണ്ണൂറ്റി", 90);
        
        map.insert("തൊണ്ണൂറ്", 90);
        
        map.insert("നാല്പത്", 40);
        
        map.insert("നാല്പത്തി", 40);
        
        map.insert("മുപ്പത്", 30);
        
        map.insert("മുപ്പത്തി", 30);
        
        map
    };
}


/// Build Numeral rules for ml locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: zeroToNine_dictionary
    // Examples: പൂജ്യം, ഒന്ന്, രണ്ട്, മുന്ന്, നാല്
    {
        let dict = &*ZEROTONINE_DICTIONARY;
        b.rule_1_terminal(
            "ml:zeroToNine_dictionary",
            b.reg(r"(?i)അഞ്ച്|ആറ്|എട്ട്|ഏഴ്|ഒന്ന്|ഒൻപത്|നാല്|പൂജ്യം|മുന്ന്|രണ്ട്").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tenToNineteen_dictionary
    // Examples: പത്ത്, പതിനൊന്ന്, പന്ത്രണ്ട്, പതിമൂന്ന്, പതിനാല്
    {
        let dict = &*TENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ml:tenToNineteen_dictionary",
            b.reg(r"(?i)പതിനഞ്ച്|പതിനാറ്|പതിനാല്|പതിനെട്ട്|പതിനേഴ്|പതിനൊന്ന്|പതിമൂന്ന്|പത്തൊമ്പത്|പത്ത്|പന്ത്രണ്ട്").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ഇരുപത്, ഇരുപത്തി, മുപ്പത്, മുപ്പത്തി, നാല്പത്
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ml:tens_dictionary",
            b.reg(r"(?i)അമ്പത്|അമ്പത്തി|അറുപത്|അറുപത്തി|ഇരുപത്|ഇരുപത്തി|എഴുപത്|എഴുപത്തി|എൺപത്|എൺപത്തി|തൊണ്ണൂറ്റി|തൊണ്ണൂറ്|നാല്പത്|നാല്പത്തി|മുപ്പത്|മുപ്പത്തി").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (4)
    // ========================================
    

    // Rule: ZeroToNine (refs: zeroToNineMap)
    {
        
        
        let dict = &*ZEROTONINE_DICTIONARY;
        b.rule_1_terminal(
            "ml:ZeroToNine",
            b.reg(r"(?i)(പൂജ്യം|ഒന്ന്|രണ്ട്|മുന്ന്|നാല്|അഞ്ച്|ആറ്|ഏഴ്|എട്ട്|ഒൻപത്)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: TenToNineteen (refs: tenToNineteenMap)
    {
        
        
        let dict = &*TENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ml:TenToNineteen",
            b.reg(r"(?i)(പത്ത്|പതിനൊന്ന്|പന്ത്രണ്ട്|പതിമൂന്ന്|പതിനാല്|പതിനഞ്ച്|പതിനാറ്|പതിനേഴ്|പതിനെട്ട്|പത്തൊമ്പത്)").unwrap(),
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
            "ml:Tens",
            b.reg(r"(?i)(ഇരുപത്|മുപ്പത്|നാല്പത്|അമ്പത്|അറുപത്|എഴുപത്|എൺപത്|തൊണ്ണൂറ്)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: CompositeTens (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ml:CompositeTens",
            b.reg(r"(?i)(ഇരുപത്തി|മുപ്പത്തി|നാല്പത്തി|അമ്പത്തി|അറുപത്തി|എഴുപത്തി|എൺപത്തി|തൊണ്ണൂറ്റി)(ഒന്ന്|രണ്ട്|മുന്ന്|നാല്|അഞ്ച്|ആറ്|ഏഴ്|എട്ട്|ഒൻപത്)").unwrap(),
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
    fn test_ml_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ml_numeral_dictionaries() {
        
        assert!(ZEROTONINE_DICTIONARY.len() > 0, "zeroToNine_dictionary should not be empty");
        
        assert!(TENTONINETEEN_DICTIONARY.len() > 0, "tenToNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ml_numeral_stats() {
        // Generation statistics
        let total_rules = 7;
        let auto_generated = 7;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ml/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

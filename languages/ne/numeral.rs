// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/NE/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ne
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: zeroToNineteen_dictionary
lazy_static! {
    static ref ZEROTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("अठार", 18);
        
        map.insert("आठ", 8);
        
        map.insert("उन्नाइस", 19);
        
        map.insert("एक", 1);
        
        map.insert("एघार", 11);
        
        map.insert("चार", 4);
        
        map.insert("चौध", 14);
        
        map.insert("छ", 6);
        
        map.insert("तीन", 3);
        
        map.insert("तेह्र", 13);
        
        map.insert("दश", 10);
        
        map.insert("दुई", 2);
        
        map.insert("नौ", 9);
        
        map.insert("पन्ध्र", 15);
        
        map.insert("पाँच", 5);
        
        map.insert("बाह्र", 12);
        
        map.insert("शुन्य", 0);
        
        map.insert("सत्र", 17);
        
        map.insert("सात", 7);
        
        map.insert("सुन्ना", 0);
        
        map.insert("सोह्र", 16);
        
        map
    };
}

// Dictionary: twentyoneToTwentynine_dictionary
lazy_static! {
    static ref TWENTYONETOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("अट्ठाइस", 28);
        
        map.insert("उनन्तिस", 29);
        
        map.insert("एक्काइस", 21);
        
        map.insert("चौबिस", 24);
        
        map.insert("छब्बिस", 26);
        
        map.insert("तेइस", 23);
        
        map.insert("पच्चिस", 25);
        
        map.insert("बाइस", 22);
        
        map.insert("सत्ताइस", 27);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("असी", 80);
        
        map.insert("चालिस", 40);
        
        map.insert("तिस", 30);
        
        map.insert("नब्बे", 90);
        
        map.insert("पचास", 50);
        
        map.insert("बिस", 20);
        
        map.insert("सत्तरी", 70);
        
        map.insert("साठी", 60);
        
        map
    };
}


/// Build Numeral rules for ne locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: zeroToNineteen_dictionary
    // Examples: शुन्य, सुन्ना, एक, दुई, तीन
    {
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ne:zeroToNineteen_dictionary",
            b.reg(r"(?i)अठार|आठ|उन्नाइस|एक|एघार|चार|चौध|छ|तीन|तेह्र|दश|दुई|नौ|पन्ध्र|पाँच|बाह्र|शुन्य|सत्र|सात|सुन्ना|सोह्र").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyoneToTwentynine_dictionary
    // Examples: एक्काइस, बाइस, तेइस, चौबिस, पच्चिस
    {
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "ne:twentyoneToTwentynine_dictionary",
            b.reg(r"(?i)अट्ठाइस|उनन्तिस|एक्काइस|चौबिस|छब्बिस|तेइस|पच्चिस|बाइस|सत्ताइस").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: बिस, तिस, चालिस, पचास, साठी
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ne:tens_dictionary",
            b.reg(r"(?i)असी|चालिस|तिस|नब्बे|पचास|बिस|सत्तरी|साठी").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (3)
    // ========================================
    

    // Rule: ToNineteen (refs: zeroToNineteenMap)
    {
        
        
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ne:ToNineteen",
            b.reg(r"(?i)(शुन्य|सुन्ना|एक|दुई|तीन|चार|पाँच|छ|सात|आठ|नौ|दश|एघार|बाह्र|तेह्र|चौध|पन्ध्र|सोह्र|सत्र|अठार|उन्नाइस)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: TwentyoneToTwentynine (refs: twentyoneToTwentynineMap)
    {
        
        
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "ne:TwentyoneToTwentynine",
            b.reg(r"(?i)(एक्काइस|बाइस|तेइस|चौबिस|पच्चिस|छब्बिस|सत्ताइस|अट्ठाइस|उनन्तिस)").unwrap(),
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
            "ne:Tens",
            b.reg(r"(?i)(बिस|तिस|चालिस|पचास|साठी|सत्तरी|असी|नब्बे)").unwrap(),
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
    fn test_ne_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ne_numeral_dictionaries() {
        
        assert!(ZEROTONINETEEN_DICTIONARY.len() > 0, "zeroToNineteen_dictionary should not be empty");
        
        assert!(TWENTYONETOTWENTYNINE_DICTIONARY.len() > 0, "twentyoneToTwentynine_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ne_numeral_stats() {
        // Generation statistics
        let total_rules = 6;
        let auto_generated = 6;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ne/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

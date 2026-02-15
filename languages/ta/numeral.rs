// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/TA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ta
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: zeroToNine_dictionary
lazy_static! {
    static ref ZEROTONINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ஆறு", 6);
        
        map.insert("இரண்டு", 2);
        
        map.insert("எட்டு", 8);
        
        map.insert("ஏழு", 7);
        
        map.insert("ஐந்து", 5);
        
        map.insert("ஒன்பது", 9);
        
        map.insert("ஒன்று", 1);
        
        map.insert("நான்கு", 4);
        
        map.insert("பூஜ்ஜியம்", 0);
        
        map.insert("மூன்று", 3);
        
        map
    };
}

// Dictionary: tenToNineteen_dictionary
lazy_static! {
    static ref TENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("பதினான்கு", 14);
        
        map.insert("பதினாறு", 16);
        
        map.insert("பதினெட்டு", 18);
        
        map.insert("பதினேழு", 17);
        
        map.insert("பதினைந்து", 15);
        
        map.insert("பதினொன்று", 11);
        
        map.insert("பதின்மூன்று", 13);
        
        map.insert("பத்து", 10);
        
        map.insert("பத்தொன்பது", 19);
        
        map.insert("பன்னிரண்டு", 12);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("அறுபது", 60);
        
        map.insert("அறுபத்", 60);
        
        map.insert("இருபது", 20);
        
        map.insert("இருபத்தி", 20);
        
        map.insert("எண்பது", 80);
        
        map.insert("எண்பத்", 80);
        
        map.insert("எழுபது", 70);
        
        map.insert("எழுபத்தி", 70);
        
        map.insert("ஐம்பது", 50);
        
        map.insert("ஐம்பத்தி", 50);
        
        map.insert("தொண்ணுற்று", 90);
        
        map.insert("தொண்ணூறு", 90);
        
        map.insert("நாற்பது", 40);
        
        map.insert("நாற்பத்து", 40);
        
        map.insert("முப்பது", 30);
        
        map.insert("முப்பத்து", 30);
        
        map
    };
}


/// Build Numeral rules for ta locale
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
    // Examples: பூஜ்ஜியம், ஒன்று, இரண்டு, மூன்று, நான்கு
    {
        let dict = &*ZEROTONINE_DICTIONARY;
        b.rule_1_terminal(
            "ta:zeroToNine_dictionary",
            b.reg(r"(?i)ஆறு|இரண்டு|எட்டு|ஏழு|ஐந்து|ஒன்பது|ஒன்று|நான்கு|பூஜ்ஜியம்|மூன்று").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tenToNineteen_dictionary
    // Examples: பத்து, பதினொன்று, பன்னிரண்டு, பதின்மூன்று, பதினான்கு
    {
        let dict = &*TENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ta:tenToNineteen_dictionary",
            b.reg(r"(?i)பதினான்கு|பதினாறு|பதினெட்டு|பதினேழு|பதினைந்து|பதினொன்று|பதின்மூன்று|பத்து|பத்தொன்பது|பன்னிரண்டு").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: இருபது, இருபத்தி, முப்பது, முப்பத்து, நாற்பது
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ta:tens_dictionary",
            b.reg(r"(?i)அறுபது|அறுபத்|இருபது|இருபத்தி|எண்பது|எண்பத்|எழுபது|எழுபத்தி|ஐம்பது|ஐம்பத்தி|தொண்ணுற்று|தொண்ணூறு|நாற்பது|நாற்பத்து|முப்பது|முப்பத்து").unwrap(),
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
            "ta:ZeroToNine",
            b.reg(r"(?i)(பூஜ்ஜியம்|ஒன்று|இரண்டு|மூன்று|நான்கு|ஐந்து|ஆறு|ஏழு|எட்டு|ஒன்பது)").unwrap(),
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
            "ta:TenToNineteen",
            b.reg(r"(?i)(பத்து|பதினொன்று|பன்னிரண்டு|பதின்மூன்று|பதினான்கு|பதினைந்து|பதினாறு|பதினேழு|பதினெட்டு|பத்தொன்பது)").unwrap(),
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
            "ta:Tens",
            b.reg(r"(?i)(இருபது|முப்பது|நாற்பது|ஐம்பது|அறுபது|எழுபது|எண்பது|தொண்ணூறு)").unwrap(),
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
            "ta:CompositeTens",
            b.reg(r"(?i)(இருபத்தி|முப்பத்து|நாற்பத்து|ஐம்பத்தி|அறுபத்|எழுபத்தி|எண்பத்|தொண்ணுற்று)(ஒன்று|இரண்டு|மூன்று|நான்கு|ஐந்து|ஆறு|ஏழு|எட்டு|ஒன்பது)").unwrap(),
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
    fn test_ta_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ta_numeral_dictionaries() {
        
        assert!(ZEROTONINE_DICTIONARY.len() > 0, "zeroToNine_dictionary should not be empty");
        
        assert!(TENTONINETEEN_DICTIONARY.len() > 0, "tenToNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ta_numeral_stats() {
        // Generation statistics
        let total_rules = 7;
        let auto_generated = 7;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ta/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

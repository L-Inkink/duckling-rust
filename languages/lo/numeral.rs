// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/LO/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: lo
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ສອງ", 2);
        
        map.insert("ສາມ", 3);
        
        map.insert("ສິບ", 10);
        
        map.insert("ສີ່", 4);
        
        map.insert("ສູນ", 0);
        
        map.insert("ຫົກ", 6);
        
        map.insert("ຫ້າ", 5);
        
        map.insert("ເກົ້າ", 9);
        
        map.insert("ເຈັດ", 7);
        
        map.insert("ແປດ", 8);
        
        map.insert("ໜຶ່ງ", 1);
        
        map
    };
}

// Dictionary: elevenToNineteen_dictionary
lazy_static! {
    static ref ELEVENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ສິບສອງ", 12);
        
        map.insert("ສິບສາມ", 13);
        
        map.insert("ສິບສີ່", 14);
        
        map.insert("ສິບຫົກ", 16);
        
        map.insert("ສິບຫ້າ", 15);
        
        map.insert("ສິບເກົ້າ", 19);
        
        map.insert("ສິບເຈັດ", 17);
        
        map.insert("ສິບເອັດ", 11);
        
        map.insert("ສິບແປດ", 18);
        
        map
    };
}

// Dictionary: twentyoneToTwentynine_dictionary
lazy_static! {
    static ref TWENTYONETOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ຊາວສອງ", 22);
        
        map.insert("ຊາວສາມ", 23);
        
        map.insert("ຊາວສີ່", 24);
        
        map.insert("ຊາວຫົກ", 26);
        
        map.insert("ຊາວຫ້າ", 25);
        
        map.insert("ຊາວເກົ້າ", 29);
        
        map.insert("ຊາວເຈັດ", 27);
        
        map.insert("ຊາວເອັດ", 21);
        
        map.insert("ຊາວແປດ", 28);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ຊາວ", 20);
        
        map.insert("ສາມສິບ", 30);
        
        map.insert("ສິບສີ່", 40);
        
        map.insert("ຫົກສິບ", 60);
        
        map.insert("ຫ້າສິບ", 50);
        
        map.insert("ເກົ້າສິບ", 90);
        
        map.insert("ເຈັດສິບ", 70);
        
        map.insert("ແປດສິບ", 80);
        
        map
    };
}


/// Build Numeral rules for lo locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 5 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: ສູນ, ໜຶ່ງ, ສອງ, ສາມ, ສີ່
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "lo:ruleNumeral_dictionary",
            b.reg(r"(?i)ສອງ|ສາມ|ສິບ|ສີ່|ສູນ|ຫົກ|ຫ້າ|ເກົ້າ|ເຈັດ|ແປດ|ໜຶ່ງ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenToNineteen_dictionary
    // Examples: ສິບເອັດ, ສິບສອງ, ສິບສາມ, ສິບສີ່, ສິບຫ້າ
    {
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "lo:elevenToNineteen_dictionary",
            b.reg(r"(?i)ສິບສອງ|ສິບສາມ|ສິບສີ່|ສິບຫົກ|ສິບຫ້າ|ສິບເກົ້າ|ສິບເຈັດ|ສິບເອັດ|ສິບແປດ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyoneToTwentynine_dictionary
    // Examples: ຊາວເອັດ, ຊາວສອງ, ຊາວສາມ, ຊາວສີ່, ຊາວຫ້າ
    {
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "lo:twentyoneToTwentynine_dictionary",
            b.reg(r"(?i)ຊາວສອງ|ຊາວສາມ|ຊາວສີ່|ຊາວຫົກ|ຊາວຫ້າ|ຊາວເກົ້າ|ຊາວເຈັດ|ຊາວເອັດ|ຊາວແປດ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ຊາວ, ສາມສິບ, ສິບສີ່, ຫ້າສິບ, ຫົກສິບ
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "lo:tens_dictionary",
            b.reg(r"(?i)ຊາວ|ສາມສິບ|ສິບສີ່|ຫົກສິບ|ຫ້າສິບ|ເກົ້າສິບ|ເຈັດສິບ|ແປດສິບ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (5)
    // ========================================
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "lo:Numeral",
            b.reg(r"(?i)(ສູນ|ໜຶ່ງ|ສອງ|ສາມ|ສີ່|ຫ້າ|ຫົກ|ເຈັດ|ແປດ|ເກົ້າ|ສິບ)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: ElevenToNineteen (refs: elevenToNineteenMap)
    {
        
        
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "lo:ElevenToNineteen",
            b.reg(r"(?i)(ສິບເອັດ|ສິບສອງ|ສິບສາມ|ສິບສີ່|ສິບຫ້າ|ສິບຫົກ|ສິບເຈັດ|ສິບແປດ|ສິບເກົ້າ)").unwrap(),
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
            "lo:TwentyoneToTwentynine",
            b.reg(r"(?i)(ຊາວເອັດ|ຊາວສອງ|ຊາວສາມ|ຊາວສີ່|ຊາວຫ້າ|ຊາວຫົກ|ຊາວເຈັດ|ຊາວແປດ|ຊາວເກົ້າ)").unwrap(),
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
            "lo:Tens",
            b.reg(r"(?i)(ຊາວ|ສາມສິບ|ສິບສີ່|ຫ້າສິບ|ຫົກສິບ|ເຈັດສິບ|ແປດສິບ|ເກົ້າສິບ)").unwrap(),
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
            "lo:CompositeTens",
            b.reg(r"(?i)(ສາມສິບ|ສິບສີ່|ຫ້າສິບ|ຫົກສິບ|ເຈັດສິບ|ແປດສິບ|ເກົ້າສິບ)(ໜຶ່ງ|ສອງ|ສາມ|ສີ່|ຫ້າ|ຫົກ|ເຈັດ|ແປດ|ເກົ້າ)").unwrap(),
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
    fn test_lo_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_lo_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
        assert!(ELEVENTONINETEEN_DICTIONARY.len() > 0, "elevenToNineteen_dictionary should not be empty");
        
        assert!(TWENTYONETOTWENTYNINE_DICTIONARY.len() > 0, "twentyoneToTwentynine_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_lo_numeral_stats() {
        // Generation statistics
        let total_rules = 9;
        let auto_generated = 9;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("lo/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

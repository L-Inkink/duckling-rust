// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/MN/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: mn
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("арав", 10);
        
        map.insert("арван", 10);
        
        map.insert("арван гурав", 13);
        
        map.insert("арван долоо", 17);
        
        map.insert("арван дөрөв", 14);
        
        map.insert("арван ес", 19);
        
        map.insert("арван зургаа", 16);
        
        map.insert("арван найм", 18);
        
        map.insert("арван нэг", 11);
        
        map.insert("арван тав", 15);
        
        map.insert("арван хоёр", 12);
        
        map.insert("ганц", 1);
        
        map.insert("гурав", 3);
        
        map.insert("долоо", 7);
        
        map.insert("дөрөв", 4);
        
        map.insert("ес", 9);
        
        map.insert("зургаа", 6);
        
        map.insert("найм", 8);
        
        map.insert("нойл", 0);
        
        map.insert("нуль", 0);
        
        map.insert("нэг", 1);
        
        map.insert("тав", 5);
        
        map.insert("тэг", 0);
        
        map.insert("хоёр", 2);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("гуч", 30);
        
        map.insert("дал", 70);
        
        map.insert("дөч", 40);
        
        map.insert("ер", 90);
        
        map.insert("жар", 60);
        
        map.insert("ная", 80);
        
        map.insert("тавь", 50);
        
        map.insert("хорь", 20);
        
        map
    };
}


/// Build Numeral rules for mn locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: нуль, тэг, нойл, нэг, ганц
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "mn:zeroNineteen_dictionary",
            b.reg(r"(?i)арав|арван|арван гурав|арван долоо|арван дөрөв|арван ес|арван зургаа|арван найм|арван нэг|арван тав|арван хоёр|ганц|гурав|долоо|дөрөв|ес|зургаа|найм|нойл|нуль|нэг|тав|тэг|хоёр").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: хорь, гуч, дөч, тавь, жар
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "mn:tens_dictionary",
            b.reg(r"(?i)гуч|дал|дөч|ер|жар|ная|тавь|хорь").unwrap(),
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
    

    // Rule: ZeroToNineteen (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "mn:ZeroToNineteen",
            b.reg(r"(?i)(нуль|тэг|нойл|нэг|ганц|хоёр|гурав|дөрөв|тав|зургаа|долоо|найм|ес|арван нэг|арван хоёр|арван гурав|арван дөрөв|арван тав|арван зургаа|арван долоо|арван найм|арван ес|арав|арван)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "mn:Integer2",
            b.reg(r"(?i)(хорь|гуч|дөч|тавь|жар|дал|ная|ер)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (11)
    // ========================================
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: Few (regex)
    
    //   Original: few
    
    // Manual implementation required
    

    // TODO: Ten (regex)
    
    //   Original: ten
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer ([2-9][1-9])
    
    // Manual implementation required
    

    // TODO: Couple (regex)
    
    //   Original: couple
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: NumeralsUnd (composite)
    
    //   Original: numbers und
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    
    eprintln!("⚠️  mn/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_mn_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_mn_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_mn_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 4;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("mn/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

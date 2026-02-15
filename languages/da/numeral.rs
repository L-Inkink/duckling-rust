// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/DA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: da
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("atten", 18);
        
        map.insert("elleve", 11);
        
        map.insert("en", 1);
        
        map.insert("et", 1);
        
        map.insert("fem", 5);
        
        map.insert("femten", 15);
        
        map.insert("fire", 4);
        
        map.insert("fjorten", 14);
        
        map.insert("ingen", 0);
        
        map.insert("intet", 0);
        
        map.insert("ni", 9);
        
        map.insert("nitten", 19);
        
        map.insert("nul", 0);
        
        map.insert("otte", 8);
        
        map.insert("seks", 6);
        
        map.insert("seksten", 16);
        
        map.insert("sytten", 17);
        
        map.insert("syv", 7);
        
        map.insert("ti", 10);
        
        map.insert("to", 2);
        
        map.insert("tolv", 12);
        
        map.insert("tre", 3);
        
        map.insert("tretten", 13);
        
        map.insert("én", 1);
        
        map.insert("ét", 1);
        
        map
    };
}


/// Build Numeral rules for da locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 13 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: ingen, nul, intet, en, et
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "da:zeroNineteen_dictionary",
            b.reg(r"(?i)atten|elleve|en|et|fem|femten|fire|fjorten|ingen|intet|ni|nitten|nul|otte|seks|seksten|sytten|syv|ti|to|tolv|tre|tretten|én|ét").unwrap(),
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
    

    // Rule: Integer (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "da:Integer",
            b.reg(r"(?i)(intet|ingen|nul|en|et|én|ét|to|tretten|tre|fire|femten|fem|seksten|seks|syv|otte|nitten|ni|ti|elleve|tolv|fjorten|sytten|atten)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (13)
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
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Single (regex)
    
    //   Original: single
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: APair (regex)
    
    //   Original: a pair
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: Integer2 (regex)
    
    //   Original: integer (20..90)
    
    // Manual implementation required
    

    // TODO: Integer3 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  da/numeral has 13 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_da_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_da_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_da_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 2;
        let manual_needed = 13;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("da/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

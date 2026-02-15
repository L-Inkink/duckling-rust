// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/JA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ja
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: integer_dictionary
lazy_static! {
    static ref INTEGER_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ゼロ", 0);
        
        map.insert("一", 1);
        
        map.insert("七", 7);
        
        map.insert("三", 3);
        
        map.insert("九", 9);
        
        map.insert("二", 2);
        
        map.insert("五", 5);
        
        map.insert("八", 8);
        
        map.insert("六", 6);
        
        map.insert("十", 10);
        
        map.insert("四", 4);
        
        map.insert("零", 0);
        
        map
    };
}


/// Build Numeral rules for ja locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 14 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: integer_dictionary
    // Examples: 零, ゼロ, 一, 二, 三
    {
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "ja:integer_dictionary",
            b.reg(r"(?i)ゼロ|一|七|三|九|二|五|八|六|十|四|零").unwrap(),
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
    

    // Rule: Integer (refs: integerMap)
    {
        
        
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "ja:Integer",
            b.reg(r"(?i)(ゼロ|零|一|二|三|四|五|六|七|八|九|十)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (14)
    // ========================================
    

    // TODO: Integer5 (regex)
    
    //   Original: integer (100)
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: Integer10 (regex)
    
    //   Original: integer (1000..1999)
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Integer13 (regex)
    
    //   Original: integer (10000)
    
    // Manual implementation required
    

    // TODO: Integer6 (regex)
    
    //   Original: integer (100..199)
    
    // Manual implementation required
    

    // TODO: Integer14 (regex)
    
    //   Original: integer (10000..19999)
    
    // Manual implementation required
    

    // TODO: Integer9 (regex)
    
    //   Original: integer (1000)
    
    // Manual implementation required
    

    // TODO: Integer2 (regex)
    
    //   Original: integer (11..19)
    
    // Manual implementation required
    

    // TODO: Integer12 (composite)
    
    //   Original: integer 2001..9999
    
    // Manual implementation required
    

    // TODO: Integer8 (composite)
    
    //   Original: integer 201..999
    
    // Manual implementation required
    

    // TODO: Integer16 (composite)
    
    //   Original: integer 20001..99999
    
    // Manual implementation required
    

    // TODO: Integer4 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ja/numeral has 14 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ja_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ja_numeral_dictionaries() {
        
        assert!(INTEGER_DICTIONARY.len() > 0, "integer_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ja_numeral_stats() {
        // Generation statistics
        let total_rules = 16;
        let auto_generated = 2;
        let manual_needed = 14;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ja/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

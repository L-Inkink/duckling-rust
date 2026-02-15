// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/ID/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: id
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: ruleInteger_dictionary
lazy_static! {
    static ref RULEINTEGER_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("delapan", 8);
        
        map.insert("dua", 2);
        
        map.insert("empat", 4);
        
        map.insert("enam", 6);
        
        map.insert("kosong", 0);
        
        map.insert("lima", 5);
        
        map.insert("nol", 0);
        
        map.insert("satu", 1);
        
        map.insert("sebelas", 11);
        
        map.insert("sembilan", 9);
        
        map.insert("tiga", 3);
        
        map.insert("tujuh", 7);
        
        map
    };
}


/// Build Numeral rules for id locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: ruleInteger_dictionary
    // Examples: kosong, nol, satu, dua, tiga
    {
        let dict = &*RULEINTEGER_DICTIONARY;
        b.rule_1_terminal(
            "id:ruleInteger_dictionary",
            b.reg(r"(?i)delapan|dua|empat|enam|kosong|lima|nol|satu|sebelas|sembilan|tiga|tujuh").unwrap(),
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
    

    // Rule: Integer (refs: ruleIntegerMap)
    {
        
        
        let dict = &*RULEINTEGER_DICTIONARY;
        b.rule_1_terminal(
            "id:Integer",
            b.reg(r"(?i)(kosong|nol|satu|dua|tiga|empat|lima|enam|tujuh|delapan|sembilan|sebelas)").unwrap(),
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
    

    // TODO: Ten (regex)
    
    //   Original: ten
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Somefewcouple (regex)
    
    //   Original: some/few/couple
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Integer3 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Integer2 (composite)
    
    //   Original: integer 20..90
    
    // Manual implementation required
    

    
    eprintln!("⚠️  id/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_id_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_id_numeral_dictionaries() {
        
        assert!(RULEINTEGER_DICTIONARY.len() > 0, "ruleInteger_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_id_numeral_stats() {
        // Generation statistics
        let total_rules = 13;
        let auto_generated = 2;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("id/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/AR/EG/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ar
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  





/// Build Numeral rules for ar locale
///
/// Auto-generated rules:
///   - 0 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 29 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (1)
    // ========================================
    

    // Rule: Integer30_80 (refs: digitsMap)
    {
        
        
        let dict = &*DIGITS_DICTIONARY;
        b.rule_1_terminal(
            "ar:Integer30_80",
            b.reg(r"(?i)([تط]لا[تط]|[تط]مان)(ين)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (29)
    // ========================================
    

    // TODO: Integer2 (regex)
    
    //   Original: integer 2
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer 3
    
    // Manual implementation required
    

    // TODO: Integer8 (regex)
    
    //   Original: integer 8
    
    // Manual implementation required
    

    // TODO: Integer11 (regex)
    
    //   Original: integer 11
    
    // Manual implementation required
    

    // TODO: Integer12 (regex)
    
    //   Original: integer 12
    
    // Manual implementation required
    

    // TODO: Integer13 (regex)
    
    //   Original: integer 13
    
    // Manual implementation required
    

    // TODO: Integer14 (regex)
    
    //   Original: integer 14
    
    // Manual implementation required
    

    // TODO: Integer15 (regex)
    
    //   Original: integer 15
    
    // Manual implementation required
    

    // TODO: Integer16 (regex)
    
    //   Original: integer 16
    
    // Manual implementation required
    

    // TODO: Integer17 (regex)
    
    //   Original: integer 17
    
    // Manual implementation required
    

    // TODO: Integer18 (regex)
    
    //   Original: integer 18
    
    // Manual implementation required
    

    // TODO: Integer19 (regex)
    
    //   Original: integer 19
    
    // Manual implementation required
    

    // TODO: Integer100 (regex)
    
    //   Original: integer (100)
    
    // Manual implementation required
    

    // TODO: Integer200 (regex)
    
    //   Original: integer (200)
    
    // Manual implementation required
    

    // TODO: Integer300 (regex)
    
    //   Original: integer (300)
    
    // Manual implementation required
    

    // TODO: Integer400 (regex)
    
    //   Original: integer (400)
    
    // Manual implementation required
    

    // TODO: Integer500 (regex)
    
    //   Original: integer (500)
    
    // Manual implementation required
    

    // TODO: Integer600 (regex)
    
    //   Original: integer (600)
    
    // Manual implementation required
    

    // TODO: Integer700 (regex)
    
    //   Original: integer (700)
    
    // Manual implementation required
    

    // TODO: Integer800 (regex)
    
    //   Original: integer (800)
    
    // Manual implementation required
    

    // TODO: Integer900 (regex)
    
    //   Original: integer (900)
    
    // Manual implementation required
    

    // TODO: Integer3000 (regex)
    
    //   Original: integer (3000)
    
    // Manual implementation required
    

    // TODO: Integer4000 (regex)
    
    //   Original: integer (4000)
    
    // Manual implementation required
    

    // TODO: Integer5000 (regex)
    
    //   Original: integer (5000)
    
    // Manual implementation required
    

    // TODO: Integer6000 (regex)
    
    //   Original: integer (6000)
    
    // Manual implementation required
    

    // TODO: Integer7000 (regex)
    
    //   Original: integer (7000)
    
    // Manual implementation required
    

    // TODO: Integer8000 (regex)
    
    //   Original: integer (8000)
    
    // Manual implementation required
    

    // TODO: Integer9000 (regex)
    
    //   Original: integer (9000)
    
    // Manual implementation required
    

    // TODO: Integer101_999 (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ar/numeral has 29 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ar_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    

    #[test]
    fn test_ar_numeral_stats() {
        // Generation statistics
        let total_rules = 30;
        let auto_generated = 1;
        let manual_needed = 29;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ar/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

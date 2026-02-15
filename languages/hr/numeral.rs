// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/HR/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: hr
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  





/// Build Numeral rules for hr locale
///
/// Auto-generated rules:
///   - 0 dictionary rules
///   - 0 constant regex rules
///   - 0 dictionary-reference regex rules
///   - 17 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    

    

    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (17)
    // ========================================
    

    // TODO: NumbersPrefixWithNegativeOrMinus (regex)
    
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
    

    // TODO: DecimalNumber (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer (100..900)
    
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
    

    // TODO: Integer (regex)
    
    //   Original: integer (0..19)
    
    // Manual implementation required
    

    // TODO: Integer2 (regex)
    
    //   Original: integer (20..90)
    
    // Manual implementation required
    

    // TODO: NumbersI (composite)
    
    //   Original: numbers i
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Integer4 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Numbers (composite)
    
    //   Original: numbers 100..999
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  hr/numeral has 17 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_hr_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    

    #[test]
    fn test_hr_numeral_stats() {
        // Generation statistics
        let total_rules = 17;
        let auto_generated = 0;
        let manual_needed = 17;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("hr/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/HE/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: he
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::RuleSetBuilder;








  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  





/// Build Numeral rules for he locale
///
/// Auto-generated rules:
///   - 0 dictionary rules
///   - 0 constant regex rules
///   - 0 dictionary-reference regex rules
///   - 26 complex rules (manual implementation required)
pub fn rules(_b: &RuleSetBuilder<Value>) {
    

    

    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (26)
    // ========================================
    

    // TODO: Integer5 (regex)
    
    //   Original: integer 4
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: Integer10 (regex)
    
    //   Original: integer 9
    
    // Manual implementation required
    

    // TODO: Integer15 (regex)
    
    //   Original: integer (20..90)
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer 2
    
    // Manual implementation required
    

    // TODO: Single (regex)
    
    //   Original: single
    
    // Manual implementation required
    

    // TODO: Integer13 (regex)
    
    //   Original: integer 12
    
    // Manual implementation required
    

    // TODO: Integer6 (regex)
    
    //   Original: integer 5
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Integer7 (regex)
    
    //   Original: integer 6
    
    // Manual implementation required
    

    // TODO: Integer8 (regex)
    
    //   Original: integer 7
    
    // Manual implementation required
    

    // TODO: Couple (regex)
    
    //   Original: couple
    
    // Manual implementation required
    

    // TODO: Integer9 (regex)
    
    //   Original: integer 8
    
    // Manual implementation required
    

    // TODO: Integer (regex)
    
    //   Original: integer 0
    
    // Manual implementation required
    

    // TODO: Integer4 (regex)
    
    //   Original: integer 3
    
    // Manual implementation required
    

    // TODO: Integer2 (regex)
    
    //   Original: integer 1
    
    // Manual implementation required
    

    // TODO: Integer11 (regex)
    
    //   Original: integer 10
    
    // Manual implementation required
    

    // TODO: Commas (regex)
    
    //   Original: comma-separated numbers
    
    // Manual implementation required
    

    // TODO: Half (regex)
    
    //   Original: half
    
    // Manual implementation required
    

    // TODO: IntersectNumerals (composite)
    
    //   Original: intersect numbers
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: CompositeTensWithAnd (composite)
    
    //   Original: integer 21..99 (with and)
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Integer14 (composite)
    
    //   Original: integer 11..19
    
    // Manual implementation required
    

    // TODO: Integer16 (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  he/numeral has 26 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_he_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    

    #[test]
    fn test_he_numeral_stats() {
        // Generation statistics
        let total_rules = 26;
        let auto_generated = 0;
        let manual_needed = 26;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("he/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: unknown
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
      
      
    
  





/// Build Numeral rules for unknown locale
///
/// Auto-generated rules:
///   - 0 dictionary rules
///   - 0 constant regex rules
///   - 0 dictionary-reference regex rules
///   - 1 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    

    

    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (1)
    // ========================================
    

    // TODO: Fractions (regex)
    
    //   Original: fractional number
    
    // Manual implementation required
    

    
    eprintln!("⚠️  unknown/numeral has 1 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_unknown_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    

    #[test]
    fn test_unknown_numeral_stats() {
        // Generation statistics
        let total_rules = 1;
        let auto_generated = 0;
        let manual_needed = 1;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("unknown/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

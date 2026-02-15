// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/VI/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: vi
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  





/// Build Numeral rules for vi locale
///
/// Auto-generated rules:
///   - 0 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (3)
    // ========================================
    

    // Rule: PowersOfTen (refs: powersOfTenMap)
    {
        
        
        let dict = &*POWERSOFTEN_DICTIONARY;
        b.rule_1_terminal(
            "vi:PowersOfTen",
            b.reg(r"(?i)(chục|trăm|nghìn|ngàn|triệu|t(ỷ|ỉ))").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer (refs: integerMap)
    {
        
        
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "vi:Integer",
            b.reg(r"(?i)(không|một|linh một|lẻ một|hai|linh hai|lẻ hai|ba|linh ba|lẻ ba|bốn|linh bốn|lẻ bốn|năm|linh năm|lẻ năm|sáu|lẻ sáu|linh sáu|bảy|lẻ bảy|linh bảy|tám|linh tám|lẻ tám|chín|linh chín|lẻ chín|mười một|mười hai|mười ba|mười bốn|mười lăm|mười sáu|mười bảy|mười tám|mười chín|mười|linh mười)").unwrap(),
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
            "vi:Integer2",
            b.reg(r"(?i)(hai mươi|ba mươi|bốn mươi|năm mươi|sáu mươi|bảy mươi|tám mươi|chín mươi)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (9)
    // ========================================
    

    // TODO: NumeralsPrefixWithM (regex)
    
    //   Original: numbers prefix with -, âm
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: T (regex)
    
    //   Original: tá
    
    // Manual implementation required
    

    // TODO: Numerals2 (composite)
    
    //   Original: numbers 25 35 45 55 65 75 85 95
    
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
    

    // TODO: Numerals (composite)
    
    //   Original: numbers 21 31 41 51 61 71 81 91
    
    // Manual implementation required
    

    
    eprintln!("⚠️  vi/numeral has 9 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_vi_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    

    #[test]
    fn test_vi_numeral_stats() {
        // Generation statistics
        let total_rules = 12;
        let auto_generated = 3;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("vi/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

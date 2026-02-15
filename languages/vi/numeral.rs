// Auto-generated from Duckling/Numeral/VI/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: vi
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: integer_dictionary
lazy_static! {
    static ref INTEGER_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ba", 3);
        
        map.insert("bảy", 7);
        
        map.insert("bốn", 4);
        
        map.insert("chín", 9);
        
        map.insert("hai", 2);
        
        map.insert("không", 0);
        
        map.insert("linh ba", 3);
        
        map.insert("linh bảy", 7);
        
        map.insert("linh bốn", 4);
        
        map.insert("linh chín", 9);
        
        map.insert("linh hai", 2);
        
        map.insert("linh mười", 10);
        
        map.insert("linh một", 1);
        
        map.insert("linh năm", 5);
        
        map.insert("linh sáu", 6);
        
        map.insert("linh tám", 8);
        
        map.insert("lẻ", 3);
        
        map.insert("lẻ bảy", 7);
        
        map.insert("lẻ bốn", 4);
        
        map.insert("lẻ chín", 9);
        
        map.insert("lẻ hai", 2);
        
        map.insert("lẻ mười", 10);
        
        map.insert("lẻ một", 1);
        
        map.insert("lẻ năm", 5);
        
        map.insert("lẻ sáu", 6);
        
        map.insert("lẻ tám", 8);
        
        map.insert("mười", 10);
        
        map.insert("mười ba", 13);
        
        map.insert("mười bảy", 17);
        
        map.insert("mười bốn", 14);
        
        map.insert("mười chín", 19);
        
        map.insert("mười hai", 12);
        
        map.insert("mười lăm", 15);
        
        map.insert("mười một", 11);
        
        map.insert("mười sáu", 16);
        
        map.insert("mười tám", 18);
        
        map.insert("một", 1);
        
        map.insert("năm", 5);
        
        map.insert("sáu", 6);
        
        map.insert("tám", 8);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ba mươi", 30);
        
        map.insert("bảy mươi", 70);
        
        map.insert("bốn mươi", 40);
        
        map.insert("chín mươi", 90);
        
        map.insert("hai mươi", 20);
        
        map.insert("năm mươi", 50);
        
        map.insert("sáu mươi", 60);
        
        map.insert("tám mươi", 80);
        
        map
    };
}


/// Build Numeral rules for vi locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 1 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: integer_dictionary
    // Examples: không, một, linh một, lẻ một, hai
    {
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "vi:integer_dictionary",
            b.reg(r"(?i)ba|bảy|bốn|chín|hai|không|linh ba|linh bảy|linh bốn|linh chín|linh hai|linh mười|linh một|linh năm|linh sáu|linh tám|lẻ|lẻ bảy|lẻ bốn|lẻ chín|lẻ hai|lẻ mười|lẻ một|lẻ năm|lẻ sáu|lẻ tám|mười|mười ba|mười bảy|mười bốn|mười chín|mười hai|mười lăm|mười một|mười sáu|mười tám|một|năm|sáu|tám").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: hai mươi, ba mươi, bốn mươi, năm mươi, sáu mươi
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "vi:tens_dictionary",
            b.reg(r"(?i)ba mươi|bảy mươi|bốn mươi|chín mươi|hai mươi|năm mươi|sáu mươi|tám mươi").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Constant Value Regex Rules (1)
    // ========================================
    

    // Rule: T → 12
    b.rule_1_terminal(
        "vi:T",
        b.reg(r"(?i)tá").unwrap(),
        |_| {
            Ok(Value::Integer(12))
        }
    );
    
    

    
    // ========================================
    // Dictionary-Reference Regex Rules (2)
    // ========================================
    

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
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithM (regex)
    
    //   Original: numbers prefix with -, âm
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
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
    fn test_vi_numeral_dictionaries() {
        
        assert!(INTEGER_DICTIONARY.len() > 0, "integer_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_vi_numeral_stats() {
        // Generation statistics
        let total_rules = 14;
        let auto_generated = 5;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("vi/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

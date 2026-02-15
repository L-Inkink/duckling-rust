// Auto-generated from Duckling/Numeral/AR/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ar
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: digits_dictionary
lazy_static! {
    static ref DIGITS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("أربع", 4);
        
        map.insert("اربع", 4);
        
        map.insert("تسع", 9);
        
        map.insert("ثلاث", 3);
        
        map.insert("ثمان", 8);
        
        map.insert("خمس", 5);
        
        map.insert("سبع", 7);
        
        map.insert("ست", 6);
        
        map.insert("عشر", 2);
        
        map
    };
}


/// Build Numeral rules for ar locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 19 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: digits_dictionary
    // Examples: عشر, ثلاث, اربع, أربع, خمس
    {
        let dict = &*DIGITS_DICTIONARY;
        b.rule_1_terminal(
            "ar:digits_dictionary",
            b.reg(r"(?i)أربع|اربع|تسع|ثلاث|ثمان|خمس|سبع|ست|عشر").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Constant Value Regex Rules (19)
    // ========================================
    

    // Rule: Integer5 → 4
    b.rule_1_terminal(
        "ar:Integer5",
        b.reg(r"(?i)([أا]ربع[ةه]?)").unwrap(),
        |_| {
            Ok(Value::Integer(4))
        }
    );
    

    // Rule: Integer18 → 12
    b.rule_1_terminal(
        "ar:Integer18",
        b.reg(r"(?i)(?:[إا]?ثن(?:ت)?[يىا] ?عشر[ةه]?)").unwrap(),
        |_| {
            Ok(Value::Integer(12))
        }
    );
    

    // Rule: Integer300 → 300
    b.rule_1_terminal(
        "ar:Integer300",
        b.reg(r"(?i)(ثلاث)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(300))
        }
    );
    

    // Rule: Integer400 → 400
    b.rule_1_terminal(
        "ar:Integer400",
        b.reg(r"(?i)([أا]ربع)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(400))
        }
    );
    

    // Rule: Integer500 → 500
    b.rule_1_terminal(
        "ar:Integer500",
        b.reg(r"(?i)(خمس)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(500))
        }
    );
    

    // Rule: Integer600 → 600
    b.rule_1_terminal(
        "ar:Integer600",
        b.reg(r"(?i)(ست)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(600))
        }
    );
    

    // Rule: Integer700 → 700
    b.rule_1_terminal(
        "ar:Integer700",
        b.reg(r"(?i)(سبع)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(700))
        }
    );
    

    // Rule: Integer800 → 800
    b.rule_1_terminal(
        "ar:Integer800",
        b.reg(r"(?i)(ثمان[ي]?)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(800))
        }
    );
    

    // Rule: Integer900 → 900
    b.rule_1_terminal(
        "ar:Integer900",
        b.reg(r"(?i)(تسع)ما?[ئي][ةه]").unwrap(),
        |_| {
            Ok(Value::Integer(900))
        }
    );
    

    // Rule: Integer15 → 11
    b.rule_1_terminal(
        "ar:Integer15",
        b.reg(r"(?i)([إاأ]حد[يى]? عشر[ةه]?)").unwrap(),
        |_| {
            Ok(Value::Integer(11))
        }
    );
    

    // Rule: Integer3 → 2
    b.rule_1_terminal(
        "ar:Integer3",
        b.reg(r"(?i)[إا]ثنت?[اي]ن").unwrap(),
        |_| {
            Ok(Value::Integer(2))
        }
    );
    

    // Rule: Integer13 → 9
    b.rule_1_terminal(
        "ar:Integer13",
        b.reg(r"(?i)تسع[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(9))
        }
    );
    

    // Rule: Integer12 → 8
    b.rule_1_terminal(
        "ar:Integer12",
        b.reg(r"(?i)ثما??ني?[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(8))
        }
    );
    

    // Rule: Integer7 → 5
    b.rule_1_terminal(
        "ar:Integer7",
        b.reg(r"(?i)خمس[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(5))
        }
    );
    

    // Rule: Integer14 → 10
    b.rule_1_terminal(
        "ar:Integer14",
        b.reg(r"(?i)عشر[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(10))
        }
    );
    

    // Rule: Integer9 → 6
    b.rule_1_terminal(
        "ar:Integer9",
        b.reg(r"(?i)ست[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(6))
        }
    );
    

    // Rule: Integer4 → 3
    b.rule_1_terminal(
        "ar:Integer4",
        b.reg(r"(?i)(ثلاث[ةه]?)").unwrap(),
        |_| {
            Ok(Value::Integer(3))
        }
    );
    

    // Rule: Integer2 → 1
    b.rule_1_terminal(
        "ar:Integer2",
        b.reg(r"(?i)واحد[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(1))
        }
    );
    

    // Rule: Integer11 → 7
    b.rule_1_terminal(
        "ar:Integer11",
        b.reg(r"(?i)سبع[ةه]?").unwrap(),
        |_| {
            Ok(Value::Integer(7))
        }
    );
    
    

    
    // ========================================
    // Dictionary-Reference Regex Rules (1)
    // ========================================
    

    // Rule: Integer19 (refs: digitsMap)
    {
        
        
        let dict = &*DIGITS_DICTIONARY;
        b.rule_1_terminal(
            "ar:Integer19",
            b.reg(r"(?i)(?:عشر|ثلاث|[أا]ربع|خمس|ست|سبع|ثمان|تسع)(?:ون|ين)").unwrap(),
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
    

    // TODO: Integer200 (regex)
    
    //   Original: integer (200)
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithMinus (regex)
    
    //   Original: numbers prefix with -, minus
    
    // Manual implementation required
    

    // TODO: Integer (regex)
    
    //   Original: integer 0
    
    // Manual implementation required
    

    // TODO: FractionsNumeric (regex)
    
    //   Original: Arabic fractional number numeric
    
    // Manual implementation required
    

    // TODO: ArabicDecimal (regex)
    
    //   Original: Arabic decimal number with Arabic decimal separator
    
    // Manual implementation required
    

    // TODO: Integer23 (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    // TODO: Integer21 (composite)
    
    //   Original: integer (13..19)
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ar/numeral has 11 unimplemented complex rules");
    
    
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
    fn test_ar_numeral_dictionaries() {
        
        assert!(DIGITS_DICTIONARY.len() > 0, "digits_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ar_numeral_stats() {
        // Generation statistics
        let total_rules = 32;
        let auto_generated = 21;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ar/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

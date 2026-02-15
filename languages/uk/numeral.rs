// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/UK/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: uk
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: twentyNinety_dictionary
lazy_static! {
    static ref TWENTYNINETY_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("вісімдесят", 80);
        
        map.insert("двадцять", 20);
        
        map.insert("дев‘яносто", 90);
        
        map.insert("п‘ятдесят", 50);
        
        map.insert("сорок", 40);
        
        map.insert("сімдесят", 70);
        
        map.insert("тридцять", 30);
        
        map.insert("шістдесят", 60);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("вісімсот", 800);
        
        map.insert("двісті", 200);
        
        map.insert("дев‘ятсот", 900);
        
        map.insert("п‘ятсот", 500);
        
        map.insert("сто", 100);
        
        map.insert("сімсот", 700);
        
        map.insert("триста", 300);
        
        map.insert("чотириста", 400);
        
        map.insert("шістсот", 600);
        
        map
    };
}

// Dictionary: threeNineteen_dictionary
lazy_static! {
    static ref THREENINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("вісім", 8);
        
        map.insert("вісімнадцять", 18);
        
        map.insert("дванадцять", 12);
        
        map.insert("дев‘ятнадцять", 19);
        
        map.insert("дев‘ять", 9);
        
        map.insert("десять", 10);
        
        map.insert("одинадцять", 11);
        
        map.insert("п‘ятнадцять", 15);
        
        map.insert("п‘ять", 5);
        
        map.insert("сім", 7);
        
        map.insert("сімнадцять", 17);
        
        map.insert("три", 3);
        
        map.insert("тринадцять", 13);
        
        map.insert("чотири", 4);
        
        map.insert("чотирнадцять", 14);
        
        map.insert("шістнадцять", 16);
        
        map.insert("шість", 6);
        
        map
    };
}


/// Build Numeral rules for uk locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 8 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: twentyNinety_dictionary
    // Examples: двадцять, тридцять, сорок, п‘ятдесят, шістдесят
    {
        let dict = &*TWENTYNINETY_DICTIONARY;
        b.rule_1_terminal(
            "uk:twentyNinety_dictionary",
            b.reg(r"(?i)вісімдесят|двадцять|дев‘яносто|п‘ятдесят|сорок|сімдесят|тридцять|шістдесят").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: сто, двісті, триста, чотириста, п‘ятсот
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "uk:hundreds_dictionary",
            b.reg(r"(?i)вісімсот|двісті|дев‘ятсот|п‘ятсот|сто|сімсот|триста|чотириста|шістсот").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: threeNineteen_dictionary
    // Examples: три, чотири, п‘ять, шість, сім
    {
        let dict = &*THREENINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "uk:threeNineteen_dictionary",
            b.reg(r"(?i)вісім|вісімнадцять|дванадцять|дев‘ятнадцять|дев‘ять|десять|одинадцять|п‘ятнадцять|п‘ять|сім|сімнадцять|три|тринадцять|чотири|чотирнадцять|шістнадцять|шість").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (3)
    // ========================================
    

    // Rule: Integer5 (refs: twentyNinetyMap)
    {
        
        
        let dict = &*TWENTYNINETY_DICTIONARY;
        b.rule_1_terminal(
            "uk:Integer5",
            b.reg(r"(?i)(двадцять|тридцять|сорок|п‘ятдесят|шістдесят|сімдесят|вісімдесят|дев‘яносто)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer6 (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "uk:Integer6",
            b.reg(r"(?i)(сто|двісті|триста|чотириста|п‘ятсот|шістсот|сімсот|вісімсот|дев‘ятсот)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer4 (refs: threeNineteenMap)
    {
        
        
        let dict = &*THREENINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "uk:Integer4",
            b.reg(r"(?i)(три|чотирнадцять|чотири|п‘ятнадцять|п‘ять|шістнадцять|шість|сімнадцять|сім|вісімнадцять|вісім|дев‘ятнадцять|дев‘ять|десять|одинадцять|дванадцять|тринадцять)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (8)
    // ========================================
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer 2
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithMinus (regex)
    
    //   Original: numbers prefix with -, minus
    
    // Manual implementation required
    

    // TODO: Integer (regex)
    
    //   Original: integer 0
    
    // Manual implementation required
    

    // TODO: Integer2 (regex)
    
    //   Original: integer 1
    
    // Manual implementation required
    

    // TODO: Integer7 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Integer8 (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  uk/numeral has 8 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_uk_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_uk_numeral_dictionaries() {
        
        assert!(TWENTYNINETY_DICTIONARY.len() > 0, "twentyNinety_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
        assert!(THREENINETEEN_DICTIONARY.len() > 0, "threeNineteen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_uk_numeral_stats() {
        // Generation statistics
        let total_rules = 14;
        let auto_generated = 6;
        let manual_needed = 8;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("uk/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/RU/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ru
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("восемьдесят", 80);
        
        map.insert("восемьдесяти", 80);
        
        map.insert("восьмидесят", 80);
        
        map.insert("восьмидесяти", 80);
        
        map.insert("двадцати", 20);
        
        map.insert("двадцать", 20);
        
        map.insert("девяноста", 90);
        
        map.insert("девяносто", 90);
        
        map.insert("пятидесяти", 50);
        
        map.insert("пятьдесят", 50);
        
        map.insert("семидесят", 70);
        
        map.insert("семидесяти", 70);
        
        map.insert("семьдесят", 70);
        
        map.insert("сорок", 40);
        
        map.insert("сорока", 40);
        
        map.insert("тридцати", 30);
        
        map.insert("тридцать", 30);
        
        map.insert("шестидесят", 60);
        
        map.insert("шестидесяти", 60);
        
        map.insert("шестьдесят", 60);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("восемьсот", 800);
        
        map.insert("двести", 200);
        
        map.insert("девятьсот", 900);
        
        map.insert("пятьсот", 500);
        
        map.insert("семьсот", 700);
        
        map.insert("сто", 100);
        
        map.insert("триста", 300);
        
        map.insert("четыреста", 400);
        
        map.insert("шестьсот", 600);
        
        map
    };
}

// Dictionary: threeToNineteen_dictionary
lazy_static! {
    static ref THREETONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("восемнадцать", 18);
        
        map.insert("восемь", 8);
        
        map.insert("двенадцать", 12);
        
        map.insert("девятнадцать", 19);
        
        map.insert("девять", 9);
        
        map.insert("десять", 10);
        
        map.insert("одиннадцать", 11);
        
        map.insert("пятнадцать", 15);
        
        map.insert("пять", 5);
        
        map.insert("семнадцать", 17);
        
        map.insert("семь", 7);
        
        map.insert("три", 3);
        
        map.insert("тринадцать", 13);
        
        map.insert("четыре", 4);
        
        map.insert("четырнадцать", 14);
        
        map.insert("шестнадцать", 16);
        
        map.insert("шесть", 6);
        
        map
    };
}


/// Build Numeral rules for ru locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: tens_dictionary
    // Examples: двадцать, двадцати, тридцать, тридцати, сорок
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ru:tens_dictionary",
            b.reg(r"(?i)восемьдесят|восемьдесяти|восьмидесят|восьмидесяти|двадцати|двадцать|девяноста|девяносто|пятидесяти|пятьдесят|семидесят|семидесяти|семьдесят|сорок|сорока|тридцати|тридцать|шестидесят|шестидесяти|шестьдесят").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: сто, двести, триста, четыреста, пятьсот
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "ru:hundreds_dictionary",
            b.reg(r"(?i)восемьсот|двести|девятьсот|пятьсот|семьсот|сто|триста|четыреста|шестьсот").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: threeToNineteen_dictionary
    // Examples: три, четыре, пять, шесть, семь
    {
        let dict = &*THREETONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ru:threeToNineteen_dictionary",
            b.reg(r"(?i)восемнадцать|восемь|двенадцать|девятнадцать|девять|десять|одиннадцать|пятнадцать|пять|семнадцать|семь|три|тринадцать|четыре|четырнадцать|шестнадцать|шесть").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (4)
    // ========================================
    

    // Rule: Integer5 (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ru:Integer5",
            b.reg(r"(?i)(двадцат(ь|и)|тридцат(ь|и)|сорока?|пят(ь|и)десяти?|шест(ь|и)десяти?|сем(ь|и)десяти?|вос(е|ь)м(ь|и)десяти?|девяност(о|а))").unwrap(),
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
            "ru:Integer6",
            b.reg(r"(?i)(сто|двести|триста|четыреста|пятьсот|шестьсот|семьсот|восемьсот|девятьсот)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer4 (refs: threeToNineteenMap)
    {
        
        
        let dict = &*THREETONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ru:Integer4",
            b.reg(r"(?i)(три|четырнадцать|четыре|пятнадцать|пять|шестнадцать|шесть|семнадцать|семь|восемнадцать|восемь|девятнадцать|девять|десять|одиннадцать|двенадцать|тринадцать)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer4Genitive (refs: threeToNineteenMap)
    {
        
        
        let dict = &*THREETONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ru:Integer4Genitive",
            b.reg(r"(?i)(трех|четырнадцати|четырех|пятнадцати|пяти|шестнадцати|шести|семнадцати|семи|восемнадцати|восьми|девятнадцати|девяти|десяти|одиннадцати|двенадцати|тринадцати)").unwrap(),
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
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Integer3 (regex)
    
    //   Original: integer 2
    
    // Manual implementation required
    

    // TODO: DecimalOneAndAHalf (regex)
    
    //   Original: decimal one and a half
    
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
    

    
    eprintln!("⚠️  ru/numeral has 9 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ru_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ru_numeral_dictionaries() {
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
        assert!(THREETONINETEEN_DICTIONARY.len() > 0, "threeToNineteen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ru_numeral_stats() {
        // Generation statistics
        let total_rules = 16;
        let auto_generated = 7;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ru/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

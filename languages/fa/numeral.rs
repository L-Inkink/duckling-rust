// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/FA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: fa
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ده", 10);
        
        map.insert("دو", 2);
        
        map.insert("دوازده", 12);
        
        map.insert("سه", 3);
        
        map.insert("سیزده", 13);
        
        map.insert("شانزده", 16);
        
        map.insert("شش", 6);
        
        map.insert("شونزده", 16);
        
        map.insert("شیش", 6);
        
        map.insert("صفر", 0);
        
        map.insert("نه", 9);
        
        map.insert("نوزده", 19);
        
        map.insert("هجده", 18);
        
        map.insert("هشت", 8);
        
        map.insert("هفت", 7);
        
        map.insert("هفده", 17);
        
        map.insert("هیجده", 18);
        
        map.insert("هیفده", 17);
        
        map.insert("پانزده", 15);
        
        map.insert("پنج", 5);
        
        map.insert("پونزده", 15);
        
        map.insert("چهار", 4);
        
        map.insert("چهارده", 14);
        
        map.insert("یازده", 11);
        
        map.insert("یک", 1);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("بیست", 20);
        
        map.insert("دویست", 200);
        
        map.insert("سی", 30);
        
        map.insert("سی صد", 300);
        
        map.insert("سیصد", 300);
        
        map.insert("شش صد", 600);
        
        map.insert("ششصد", 600);
        
        map.insert("شصت", 60);
        
        map.insert("شیش صد", 600);
        
        map.insert("شیشصد", 600);
        
        map.insert("صد", 100);
        
        map.insert("نه صد", 900);
        
        map.insert("نهصد", 900);
        
        map.insert("نود", 90);
        
        map.insert("هشت صد", 800);
        
        map.insert("هشتاد", 80);
        
        map.insert("هشتصد", 800);
        
        map.insert("هفت صد", 700);
        
        map.insert("هفتاد", 70);
        
        map.insert("هفتصد", 700);
        
        map.insert("پانصد", 500);
        
        map.insert("پنجاه", 50);
        
        map.insert("پونصد", 500);
        
        map.insert("چهار صد", 400);
        
        map.insert("چهارصد", 400);
        
        map.insert("چهل", 40);
        
        map
    };
}


/// Build Numeral rules for fa locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 6 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: صفر, یک, دو, سه, چهار
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "fa:zeroNineteen_dictionary",
            b.reg(r"(?i)ده|دو|دوازده|سه|سیزده|شانزده|شش|شونزده|شیش|صفر|نه|نوزده|هجده|هشت|هفت|هفده|هیجده|هیفده|پانزده|پنج|پونزده|چهار|چهارده|یازده|یک").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: بیست, سی, چهل, پنجاه, شصت
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "fa:tens_dictionary",
            b.reg(r"(?i)بیست|دویست|سی|سی صد|سیصد|شش صد|ششصد|شصت|شیش صد|شیشصد|صد|نه صد|نهصد|نود|هشت صد|هشتاد|هشتصد|هفت صد|هفتاد|هفتصد|پانصد|پنجاه|پونصد|چهار صد|چهارصد|چهل").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (2)
    // ========================================
    

    // Rule: ToNineteen (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "fa:ToNineteen",
            b.reg(r"(?i)(صفر|یک|سه|چهارده|چهار|پنج|شی?ش|هفت|هشت|نه|یازده|دوازده|سیزده|پ(ا|و)نزده|ش(ا|و)نزده|هی?فده|هی?جده|نوزده|ده|دو)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Tens (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "fa:Tens",
            b.reg(r"(?i)(دویست|(سی|چهار|پان|پون|شی?ش|هفت|هشت|نه)? ?صد|بیست|سی|چهل|پنجاه|شصت|هفتاد|هشتاد|نود)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (6)
    // ========================================
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: CompositeHundred (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: SumAnd (composite)
    
    //   Original: intersect 2 numbers (with and)
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  fa/numeral has 6 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_fa_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_fa_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_fa_numeral_stats() {
        // Generation statistics
        let total_rules = 10;
        let auto_generated = 4;
        let manual_needed = 6;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("fa/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

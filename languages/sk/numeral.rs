// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/SK/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: sk
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroToNineteen_dictionary
lazy_static! {
    static ref ZEROTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("desať", 10);
        
        map.insert("devätnásť", 19);
        
        map.insert("deväť", 9);
        
        map.insert("dva", 2);
        
        map.insert("dvanásť", 12);
        
        map.insert("dve", 2);
        
        map.insert("jeden", 1);
        
        map.insert("jedenásť", 11);
        
        map.insert("jedna", 1);
        
        map.insert("jedno", 1);
        
        map.insert("nula", 0);
        
        map.insert("osem", 8);
        
        map.insert("osemnásť", 18);
        
        map.insert("pätnásť", 15);
        
        map.insert("päť", 5);
        
        map.insert("sedem", 7);
        
        map.insert("sedemnásť", 17);
        
        map.insert("tri", 3);
        
        map.insert("trinásť", 13);
        
        map.insert("šestnásť", 16);
        
        map.insert("šesť", 6);
        
        map.insert("štrnásť", 14);
        
        map.insert("štyri", 4);
        
        map
    };
}

// Dictionary: dozen_dictionary
lazy_static! {
    static ref DOZEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("devätdesiat", 90);
        
        map.insert("dvadsať", 20);
        
        map.insert("osemdesiat", 80);
        
        map.insert("päťdesiat", 50);
        
        map.insert("sedemdesiat", 70);
        
        map.insert("tridsať", 30);
        
        map.insert("šesťdesiat", 60);
        
        map.insert("štyridsať", 40);
        
        map
    };
}


/// Build Numeral rules for sk locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: zeroToNineteen_dictionary
    // Examples: nula, jeden, jedna, jedno, dva
    {
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "sk:zeroToNineteen_dictionary",
            b.reg(r"(?i)desať|devätnásť|deväť|dva|dvanásť|dve|jeden|jedenásť|jedna|jedno|nula|osem|osemnásť|pätnásť|päť|sedem|sedemnásť|tri|trinásť|šestnásť|šesť|štrnásť|štyri").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: dozen_dictionary
    // Examples: dvadsať, tridsať, štyridsať, päťdesiat, šesťdesiat
    {
        let dict = &*DOZEN_DICTIONARY;
        b.rule_1_terminal(
            "sk:dozen_dictionary",
            b.reg(r"(?i)devätdesiat|dvadsať|osemdesiat|päťdesiat|sedemdesiat|tridsať|šesťdesiat|štyridsať").unwrap(),
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
    

    // Rule: Integer (refs: zeroToNineteenMap)
    {
        
        
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "sk:Integer",
            b.reg(r"(?i)(nula|jed(enásť|en|na|no)|dv(anásť|a|e)|trinásť|tri|štrnásť|štyri|pätnásť|päť|šestnásť|šesť|sedemnásť|sedem|osemnásť|osem|devätnásť|deväť|desať)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: dozenMap)
    {
        
        
        let dict = &*DOZEN_DICTIONARY;
        b.rule_1_terminal(
            "sk:Integer2",
            b.reg(r"(?i)((dva|tri|štyri)dsať|(päť|šesť|sedem|osem|devät)desiat)").unwrap(),
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
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: Few (regex)
    
    //   Original: few
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Single (regex)
    
    //   Original: single
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Couple (regex)
    
    //   Original: couple, a pair
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: IntegerCompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  sk/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_sk_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_sk_numeral_dictionaries() {
        
        assert!(ZEROTONINETEEN_DICTIONARY.len() > 0, "zeroToNineteen_dictionary should not be empty");
        
        assert!(DOZEN_DICTIONARY.len() > 0, "dozen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_sk_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 4;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("sk/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

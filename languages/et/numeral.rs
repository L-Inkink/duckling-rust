// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/ET/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: et
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kaheksa", 8);
        
        map.insert("kaheksateist", 18);
        
        map.insert("kaks", 2);
        
        map.insert("kaksteist", 12);
        
        map.insert("kolm", 3);
        
        map.insert("kolmteist", 13);
        
        map.insert("kuus", 6);
        
        map.insert("kuusteist", 16);
        
        map.insert("kümme", 10);
        
        map.insert("neli", 4);
        
        map.insert("neliteist", 14);
        
        map.insert("null", 0);
        
        map.insert("seitse", 7);
        
        map.insert("seitseteist", 17);
        
        map.insert("viis", 5);
        
        map.insert("viisteist", 15);
        
        map.insert("üheksa", 9);
        
        map.insert("üheksateist", 19);
        
        map.insert("üks", 1);
        
        map.insert("üksteist", 11);
        
        map
    };
}

// Dictionary: twentyNinety_dictionary
lazy_static! {
    static ref TWENTYNINETY_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kaheksakümmend", 80);
        
        map.insert("kakskümmend", 20);
        
        map.insert("kolmkümmend", 30);
        
        map.insert("kuuskümmend", 60);
        
        map.insert("nelikümmend", 40);
        
        map.insert("seitsekümmend", 70);
        
        map.insert("viiskümmend", 50);
        
        map.insert("üheksakümmend", 90);
        
        map
    };
}


/// Build Numeral rules for et locale
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
    

    // Rule: zeroNineteen_dictionary
    // Examples: null, üks, kaks, kolm, neli
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "et:zeroNineteen_dictionary",
            b.reg(r"(?i)kaheksa|kaheksateist|kaks|kaksteist|kolm|kolmteist|kuus|kuusteist|kümme|neli|neliteist|null|seitse|seitseteist|viis|viisteist|üheksa|üheksateist|üks|üksteist").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyNinety_dictionary
    // Examples: kakskümmend, kolmkümmend, nelikümmend, viiskümmend, kuuskümmend
    {
        let dict = &*TWENTYNINETY_DICTIONARY;
        b.rule_1_terminal(
            "et:twentyNinety_dictionary",
            b.reg(r"(?i)kaheksakümmend|kakskümmend|kolmkümmend|kuuskümmend|nelikümmend|seitsekümmend|viiskümmend|üheksakümmend").unwrap(),
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
    

    // Rule: Integer (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "et:Integer",
            b.reg(r"(?i)(null|üksteist|üks|kaksteist|kaks|kolmteist|kolm|neliteist|neli|viisteist|viis|kuusteist|kuus|seitseteist|seitse|kaheksateist|kaheksa|üheksateist|üheksa|kümme)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyNinety (refs: twentyNinetyMap)
    {
        
        
        let dict = &*TWENTYNINETY_DICTIONARY;
        b.rule_1_terminal(
            "et:twentyNinety",
            b.reg(r"(?i)((kaks|kolm|neli|viis|kuus|seitse|kaheksa|(ü)heksa)k(ü)mmend)").unwrap(),
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
    

    // TODO: ACoupleOf (regex)
    
    //   Original: a couple of
    
    // Manual implementation required
    

    // TODO: Ten (regex)
    
    //   Original: ten
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: AFew (regex)
    
    //   Original: (a )?few
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Integer4 (regex)
    
    //   Original: integer (200..900)
    
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
    

    
    eprintln!("⚠️  et/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_et_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_et_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(TWENTYNINETY_DICTIONARY.len() > 0, "twentyNinety_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_et_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 4;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("et/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

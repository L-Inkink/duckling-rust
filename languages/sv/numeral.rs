// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/SV/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: sv
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroToNineteen_dictionary
lazy_static! {
    static ref ZEROTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("arton", 18);
        
        map.insert("elva", 11);
        
        map.insert("en", 1);
        
        map.insert("ett", 1);
        
        map.insert("fem", 5);
        
        map.insert("femton", 15);
        
        map.insert("fjorton", 14);
        
        map.insert("fyra", 4);
        
        map.insert("ingen", 0);
        
        map.insert("inget", 0);
        
        map.insert("nio", 9);
        
        map.insert("nitton", 19);
        
        map.insert("noll", 0);
        
        map.insert("sex", 6);
        
        map.insert("sexton", 16);
        
        map.insert("sju", 7);
        
        map.insert("sjutton", 17);
        
        map.insert("tio", 10);
        
        map.insert("tolv", 12);
        
        map.insert("tre", 3);
        
        map.insert("tretton", 13);
        
        map.insert("två", 2);
        
        map.insert("åtta", 8);
        
        map
    };
}

// Dictionary: dozen_dictionary
lazy_static! {
    static ref DOZEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("femtio", 50);
        
        map.insert("fyrtio", 40);
        
        map.insert("nittio", 90);
        
        map.insert("sextio", 60);
        
        map.insert("sjuttio", 70);
        
        map.insert("tjugo", 20);
        
        map.insert("trettio", 30);
        
        map.insert("åttio", 80);
        
        map
    };
}


/// Build Numeral rules for sv locale
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
    // Examples: inget, ingen, noll, en, ett
    {
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "sv:zeroToNineteen_dictionary",
            b.reg(r"(?i)arton|elva|en|ett|fem|femton|fjorton|fyra|ingen|inget|nio|nitton|noll|sex|sexton|sju|sjutton|tio|tolv|tre|tretton|två|åtta").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: dozen_dictionary
    // Examples: tjugo, trettio, fyrtio, femtio, sextio
    {
        let dict = &*DOZEN_DICTIONARY;
        b.rule_1_terminal(
            "sv:dozen_dictionary",
            b.reg(r"(?i)femtio|fyrtio|nittio|sextio|sjuttio|tjugo|trettio|åttio").unwrap(),
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
            "sv:Integer",
            b.reg(r"(?i)(inget|ingen|noll|en|ett|två|tretton|tre|fyra|femton|fem|sexton|sex|sjutton|sju|åtta|nio|tio|elva|tolv|fjorton|arton|nitton)").unwrap(),
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
            "sv:Integer2",
            b.reg(r"(?i)(tjugo|trettio|fyrtio|femtio|sextio|sjuttio|åttio|nittio)").unwrap(),
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
    

    // TODO: Integer3 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  sv/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_sv_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_sv_numeral_dictionaries() {
        
        assert!(ZEROTONINETEEN_DICTIONARY.len() > 0, "zeroToNineteen_dictionary should not be empty");
        
        assert!(DOZEN_DICTIONARY.len() > 0, "dozen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_sv_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 4;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("sv/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

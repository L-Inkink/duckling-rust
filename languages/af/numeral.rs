// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/AF/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: af
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: zeroAndTen_dictionary
lazy_static! {
    static ref ZEROANDTEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("geen", 0);
        
        map.insert("niks", 0);
        
        map.insert("nul", 0);
        
        map.insert("tien", 10);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: oneToNine_dictionary
lazy_static! {
    static ref ONETONINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ag", 8);
        
        map.insert("agt", 8);
        
        map.insert("drie", 3);
        
        map.insert("een", 1);
        
        map.insert("nege", 9);
        
        map.insert("ses", 6);
        
        map.insert("sewe", 7);
        
        map.insert("twee", 2);
        
        map.insert("vier", 4);
        
        map.insert("vyf", 5);
        
        map
    };
}

// Dictionary: elevenToNineteen_dictionary
lazy_static! {
    static ref ELEVENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("agtien", 18);
        
        map.insert("dertien", 13);
        
        map.insert("elf", 11);
        
        map.insert("neentien", 19);
        
        map.insert("negentien", 19);
        
        map.insert("sestien", 16);
        
        map.insert("sewentien", 17);
        
        map.insert("twaalf", 12);
        
        map.insert("veertien", 14);
        
        map.insert("vyftien", 15);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("der", 30);
        
        map.insert("neen", 90);
        
        map.insert("negen", 90);
        
        map.insert("ses", 60);
        
        map.insert("sewen", 70);
        
        map.insert("tag", 80);
        
        map.insert("twin", 20);
        
        map.insert("veer", 40);
        
        map.insert("vyf", 50);
        
        map
    };
}


/// Build Numeral rules for af locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 5 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroAndTen_dictionary
    // Examples: nul, geen, niks, zero, tien
    {
        let dict = &*ZEROANDTEN_DICTIONARY;
        b.rule_1_terminal(
            "af:zeroAndTen_dictionary",
            b.reg(r"(?i)geen|niks|nul|tien|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: oneToNine_dictionary
    // Examples: een, twee, drie, vier, vyf
    {
        let dict = &*ONETONINE_DICTIONARY;
        b.rule_1_terminal(
            "af:oneToNine_dictionary",
            b.reg(r"(?i)ag|agt|drie|een|nege|ses|sewe|twee|vier|vyf").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenToNineteen_dictionary
    // Examples: elf, twaalf, dertien, veertien, vyftien
    {
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "af:elevenToNineteen_dictionary",
            b.reg(r"(?i)agtien|dertien|elf|neentien|negentien|sestien|sewentien|twaalf|veertien|vyftien").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: twin, der, veer, vyf, ses
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "af:tens_dictionary",
            b.reg(r"(?i)der|neen|negen|ses|sewen|tag|twin|veer|vyf").unwrap(),
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
    

    // Rule: Numeral (refs: oneToNineMap)
    {
        
        
        let dict = &*ONETONINE_DICTIONARY;
        b.rule_1_terminal(
            "af:Numeral",
            b.reg(r"(?i)(nul|geen|niks|zero|tien|een|twee|drie|vier|vyf|ses|sewe|agt?|nege)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: ElevenToNineteen (refs: elevenToNineteenMap)
    {
        
        
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "af:ElevenToNineteen",
            b.reg(r"(?i)(elf|twaalf|dertien|veertien|vyftien|sestien|sewentien|agtien|neg?entien)").unwrap(),
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
            "af:Tens",
            b.reg(r"(?i)(twin|der|veer|vyf|ses|sewen|tag|neg?en)tig").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (5)
    // ========================================
    

    // TODO: Decimals (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: a dozen of
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  af/numeral has 5 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_af_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_af_numeral_dictionaries() {
        
        assert!(ZEROANDTEN_DICTIONARY.len() > 0, "zeroAndTen_dictionary should not be empty");
        
        assert!(ONETONINE_DICTIONARY.len() > 0, "oneToNine_dictionary should not be empty");
        
        assert!(ELEVENTONINETEEN_DICTIONARY.len() > 0, "elevenToNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_af_numeral_stats() {
        // Generation statistics
        let total_rules = 12;
        let auto_generated = 7;
        let manual_needed = 5;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("af/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

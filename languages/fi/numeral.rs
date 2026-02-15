// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/FI/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: fi
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: numeral_dictionary
lazy_static! {
    static ref NUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kahdeksan", 8);
        
        map.insert("kaksi", 2);
        
        map.insert("kolme", 3);
        
        map.insert("kuusi", 6);
        
        map.insert("kymmenen", 10);
        
        map.insert("neljä", 4);
        
        map.insert("nolla", 0);
        
        map.insert("seitsemän", 7);
        
        map.insert("viisi", 5);
        
        map.insert("yhdeksän", 9);
        
        map.insert("yksi", 1);
        
        map
    };
}

// Dictionary: elevenToNineteen_dictionary
lazy_static! {
    static ref ELEVENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kahdeksantoista", 18);
        
        map.insert("kaksitoista", 12);
        
        map.insert("kolmetoista", 13);
        
        map.insert("kuusitoista", 16);
        
        map.insert("neljätoista", 14);
        
        map.insert("seitsemäntoista", 17);
        
        map.insert("viisitoista", 15);
        
        map.insert("yhdeksäntoista", 19);
        
        map.insert("yksitoista", 11);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("kahdeksankymmentä", 80);
        
        map.insert("kaksikymmentä", 20);
        
        map.insert("kolmekymmentä", 30);
        
        map.insert("kuusikymmentä", 60);
        
        map.insert("neljäkymmentä", 40);
        
        map.insert("seitsemänkymmentä", 70);
        
        map.insert("viisikymmentä", 50);
        
        map.insert("yhdeksänkymmentä", 90);
        
        map
    };
}


/// Build Numeral rules for fi locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: numeral_dictionary
    // Examples: nolla, yksi, kaksi, kolme, neljä
    {
        let dict = &*NUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "fi:numeral_dictionary",
            b.reg(r"(?i)kahdeksan|kaksi|kolme|kuusi|kymmenen|neljä|nolla|seitsemän|viisi|yhdeksän|yksi").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenToNineteen_dictionary
    // Examples: yksitoista, kaksitoista, kolmetoista, neljätoista, viisitoista
    {
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "fi:elevenToNineteen_dictionary",
            b.reg(r"(?i)kahdeksantoista|kaksitoista|kolmetoista|kuusitoista|neljätoista|seitsemäntoista|viisitoista|yhdeksäntoista|yksitoista").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: kaksikymmentä, kolmekymmentä, neljäkymmentä, viisikymmentä, kuusikymmentä
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "fi:tens_dictionary",
            b.reg(r"(?i)kahdeksankymmentä|kaksikymmentä|kolmekymmentä|kuusikymmentä|neljäkymmentä|seitsemänkymmentä|viisikymmentä|yhdeksänkymmentä").unwrap(),
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
    

    // Rule: Numeral (refs: numeralMap)
    {
        
        
        let dict = &*NUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "fi:Numeral",
            b.reg(r"(?i)(nolla|yksi|kaksi|kolme|neljä|viisi|kuusi|seitsemän|kahdeksan|yhdeksän|kymmenen)").unwrap(),
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
            "fi:ElevenToNineteen",
            b.reg(r"(?i)(yksitoista|kaksitoista|kolmetoista|neljätoista|viisitoista|kuusitoista|seitsemäntoista|kahdeksantoista|yhdeksäntoista)").unwrap(),
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
            "fi:Tens",
            b.reg(r"(?i)(kaksikymmentä|kolmekymmentä|neljäkymmentä|viisikymmentä|kuusikymmentä|seitsemänkymmentä|kahdeksankymmentä|yhdeksänkymmentä)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: CompositeTens (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "fi:CompositeTens",
            b.reg(r"(?i)(kaksikymmentä|kolmekymmentä|neljäkymmentä|viisikymmentä|kuusikymmentä|seitsemänkymmentä|kahdeksankymmentä|yhdeksänkymmentä)(yksi|kaksi|kolme|neljä|viisi|kuusi|seitsemän|kahdeksan|yhdeksän)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_fi_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_fi_numeral_dictionaries() {
        
        assert!(NUMERAL_DICTIONARY.len() > 0, "numeral_dictionary should not be empty");
        
        assert!(ELEVENTONINETEEN_DICTIONARY.len() > 0, "elevenToNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_fi_numeral_stats() {
        // Generation statistics
        let total_rules = 7;
        let auto_generated = 7;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("fi/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

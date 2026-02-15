// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/BG/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: bg
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("два", 2);
        
        map.insert("дванадесет", 12);
        
        map.insert("дванайсет", 12);
        
        map.insert("две", 2);
        
        map.insert("девет", 9);
        
        map.insert("деветнадесет", 19);
        
        map.insert("деветнайсет", 19);
        
        map.insert("десет", 10);
        
        map.insert("един", 1);
        
        map.insert("единадесет", 11);
        
        map.insert("единайсет", 11);
        
        map.insert("една", 1);
        
        map.insert("едно", 1);
        
        map.insert("нула", 0);
        
        map.insert("осем", 8);
        
        map.insert("осемнадесет", 18);
        
        map.insert("осемнайсет", 18);
        
        map.insert("пет", 5);
        
        map.insert("петнадесет", 15);
        
        map.insert("петнайсет", 15);
        
        map.insert("седем", 7);
        
        map.insert("седемнадесет", 17);
        
        map.insert("седемнайсет", 17);
        
        map.insert("три", 3);
        
        map.insert("тринадесет", 13);
        
        map.insert("тринайсет", 13);
        
        map.insert("четири", 4);
        
        map.insert("четиринадесет", 14);
        
        map.insert("четиринайсет", 14);
        
        map.insert("шест", 6);
        
        map.insert("шестнадесет", 16);
        
        map.insert("шестнайсет", 16);
        
        map
    };
}


/// Build Numeral rules for bg locale
///
/// Auto-generated rules:
///   - 1 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 7 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (1)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: нула, един, една, едно, два
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "bg:zeroNineteen_dictionary",
            b.reg(r"(?i)два|дванадесет|дванайсет|две|девет|деветнадесет|деветнайсет|десет|един|единадесет|единайсет|една|едно|нула|осем|осемнадесет|осемнайсет|пет|петнадесет|петнайсет|седем|седемнадесет|седемнайсет|три|тринадесет|тринайсет|четири|четиринадесет|четиринайсет|шест|шестнадесет|шестнайсет").unwrap(),
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
            "bg:ToNineteen",
            b.reg(r"(?i)(нула|едина(де|й)сет|двана(де|й)сет|трина(де|й)сет|четирина(де|й)сет|петна(де|й)сет|шестна(де|й)сет|седемна(де|й)сет|осемна(де|й)сет|деветна(де|й)сет|един|една|едно|два|две|три|четири|пет|шест|седем|осем|девет|десет)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Tens (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "bg:Tens",
            b.reg(r"(?i)((два|три|четири|пет|шест|седем|осем|девет)десет)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (7)
    // ========================================
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Hundreds (regex)
    
    //   Original: integer (100..900)
    
    // Manual implementation required
    

    // TODO: Decimals (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Commas (regex)
    
    //   Original: comma-separated numbers
    
    // Manual implementation required
    

    // TODO: Negative (regex)
    
    //   Original: negative numbers
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: CompositeHundreds (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  bg/numeral has 7 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_bg_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_bg_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_bg_numeral_stats() {
        // Generation statistics
        let total_rules = 10;
        let auto_generated = 3;
        let manual_needed = 7;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("bg/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

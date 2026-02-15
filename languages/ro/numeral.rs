// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/RO/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ro
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroTen_dictionary
lazy_static! {
    static ref ZEROTEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("\537apte", 7);
        
        map.insert("\537ase", 6);
        
        map.insert("cinci", 5);
        
        map.insert("doi", 2);
        
        map.insert("doua", 2);
        
        map.insert("două", 2);
        
        map.insert("intai", 1);
        
        map.insert("intâi", 1);
        
        map.insert("nici o", 0);
        
        map.insert("nici una", 0);
        
        map.insert("nici unu", 0);
        
        map.insert("nici unul", 0);
        
        map.insert("nicio", 0);
        
        map.insert("nimic", 0);
        
        map.insert("noua", 9);
        
        map.insert("nouă", 9);
        
        map.insert("o", 1);
        
        map.insert("opt", 8);
        
        map.insert("patru", 4);
        
        map.insert("sapte", 7);
        
        map.insert("sase", 6);
        
        map.insert("trei", 3);
        
        map.insert("un", 1);
        
        map.insert("una", 1);
        
        map.insert("unu", 1);
        
        map.insert("unul", 1);
        
        map.insert("zece", 10);
        
        map.insert("zeci", 10);
        
        map.insert("zero", 0);
        
        map.insert("întai", 1);
        
        map.insert("întâi", 1);
        
        map
    };
}

// Dictionary: elevenNineteen_dictionary
lazy_static! {
    static ref ELEVENNINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("\537ai", 16);
        
        map.insert("\537apte", 17);
        
        map.insert("\537apti", 17);
        
        map.insert("cin", 15);
        
        map.insert("cinci", 15);
        
        map.insert("doi", 12);
        
        map.insert("noua", 19);
        
        map.insert("nouă", 19);
        
        map.insert("opt", 18);
        
        map.insert("opti", 18);
        
        map.insert("pai", 14);
        
        map.insert("sai", 16);
        
        map.insert("sapte", 17);
        
        map.insert("sapti", 17);
        
        map.insert("trei", 13);
        
        map.insert("un", 11);
        
        map
    };
}


/// Build Numeral rules for ro locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 7 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: zeroTen_dictionary
    // Examples: zero, nimic, nicio, nici o, nici una
    {
        let dict = &*ZEROTEN_DICTIONARY;
        b.rule_1_terminal(
            "ro:zeroTen_dictionary",
            b.reg(r"(?i)\537apte|\537ase|cinci|doi|doua|două|intai|intâi|nici o|nici una|nici unu|nici unul|nicio|nimic|noua|nouă|o|opt|patru|sapte|sase|trei|un|una|unu|unul|zece|zeci|zero|întai|întâi").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenNineteen_dictionary
    // Examples: un, doi, trei, pai, cin
    {
        let dict = &*ELEVENNINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ro:elevenNineteen_dictionary",
            b.reg(r"(?i)\537ai|\537apte|\537apti|cin|cinci|doi|noua|nouă|opt|opti|pai|sai|sapte|sapti|trei|un").unwrap(),
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
    

    // Rule: IntegerZeroTen (refs: zeroTenMap)
    {
        
        
        let dict = &*ZEROTEN_DICTIONARY;
        b.rule_1_terminal(
            "ro:IntegerZeroTen",
            b.reg(r"(?i)(zero|nimic|nici(\\s?o|\\sun(a|ul?))|una|unul?|doi|dou(a|ă)|trei|patru|cinci|(s|ș)ase|(s|ș)apte|opt|nou(a|ă)|zec[ei]|(i|î)nt(a|â)i|un|o)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer (refs: elevenNineteenMap)
    {
        
        
        let dict = &*ELEVENNINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ro:Integer",
            b.reg(r"(?i)(cin|sapti|opti)(s|ș)pe|(cinci|(s|ș)apte|opt)sprezece|(un|doi|trei|pai|(s|ș)ai|nou(a|ă))((s|ș)pe|sprezece)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: zeroTenMap)
    {
        
        
        let dict = &*ZEROTEN_DICTIONARY;
        b.rule_1_terminal(
            "ro:Integer2",
            b.reg(r"(?i)(dou[aă]|trei|patru|cinci|[sș]ai|[sș]apte|opt|nou[aă])\\s?zeci").unwrap(),
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
    

    // TODO: NumeralsPrefixWithOrMinus (regex)
    
    //   Original: numbers prefix with - or minus
    
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
    

    // TODO: Integer3 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ro/numeral has 7 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ro_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ro_numeral_dictionaries() {
        
        assert!(ZEROTEN_DICTIONARY.len() > 0, "zeroTen_dictionary should not be empty");
        
        assert!(ELEVENNINETEEN_DICTIONARY.len() > 0, "elevenNineteen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ro_numeral_stats() {
        // Generation statistics
        let total_rules = 12;
        let auto_generated = 5;
        let manual_needed = 7;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ro/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

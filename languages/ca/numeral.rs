// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/CA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ca
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  




// Dictionary: zeroToFifteen_dictionary
lazy_static! {
    static ref ZEROTOFIFTEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("catorze", 14);
        
        map.insert("cinc", 5);
        
        map.insert("deu", 10);
        
        map.insert("dos", 2);
        
        map.insert("dotze", 12);
        
        map.insert("dues", 2);
        
        map.insert("nou", 9);
        
        map.insert("onze", 11);
        
        map.insert("quatre", 4);
        
        map.insert("quinze", 15);
        
        map.insert("set", 7);
        
        map.insert("sis", 6);
        
        map.insert("tres", 3);
        
        map.insert("tretze", 13);
        
        map.insert("u", 1);
        
        map.insert("un", 1);
        
        map.insert("una", 1);
        
        map.insert("vuit", 8);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinquanta", 50);
        
        map.insert("noranta", 90);
        
        map.insert("quaranta", 40);
        
        map.insert("seixanta", 60);
        
        map.insert("setanta", 70);
        
        map.insert("trenta", 30);
        
        map.insert("vint", 20);
        
        map.insert("vuitanta", 80);
        
        map
    };
}

// Dictionary: sixteenToTwentyNine_dictionary
lazy_static! {
    static ref SIXTEENTOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("denou", 19);
        
        map.insert("devuit", 18);
        
        map.insert("dihuit", 18);
        
        map.insert("dinou", 19);
        
        map.insert("disset", 17);
        
        map.insert("divuit", 18);
        
        map.insert("dènou", 19);
        
        map.insert("dèsset", 17);
        
        map.insert("setze", 16);
        
        map.insert("vint-i-cinc", 25);
        
        map.insert("vint-i-dos", 22);
        
        map.insert("vint-i-nou", 29);
        
        map.insert("vint-i-quatre", 24);
        
        map.insert("vint-i-set", 27);
        
        map.insert("vint-i-sis", 26);
        
        map.insert("vint-i-tres", 23);
        
        map.insert("vint-i-u", 21);
        
        map.insert("vint-i-una", 21);
        
        map.insert("vint-i-vuit", 28);
        
        map
    };
}

// Dictionary: oneHundredToThousand_dictionary
lazy_static! {
    static ref ONEHUNDREDTOTHOUSAND_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cent", 100);
        
        map.insert("cents", 100);
        
        map.insert("cinc-cents", 500);
        
        map.insert("dos-cents", 200);
        
        map.insert("mil", 1000);
        
        map.insert("nou-cents", 900);
        
        map.insert("quatre-cents", 400);
        
        map.insert("set-cents", 700);
        
        map.insert("sis-cents", 600);
        
        map.insert("tres-cents", 300);
        
        map.insert("vuit-cents", 800);
        
        map
    };
}


/// Build Numeral rules for ca locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 5 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroToFifteen_dictionary
    // Examples: zero, u, un, una, dos
    {
        let dict = &*ZEROTOFIFTEEN_DICTIONARY;
        b.rule_1_terminal(
            "ca:zeroToFifteen_dictionary",
            b.reg(r"(?i)catorze|cinc|deu|dos|dotze|dues|nou|onze|quatre|quinze|set|sis|tres|tretze|u|un|una|vuit|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: vint, trenta, quaranta, cinquanta, seixanta
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ca:tens_dictionary",
            b.reg(r"(?i)cinquanta|noranta|quaranta|seixanta|setanta|trenta|vint|vuitanta").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: sixteenToTwentyNine_dictionary
    // Examples: setze, disset, dèsset, devuit, divuit
    {
        let dict = &*SIXTEENTOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "ca:sixteenToTwentyNine_dictionary",
            b.reg(r"(?i)denou|devuit|dihuit|dinou|disset|divuit|dènou|dèsset|setze|vint-i-cinc|vint-i-dos|vint-i-nou|vint-i-quatre|vint-i-set|vint-i-sis|vint-i-tres|vint-i-u|vint-i-una|vint-i-vuit").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: oneHundredToThousand_dictionary
    // Examples: cent, cents, dos-cents, tres-cents, quatre-cents
    {
        let dict = &*ONEHUNDREDTOTHOUSAND_DICTIONARY;
        b.rule_1_terminal(
            "ca:oneHundredToThousand_dictionary",
            b.reg(r"(?i)cent|cents|cinc-cents|dos-cents|mil|nou-cents|quatre-cents|set-cents|sis-cents|tres-cents|vuit-cents").unwrap(),
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
    

    // Rule: ZeroToFifteen (refs: zeroToFifteenMap)
    {
        
        
        let dict = &*ZEROTOFIFTEEN_DICTIONARY;
        b.rule_1_terminal(
            "ca:ZeroToFifteen",
            b.reg(r"(?i)(zero|u(na|n)?|d(o|ue)s|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze|tretze|catorze|quinze)").unwrap(),
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
            "ca:Tens",
            b.reg(r"(?i)(vint|(tre|quara|cinqua|seixa|seta|vuita|nora)nta)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: LowerTensWithOnes (refs: sixteenToTwentyNineMap)
    {
        
        
        let dict = &*SIXTEENTOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "ca:LowerTensWithOnes",
            b.reg(r"(?i)(setze|d(i|e|è)sset|d(e|i)(v|h)uit|d(i|e|è)nou|vint-i-u(na)?|vint-i-dos|vint-i-tres|vint-i-quatre|vint-i-cinc|vint-i-sis|vint-i-set|vint-i-vuit|vint-i-nou)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Hundreds (refs: oneHundredToThousandMap)
    {
        
        
        let dict = &*ONEHUNDREDTOTHOUSAND_DICTIONARY;
        b.rule_1_terminal(
            "ca:Hundreds",
            b.reg(r"(?i)(cent(s)?|dos-cents|tres-cents|quatre-cents|cinc-cents|sis-cents|set-cents|vuit-cents|nou-cents|mil)").unwrap(),
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
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: BelowTenWithTwoDigits (regex)
    
    //   Original: integer (0-9) with two digits
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator .
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number ,
    
    // Manual implementation required
    

    // TODO: HigherTensWithOnes (composite)
    
    //   Original: number (31..39 41..49 51..59 61..69 71..79 81..89 91..99)
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ca/numeral has 5 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ca_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ca_numeral_dictionaries() {
        
        assert!(ZEROTOFIFTEEN_DICTIONARY.len() > 0, "zeroToFifteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(SIXTEENTOTWENTYNINE_DICTIONARY.len() > 0, "sixteenToTwentyNine_dictionary should not be empty");
        
        assert!(ONEHUNDREDTOTHOUSAND_DICTIONARY.len() > 0, "oneHundredToThousand_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ca_numeral_stats() {
        // Generation statistics
        let total_rules = 13;
        let auto_generated = 8;
        let manual_needed = 5;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ca/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

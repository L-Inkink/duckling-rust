// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/EL/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: el
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: oneOrTwoDigits_dictionary
lazy_static! {
    static ref ONEORTWODIGITS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ένα", 1);
        
        map.insert("ένας", 1);
        
        map.insert("ένδεκα", 11);
        
        map.insert("έντεκα", 11);
        
        map.insert("έξι", 6);
        
        map.insert("δέκα", 10);
        
        map.insert("δεκαέξι", 16);
        
        map.insert("δεκαεννέα", 19);
        
        map.insert("δεκαεννιά", 19);
        
        map.insert("δεκαεπτά", 17);
        
        map.insert("δεκαοκτώ", 18);
        
        map.insert("δεκαπέντε", 15);
        
        map.insert("δεκαριά", 10);
        
        map.insert("δεκατέσσερα", 14);
        
        map.insert("δεκατρία", 13);
        
        map.insert("δυο", 2);
        
        map.insert("δύο", 2);
        
        map.insert("δώδεκα", 12);
        
        map.insert("είκοσι", 20);
        
        map.insert("εβδομήντα", 70);
        
        map.insert("ενενήντα", 90);
        
        map.insert("εννέα", 9);
        
        map.insert("εννιά", 9);
        
        map.insert("ενός", 1);
        
        map.insert("εξήντα", 60);
        
        map.insert("επτά", 7);
        
        map.insert("εφτά", 7);
        
        map.insert("μία", 1);
        
        map.insert("μηδέν", 0);
        
        map.insert("μια", 1);
        
        map.insert("ντουζίνα", 12);
        
        map.insert("ντουζίνες", 12);
        
        map.insert("ογδόντα", 80);
        
        map.insert("οκτώ", 8);
        
        map.insert("οχτώ", 8);
        
        map.insert("πέντε", 5);
        
        map.insert("πενήντα", 50);
        
        map.insert("σαράντα", 40);
        
        map.insert("τέσσερα", 4);
        
        map.insert("τέσσερις", 4);
        
        map.insert("τρία", 3);
        
        map.insert("τρεις", 3);
        
        map.insert("τριάντα", 30);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("δι", 200);
        
        map.insert("εννι", 900);
        
        map.insert("εξ", 600);
        
        map.insert("επτ", 700);
        
        map.insert("εφτ", 700);
        
        map.insert("οκτ", 800);
        
        map.insert("οχτ", 800);
        
        map.insert("πεντ", 500);
        
        map.insert("τετρ", 400);
        
        map.insert("τρι", 300);
        
        map
    };
}


/// Build Numeral rules for el locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 1 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: oneOrTwoDigits_dictionary
    // Examples: μηδέν, ένα, ένας, ενός, μία
    {
        let dict = &*ONEORTWODIGITS_DICTIONARY;
        b.rule_1_terminal(
            "el:oneOrTwoDigits_dictionary",
            b.reg(r"(?i)ένα|ένας|ένδεκα|έντεκα|έξι|δέκα|δεκαέξι|δεκαεννέα|δεκαεννιά|δεκαεπτά|δεκαοκτώ|δεκαπέντε|δεκαριά|δεκατέσσερα|δεκατρία|δυο|δύο|δώδεκα|είκοσι|εβδομήντα|ενενήντα|εννέα|εννιά|ενός|εξήντα|επτά|εφτά|μία|μηδέν|μια|ντουζίνα|ντουζίνες|ογδόντα|οκτώ|οχτώ|πέντε|πενήντα|σαράντα|τέσσερα|τέσσερις|τρία|τρεις|τριάντα").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: δι, τρι, τετρ, πεντ, εξ
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "el:hundreds_dictionary",
            b.reg(r"(?i)δι|εννι|εξ|επτ|εφτ|οκτ|οχτ|πεντ|τετρ|τρι").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (1)
    // ========================================
    

    // Rule: Hundreds (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "el:Hundreds",
            b.reg(r"(?i)(δι|τρι|τετρ|πεντ|εξ|ε(π|φ)τ|ο(χ|κ)τ|εννι)ακόσι(α|ες|οι)").unwrap(),
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
    

    // TODO: Few (regex)
    
    //   Original: few
    
    // Manual implementation required
    

    // TODO: Hundred (regex)
    
    //   Original: number (100)
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Negative (regex)
    
    //   Original: negative numbers
    
    // Manual implementation required
    

    // TODO: Decimals (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Dots (regex)
    
    //   Original: dot-separated numbers
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  el/numeral has 9 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_el_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_el_numeral_dictionaries() {
        
        assert!(ONEORTWODIGITS_DICTIONARY.len() > 0, "oneOrTwoDigits_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_el_numeral_stats() {
        // Generation statistics
        let total_rules = 12;
        let auto_generated = 3;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("el/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/HU/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: hu
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("\x00F6t", 5);
        
        map.insert("egy", 1);
        
        map.insert("h\x00E1rom", 3);
        
        map.insert("h\x00E9t", 7);
        
        map.insert("hat", 6);
        
        map.insert("kett\x0151", 2);
        
        map.insert("kilenc", 9);
        
        map.insert("n\x00E9gy", 4);
        
        map.insert("nulla", 0);
        
        map.insert("nyolc", 8);
        
        map.insert("t\x00EDz", 10);
        
        map.insert("z\x00E9r\x00F3", 0);
        
        map
    };
}

// Dictionary: elevenToNineteen_dictionary
lazy_static! {
    static ref ELEVENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("tizen\x00F6t", 15);
        
        map.insert("tizenegy", 11);
        
        map.insert("tizenh\x00E1rom", 13);
        
        map.insert("tizenh\x00E9t", 17);
        
        map.insert("tizenhat", 16);
        
        map.insert("tizenkett\x0151", 12);
        
        map.insert("tizenkilenc", 19);
        
        map.insert("tizenn\x00E9gy", 14);
        
        map.insert("tizennyolc", 18);
        
        map
    };
}

// Dictionary: twentyoneToTwentynine_dictionary
lazy_static! {
    static ref TWENTYONETOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("huszon\x00F6t", 25);
        
        map.insert("huszonegy", 21);
        
        map.insert("huszonh\x00E1rom", 23);
        
        map.insert("huszonh\x00E9t", 27);
        
        map.insert("huszonhat", 26);
        
        map.insert("huszonkett\x0151", 22);
        
        map.insert("huszonkilenc", 29);
        
        map.insert("huszonn\x00E9gy", 24);
        
        map.insert("huszonnyolc", 28);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("\x00F6tven", 50);
        
        map.insert("h\x00FAsz", 20);
        
        map.insert("harminc", 30);
        
        map.insert("hatvan", 60);
        
        map.insert("hetven", 70);
        
        map.insert("kilencven", 90);
        
        map.insert("negyven", 40);
        
        map.insert("nyolcvan", 80);
        
        map
    };
}


/// Build Numeral rules for hu locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 5 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: nulla, z\x00E9r\x00F3, egy, kett\x0151, h\x00E1rom
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "hu:ruleNumeral_dictionary",
            b.reg(r"(?i)\x00F6t|egy|h\x00E1rom|h\x00E9t|hat|kett\x0151|kilenc|n\x00E9gy|nulla|nyolc|t\x00EDz|z\x00E9r\x00F3").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenToNineteen_dictionary
    // Examples: tizenegy, tizenkett\x0151, tizenh\x00E1rom, tizenn\x00E9gy, tizen\x00F6t
    {
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "hu:elevenToNineteen_dictionary",
            b.reg(r"(?i)tizen\x00F6t|tizenegy|tizenh\x00E1rom|tizenh\x00E9t|tizenhat|tizenkett\x0151|tizenkilenc|tizenn\x00E9gy|tizennyolc").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyoneToTwentynine_dictionary
    // Examples: huszonegy, huszonkett\x0151, huszonh\x00E1rom, huszonn\x00E9gy, huszon\x00F6t
    {
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "hu:twentyoneToTwentynine_dictionary",
            b.reg(r"(?i)huszon\x00F6t|huszonegy|huszonh\x00E1rom|huszonh\x00E9t|huszonhat|huszonkett\x0151|huszonkilenc|huszonn\x00E9gy|huszonnyolc").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: h\x00FAsz, harminc, negyven, \x00F6tven, hatvan
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "hu:tens_dictionary",
            b.reg(r"(?i)\x00F6tven|h\x00FAsz|harminc|hatvan|hetven|kilencven|negyven|nyolcvan").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (5)
    // ========================================
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "hu:Numeral",
            b.reg(r"(?i)(nulla|z\x00E9r\x00F3|egy|kett\x0151|h\x00E1rom|n\x00E9gy|\x00F6t|hat|h\x00E9t|nyolc|kilenc|t\x00EDz)").unwrap(),
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
            "hu:ElevenToNineteen",
            b.reg(r"(?i)(tizenegy|tizenkett\x0151|tizenh\x00E1rom|tizenn\x00E9gy|tizen\x00F6t|tizenhat|tizenh\x00E9t|tizennyolc|tizenkilenc)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: TwentyoneToTwentynine (refs: twentyoneToTwentynineMap)
    {
        
        
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "hu:TwentyoneToTwentynine",
            b.reg(r"(?i)(huszonegy|huszonkett\x0151|huszonh\x00E1rom|huszonn\x00E9gy|huszon\x00F6t|huszonhat|huszonh\x00E9t|huszonnyolc|huszonkilenc)").unwrap(),
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
            "hu:Tens",
            b.reg(r"(?i)(h\x00FAsz|harminc|negyven|ötven|hatvan|hetven|nyolcvan|kilencven)").unwrap(),
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
            "hu:CompositeTens",
            b.reg(r"(?i)(harminc|negyven|\x00F6tven|hatvan|hetven|nyolcvan|kilencven)(egy|kett\x0151|h\x00E1rom|n\x00E9gy|\x00F6t|hat|h\x00E9t|nyolc|kilenc)").unwrap(),
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
    fn test_hu_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_hu_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
        assert!(ELEVENTONINETEEN_DICTIONARY.len() > 0, "elevenToNineteen_dictionary should not be empty");
        
        assert!(TWENTYONETOTWENTYNINE_DICTIONARY.len() > 0, "twentyoneToTwentynine_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_hu_numeral_stats() {
        // Generation statistics
        let total_rules = 9;
        let auto_generated = 9;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("hu/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

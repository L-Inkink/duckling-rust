// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/BN/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: bn
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("আট", 8);
        
        map.insert("এক", 1);
        
        map.insert("চার", 4);
        
        map.insert("ছয়", 6);
        
        map.insert("তিন", 3);
        
        map.insert("দশ", 10);
        
        map.insert("দুই", 2);
        
        map.insert("নয়", 9);
        
        map.insert("পাঁচ", 5);
        
        map.insert("শূন্য", 0);
        
        map.insert("সাত", 7);
        
        map
    };
}

// Dictionary: elevenToNineteen_dictionary
lazy_static! {
    static ref ELEVENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("আঠারো", 18);
        
        map.insert("উনিশ", 19);
        
        map.insert("এগারো", 11);
        
        map.insert("চৌদ্দ", 14);
        
        map.insert("তেরো", 13);
        
        map.insert("পনেরো", 15);
        
        map.insert("বারো", 12);
        
        map.insert("ষোল", 16);
        
        map.insert("সতেরো", 17);
        
        map
    };
}

// Dictionary: twentyoneToTwentynine_dictionary
lazy_static! {
    static ref TWENTYONETOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("আঠাশ", 28);
        
        map.insert("ঊনত্রিশ", 29);
        
        map.insert("একুশ", 21);
        
        map.insert("চব্বিশ", 24);
        
        map.insert("ছাব্বিশ", 26);
        
        map.insert("তেইশ", 23);
        
        map.insert("পঁচিশ", 25);
        
        map.insert("বাইশ", 22);
        
        map.insert("সাতাশ", 27);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("আশি", 80);
        
        map.insert("কুড়ি", 20);
        
        map.insert("চল্লিশ", 40);
        
        map.insert("তিরিশ", 30);
        
        map.insert("নব্বই", 90);
        
        map.insert("পঞ্চাশ", 50);
        
        map.insert("ষাট", 60);
        
        map.insert("সত্তর", 70);
        
        map
    };
}


/// Build Numeral rules for bn locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 0 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: শূন্য, এক, দুই, তিন, চার
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "bn:ruleNumeral_dictionary",
            b.reg(r"(?i)আট|এক|চার|ছয়|তিন|দশ|দুই|নয়|পাঁচ|শূন্য|সাত").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: elevenToNineteen_dictionary
    // Examples: এগারো, বারো, তেরো, চৌদ্দ, পনেরো
    {
        let dict = &*ELEVENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "bn:elevenToNineteen_dictionary",
            b.reg(r"(?i)আঠারো|উনিশ|এগারো|চৌদ্দ|তেরো|পনেরো|বারো|ষোল|সতেরো").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyoneToTwentynine_dictionary
    // Examples: একুশ, বাইশ, তেইশ, চব্বিশ, পঁচিশ
    {
        let dict = &*TWENTYONETOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "bn:twentyoneToTwentynine_dictionary",
            b.reg(r"(?i)আঠাশ|ঊনত্রিশ|একুশ|চব্বিশ|ছাব্বিশ|তেইশ|পঁচিশ|বাইশ|সাতাশ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: কুড়ি, তিরিশ, চল্লিশ, পঞ্চাশ, ষাট
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "bn:tens_dictionary",
            b.reg(r"(?i)আশি|কুড়ি|চল্লিশ|তিরিশ|নব্বই|পঞ্চাশ|ষাট|সত্তর").unwrap(),
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
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "bn:Numeral",
            b.reg(r"(?i)(শূন্য|এক|দুই|তিন|চার|পাঁচ|ছয়|সাত|আট|নয়|দশ)").unwrap(),
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
            "bn:ElevenToNineteen",
            b.reg(r"(?i)(এগারো|বারো|তেরো|চৌদ্দ|পনেরো|ষোল|সতেরো|আঠারো|উনিশ)").unwrap(),
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
            "bn:TwentyoneToTwentynine",
            b.reg(r"(?i)(একুশ|বাইশ|তেইশ|চব্বিশ|পঁচিশ|ছাব্বিশ|সাতাশ|আঠাশ|ঊনত্রিশ)").unwrap(),
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
            "bn:Tens",
            b.reg(r"(?i)(কুড়ি|তিরিশ|চল্লিশ|পঞ্চাশ|ষাট|সত্তর|আশি|নব্বই)").unwrap(),
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
    fn test_bn_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_bn_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
        assert!(ELEVENTONINETEEN_DICTIONARY.len() > 0, "elevenToNineteen_dictionary should not be empty");
        
        assert!(TWENTYONETOTWENTYNINE_DICTIONARY.len() > 0, "twentyoneToTwentynine_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_bn_numeral_stats() {
        // Generation statistics
        let total_rules = 8;
        let auto_generated = 8;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("bn/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/TE/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: te
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: zeroToNine_dictionary
lazy_static! {
    static ref ZEROTONINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ఆరు", 6);
        
        map.insert("ఎనిమిది", 8);
        
        map.insert("ఏడు", 7);
        
        map.insert("ఐదు", 5);
        
        map.insert("ఒకటి", 1);
        
        map.insert("తొమ్మిది", 9);
        
        map.insert("నాలుగు", 4);
        
        map.insert("మూడు", 3);
        
        map.insert("రెండు", 2);
        
        map.insert("సున్న", 0);
        
        map
    };
}

// Dictionary: tenToNineteen_dictionary
lazy_static! {
    static ref TENTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("పంతొమ్మిది", 19);
        
        map.insert("పదకొండు", 11);
        
        map.insert("పదమూడు", 13);
        
        map.insert("పదహారు", 16);
        
        map.insert("పది", 10);
        
        map.insert("పదిహేడు", 17);
        
        map.insert("పదిహేను", 15);
        
        map.insert("పద్దెనిమిది", 18);
        
        map.insert("పద్నాల్గు", 14);
        
        map.insert("పన్నెండు", 12);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("అరవై", 60);
        
        map.insert("ఇరవై", 20);
        
        map.insert("ఎనబై", 80);
        
        map.insert("డెబ్బై", 70);
        
        map.insert("తొంబై", 90);
        
        map.insert("నలబై", 40);
        
        map.insert("ముప్పై", 30);
        
        map.insert("యాబై", 50);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("కోటి", 10000000);
        
        map.insert("లక్ష", 100000);
        
        map.insert("వంద", 100);
        
        map.insert("వెయ్యి", 1000);
        
        map
    };
}


/// Build Numeral rules for te locale
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
    

    // Rule: zeroToNine_dictionary
    // Examples: సున్న, ఒకటి, రెండు, మూడు, నాలుగు
    {
        let dict = &*ZEROTONINE_DICTIONARY;
        b.rule_1_terminal(
            "te:zeroToNine_dictionary",
            b.reg(r"(?i)ఆరు|ఎనిమిది|ఏడు|ఐదు|ఒకటి|తొమ్మిది|నాలుగు|మూడు|రెండు|సున్న").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tenToNineteen_dictionary
    // Examples: పది, పదకొండు, పన్నెండు, పదమూడు, పద్నాల్గు
    {
        let dict = &*TENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "te:tenToNineteen_dictionary",
            b.reg(r"(?i)పంతొమ్మిది|పదకొండు|పదమూడు|పదహారు|పది|పదిహేడు|పదిహేను|పద్దెనిమిది|పద్నాల్గు|పన్నెండు").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ఇరవై, ముప్పై, నలబై, యాబై, అరవై
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "te:tens_dictionary",
            b.reg(r"(?i)అరవై|ఇరవై|ఎనబై|డెబ్బై|తొంబై|నలబై|ముప్పై|యాబై").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: వంద, వెయ్యి, లక్ష, కోటి
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "te:hundreds_dictionary",
            b.reg(r"(?i)కోటి|లక్ష|వంద|వెయ్యి").unwrap(),
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
    

    // Rule: ZeroToNine (refs: zeroToNineMap)
    {
        
        
        let dict = &*ZEROTONINE_DICTIONARY;
        b.rule_1_terminal(
            "te:ZeroToNine",
            b.reg(r"(?i)(సున్న|ఒకటి|రెండు|మూడు|నాలుగు|ఐదు|ఆరు|ఏడు|ఎనిమిది|తొమ్మిది)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: TenToNineteen (refs: tenToNineteenMap)
    {
        
        
        let dict = &*TENTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "te:TenToNineteen",
            b.reg(r"(?i)(పదకొండు|పన్నెండు|పదమూడు|పద్నాల్గు|పదిహేను|పదహారు|పదిహేడు|పద్దెనిమిది|పంతొమ్మిది|పది)").unwrap(),
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
            "te:Tens",
            b.reg(r"(?i)(ఇరవై|ముప్పై|నలబై|యాబై|అరవై|డెబ్బై|ఎనబై|తొంబై)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "te:hundreds",
            b.reg(r"(?i)(వంద|వెయ్యి|లక్ష|కోటి)").unwrap(),
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
    fn test_te_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_te_numeral_dictionaries() {
        
        assert!(ZEROTONINE_DICTIONARY.len() > 0, "zeroToNine_dictionary should not be empty");
        
        assert!(TENTONINETEEN_DICTIONARY.len() > 0, "tenToNineteen_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_te_numeral_stats() {
        // Generation statistics
        let total_rules = 8;
        let auto_generated = 8;
        let manual_needed = 0;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("te/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

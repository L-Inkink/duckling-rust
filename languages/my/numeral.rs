// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/MY/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: my
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: integer09_dictionary
lazy_static! {
    static ref INTEGER09_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("၀", 0);
        
        map.insert("၁", 1);
        
        map.insert("၂", 2);
        
        map.insert("၃", 3);
        
        map.insert("၄", 4);
        
        map.insert("၅", 5);
        
        map.insert("၆", 6);
        
        map.insert("၇", 7);
        
        map.insert("၈", 8);
        
        map.insert("၉", 9);
        
        map
    };
}

// Dictionary: integerPali_dictionary
lazy_static! {
    static ref INTEGERPALI_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("တတိယ", 3);
        
        map.insert("ဒုတိယ", 2);
        
        map.insert("ပထမ", 1);
        
        map
    };
}

// Dictionary: integer2_dictionary
lazy_static! {
    static ref INTEGER2_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ကိုး", 9);
        
        map.insert("ခုနှစ်", 7);
        
        map.insert("ခြေါက်", 6);
        
        map.insert("ငါး", 5);
        
        map.insert("တစ်", 1);
        
        map.insert("တစ်ဆယ်", 10);
        
        map.insert("နှစ်", 2);
        
        map.insert("ရှစ်", 8);
        
        map.insert("လေး", 4);
        
        map.insert("သုံး", 3);
        
        map
    };
}


/// Build Numeral rules for my locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 2 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: integer09_dictionary
    // Examples: ၀, ၁, ၂, ၃, ၄
    {
        let dict = &*INTEGER09_DICTIONARY;
        b.rule_1_terminal(
            "my:integer09_dictionary",
            b.reg(r"(?i)၀|၁|၂|၃|၄|၅|၆|၇|၈|၉").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integerPali_dictionary
    // Examples: ပထမ, ဒုတိယ, တတိယ
    {
        let dict = &*INTEGERPALI_DICTIONARY;
        b.rule_1_terminal(
            "my:integerPali_dictionary",
            b.reg(r"(?i)တတိယ|ဒုတိယ|ပထမ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integer2_dictionary
    // Examples: တစ်, နှစ်, သုံး, လေး, ငါး
    {
        let dict = &*INTEGER2_DICTIONARY;
        b.rule_1_terminal(
            "my:integer2_dictionary",
            b.reg(r"(?i)ကိုး|ခုနှစ်|ခြေါက်|ငါး|တစ်|တစ်ဆယ်|နှစ်|ရှစ်|လေး|သုံး").unwrap(),
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
    

    // Rule: Integer09 (refs: integer09Map)
    {
        
        
        let dict = &*INTEGER09_DICTIONARY;
        b.rule_1_terminal(
            "my:Integer09",
            b.reg(r"(?i)(၀|၁|၂|၃|၄|၅|၆|၇|၈|၉)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: IntegerPali (refs: integerPaliMap)
    {
        
        
        let dict = &*INTEGERPALI_DICTIONARY;
        b.rule_1_terminal(
            "my:IntegerPali",
            b.reg(r"(?i)(ပထမ|ဒုတိယ|တတိယ)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: integer2Map)
    {
        
        
        let dict = &*INTEGER2_DICTIONARY;
        b.rule_1_terminal(
            "my:Integer2",
            b.reg(r"(?i)(တစ်|နှစ်|သုံး|လေး|ငါး|ခြေါက်|ခုနှစ်|ရှစ်|ကိုး|တစ်ဆယ်)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (2)
    // ========================================
    

    // TODO: Integer3 (regex)
    
    //   Original: integer (11..19) 
    
    // Manual implementation required
    

    // TODO: Integer (regex)
    
    //   Original: integer 0
    
    // Manual implementation required
    

    
    eprintln!("⚠️  my/numeral has 2 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_my_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_my_numeral_dictionaries() {
        
        assert!(INTEGER09_DICTIONARY.len() > 0, "integer09_dictionary should not be empty");
        
        assert!(INTEGERPALI_DICTIONARY.len() > 0, "integerPali_dictionary should not be empty");
        
        assert!(INTEGER2_DICTIONARY.len() > 0, "integer2_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_my_numeral_stats() {
        // Generation statistics
        let total_rules = 8;
        let auto_generated = 6;
        let manual_needed = 2;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("my/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

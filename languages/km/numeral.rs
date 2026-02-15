// Auto-generated from Duckling/Numeral/KM/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: km
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("បី", 3);
        
        map.insert("បួន", 4);
        
        map.insert("ប្រាំ", 5);
        
        map.insert("ប្រាំបី", 8);
        
        map.insert("ប្រាំបួន", 9);
        
        map.insert("ប្រាំពីរ", 7);
        
        map.insert("ប្រាំមួយ", 6);
        
        map.insert("ពីរ", 2);
        
        map.insert("មួយ", 1);
        
        map.insert("សូន្យ", 0);
        
        map.insert("០", 0);
        
        map.insert("១", 1);
        
        map.insert("២", 2);
        
        map.insert("៣", 3);
        
        map.insert("៤", 4);
        
        map.insert("៥", 5);
        
        map.insert("៦", 6);
        
        map.insert("៧", 7);
        
        map.insert("៨", 8);
        
        map.insert("៩", 9);
        
        map
    };
}

// Dictionary: ruleTens_dictionary
lazy_static! {
    static ref RULETENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("កៅ", 90);
        
        map.insert("ចិត", 70);
        
        map.insert("ដប់", 10);
        
        map.insert("ប៉ែត", 80);
        
        map.insert("ម្ភៃ", 20);
        
        map.insert("សាម", 30);
        
        map.insert("សែ", 40);
        
        map.insert("ហា", 50);
        
        map.insert("ហុក", 60);
        
        map
    };
}


/// Build Numeral rules for km locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 4 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: ruleNumeral_dictionary
    // Examples: ០, ១, ២, ៣, ៤
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "km:ruleNumeral_dictionary",
            b.reg(r"(?i)បី|បួន|ប្រាំ|ប្រាំបី|ប្រាំបួន|ប្រាំពីរ|ប្រាំមួយ|ពីរ|មួយ|សូន្យ|០|១|២|៣|៤|៥|៦|៧|៨|៩").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: ruleTens_dictionary
    // Examples: ដប់, ម្ភៃ, សាម, សែ, ហា
    {
        let dict = &*RULETENS_DICTIONARY;
        b.rule_1_terminal(
            "km:ruleTens_dictionary",
            b.reg(r"(?i)កៅ|ចិត|ដប់|ប៉ែត|ម្ភៃ|សាម|សែ|ហា|ហុក").unwrap(),
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
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "km:Numeral",
            b.reg(r"(?i)(០|១|២|៣|៤|៥|៦|៧|៨|៩|ប្រាំបួន|ប្រាំបី|ប្រាំពីរ|ប្រាំមួយ|ប្រាំ|បួន|បី|ពីរ|មួយ|សូន្យ)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Tens (refs: ruleTensMap)
    {
        
        
        let dict = &*RULETENS_DICTIONARY;
        b.rule_1_terminal(
            "km:Tens",
            b.reg(r"(?i)(កៅ|ប៉ែត|ចិត|ហុក|ហា|សែ|សាម|ម្ភៃ|ដប់)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (4)
    // ========================================
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer (11..99)
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  km/numeral has 4 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_km_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_km_numeral_dictionaries() {
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
        assert!(RULETENS_DICTIONARY.len() > 0, "ruleTens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_km_numeral_stats() {
        // Generation statistics
        let total_rules = 8;
        let auto_generated = 4;
        let manual_needed = 4;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("km/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

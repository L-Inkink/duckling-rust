// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/HI/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: hi
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: ruleZeroToNinetyNine_dictionary
lazy_static! {
    static ref RULEZEROTONINETYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("अट्ठाईस", 28);
        
        map.insert("अट्ठानवे", 98);
        
        map.insert("अट्ठावन", 58);
        
        map.insert("अट्ठासी", 88);
        
        map.insert("अठहत्तर", 78);
        
        map.insert("अठारह", 18);
        
        map.insert("अड़तालीस", 48);
        
        map.insert("अड़तीस", 38);
        
        map.insert("अड़सठ", 68);
        
        map.insert("आठ", 8);
        
        map.insert("इकतालीस", 41);
        
        map.insert("इकतीस", 31);
        
        map.insert("इकत्तीस", 31);
        
        map.insert("इकसठ", 61);
        
        map.insert("इकहत्तर", 71);
        
        map.insert("इक्कीस", 21);
        
        map.insert("इक्यानवे", 91);
        
        map.insert("इक्यावन", 51);
        
        map.insert("इक्यासी", 81);
        
        map.insert("उनचास", 49);
        
        map.insert("उनतालीस", 39);
        
        map.insert("उनतीस", 29);
        
        map.insert("उनसठ", 59);
        
        map.insert("उनहत्तर", 69);
        
        map.insert("उनासी", 79);
        
        map.insert("उन्नीस", 19);
        
        map.insert("एक", 1);
        
        map.insert("ग्यारह", 11);
        
        map.insert("चार", 4);
        
        map.insert("चौंतीस", 34);
        
        map.insert("चौंसठ", 64);
        
        map.insert("चौदह", 14);
        
        map.insert("चौबीस", 24);
        
        map.insert("चौरानवे", 94);
        
        map.insert("चौरासी", 84);
        
        map.insert("चौवन", 54);
        
        map.insert("चौवालीस", 44);
        
        map.insert("चौहत्तर", 74);
        
        map.insert("छः", 6);
        
        map.insert("छत्तीस", 36);
        
        map.insert("छप्पन", 56);
        
        map.insert("छब्बीस", 26);
        
        map.insert("छह", 6);
        
        map.insert("छियानवे", 96);
        
        map.insert("छियालीस", 46);
        
        map.insert("छियासठ", 66);
        
        map.insert("छियासी", 86);
        
        map.insert("छिहत्तर", 76);
        
        map.insert("तिरानवे", 93);
        
        map.insert("तिरासी", 83);
        
        map.insert("तिरेपन", 53);
        
        map.insert("तिरेसठ", 63);
        
        map.insert("तिहत्तर", 73);
        
        map.insert("तीन", 3);
        
        map.insert("तेईस", 23);
        
        map.insert("तेरह", 13);
        
        map.insert("तैंतालीस", 43);
        
        map.insert("तैंतीस", 33);
        
        map.insert("दो", 2);
        
        map.insert("नवासी", 89);
        
        map.insert("निन्यानवे", 99);
        
        map.insert("नौ", 9);
        
        map.insert("पचपन", 55);
        
        map.insert("पचहत्तर", 75);
        
        map.insert("पचानवे", 95);
        
        map.insert("पचासी", 85);
        
        map.insert("पच्चीस", 25);
        
        map.insert("पन्द्रह", 15);
        
        map.insert("पाँच", 5);
        
        map.insert("पैंतालीस", 45);
        
        map.insert("पैंतीस", 35);
        
        map.insert("पैंसठ", 65);
        
        map.insert("बत्तीस", 32);
        
        map.insert("बयालीस", 42);
        
        map.insert("बयासी", 82);
        
        map.insert("बहत्तर", 72);
        
        map.insert("बाईस", 22);
        
        map.insert("बानवे", 92);
        
        map.insert("बारह", 12);
        
        map.insert("बावन", 52);
        
        map.insert("बासठ", 62);
        
        map.insert("शून्य", 0);
        
        map.insert("सड़सठ", 67);
        
        map.insert("सतहत्तर", 77);
        
        map.insert("सतासी", 87);
        
        map.insert("सत्ताईस", 27);
        
        map.insert("सत्तानवे", 97);
        
        map.insert("सत्तावन", 57);
        
        map.insert("सत्रह", 17);
        
        map.insert("सात", 7);
        
        map.insert("सैंतालीस", 47);
        
        map.insert("सैंतीस", 37);
        
        map.insert("सोलह", 16);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("अस्सी", 80);
        
        map.insert("चालीस", 40);
        
        map.insert("तीस", 30);
        
        map.insert("दस", 10);
        
        map.insert("नब्बे", 90);
        
        map.insert("पचास", 50);
        
        map.insert("बीस", 20);
        
        map.insert("सत्तर", 70);
        
        map.insert("साठ", 60);
        
        map
    };
}


/// Build Numeral rules for hi locale
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
    

    // Rule: ruleZeroToNinetyNine_dictionary
    // Examples: शून्य, एक, दो, तीन, चार
    {
        let dict = &*RULEZEROTONINETYNINE_DICTIONARY;
        b.rule_1_terminal(
            "hi:ruleZeroToNinetyNine_dictionary",
            b.reg(r"(?i)अट्ठाईस|अट्ठानवे|अट्ठावन|अट्ठासी|अठहत्तर|अठारह|अड़तालीस|अड़तीस|अड़सठ|आठ|इकतालीस|इकतीस|इकत्तीस|इकसठ|इकहत्तर|इक्कीस|इक्यानवे|इक्यावन|इक्यासी|उनचास|उनतालीस|उनतीस|उनसठ|उनहत्तर|उनासी|उन्नीस|एक|ग्यारह|चार|चौंतीस|चौंसठ|चौदह|चौबीस|चौरानवे|चौरासी|चौवन|चौवालीस|चौहत्तर|छः|छत्तीस|छप्पन|छब्बीस|छह|छियानवे|छियालीस|छियासठ|छियासी|छिहत्तर|तिरानवे|तिरासी|तिरेपन|तिरेसठ|तिहत्तर|तीन|तेईस|तेरह|तैंतालीस|तैंतीस|दो|नवासी|निन्यानवे|नौ|पचपन|पचहत्तर|पचानवे|पचासी|पच्चीस|पन्द्रह|पाँच|पैंतालीस|पैंतीस|पैंसठ|बत्तीस|बयालीस|बयासी|बहत्तर|बाईस|बानवे|बारह|बावन|बासठ|शून्य|सड़सठ|सतहत्तर|सतासी|सत्ताईस|सत्तानवे|सत्तावन|सत्रह|सात|सैंतालीस|सैंतीस|सोलह").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: दस, बीस, तीस, चालीस, पचास
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "hi:tens_dictionary",
            b.reg(r"(?i)अस्सी|चालीस|तीस|दस|नब्बे|पचास|बीस|सत्तर|साठ").unwrap(),
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
    

    // Rule: ZeroToNinetyNine (refs: ruleZeroToNinetyNineMap)
    {
        
        
        let dict = &*RULEZEROTONINETYNINE_DICTIONARY;
        b.rule_1_terminal(
            "hi:ZeroToNinetyNine",
            b.reg(r"(?i)(शून्य|एक|दो|तीन|चार|पाँच|छे|छह|सात|आठ|नौ|ग्यारह|बारह|तेरह|चौदह|पन्द्रह|सोलह|सत्रह|अठारह|उन्नीस|इक्कीस|बाईस|तेईस|चौबीस|पच्चीस|छब्बीस|सत्ताईस|अट्ठाईस|उनतीस|इकतीस|बत्तीस|तैंतीस|चौंतीस|पैंतीस|छत्तीस|सैंतीस|अड़तीस|उनतालीस|इकतालीस|बयालीस|तैंतालीस|चौवालीस|पैंतालीस|छियालीस|सैंतालीस|अड़तालीस|उनचास|इक्यावन|बावन|तिरेपन|चौवन|पचपन|छप्पन|सत्तावन|अट्ठावन|उनसठ|इकसठ|बासठ|तिरेसठ|चौंसठ|पैंसठ|छियासठ|सड़सठ|अड़सठ|उनहत्तर|इकहत्तर|बहत्तर|तिहत्तर|चौहत्तर|पचहत्तर|छिहत्तर|सतहत्तर|अठहत्तर|उनासी|इक्यासी|बयासी|तिरासी|चौरासी|पचासी|छियासी|सतासी|अट्ठासी|नवासी|इक्यानवे|बानवे|तिरानवे|चौरानवे|पचानवे|छियानवे|सत्तानवे|अट्ठानवे|निन्यानवे)").unwrap(),
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
            "hi:Tens",
            b.reg(r"(?i)(दस|बीस|तीस|चालीस|पचास|साठ|सत्तर|अस्सी|नब्बे)").unwrap(),
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
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: CompositeHundreds (composite)
    
    //   Original: integer 100s..
    
    // Manual implementation required
    

    // TODO: CompositeThousands (composite)
    
    //   Original: integer 100s..
    
    // Manual implementation required
    

    
    eprintln!("⚠️  hi/numeral has 4 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_hi_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_hi_numeral_dictionaries() {
        
        assert!(RULEZEROTONINETYNINE_DICTIONARY.len() > 0, "ruleZeroToNinetyNine_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_hi_numeral_stats() {
        // Generation statistics
        let total_rules = 8;
        let auto_generated = 4;
        let manual_needed = 4;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("hi/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

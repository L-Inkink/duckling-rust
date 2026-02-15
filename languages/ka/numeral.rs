// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/KA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ka
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("აათი", 10);
        
        map.insert("ათი", 10);
        
        map.insert("ერთი", 1);
        
        map.insert("ექვს", 6);
        
        map.insert("ექვსი", 6);
        
        map.insert("თერთმეტ", 11);
        
        map.insert("თერთმეტი", 11);
        
        map.insert("თექვსმეტ", 16);
        
        map.insert("თექვსმეტი", 16);
        
        map.insert("თვრამეტ", 18);
        
        map.insert("თვრამეტი", 18);
        
        map.insert("თოთხმეტ", 14);
        
        map.insert("თოთხმეტი", 14);
        
        map.insert("თორმეტ", 12);
        
        map.insert("თორმეტი", 12);
        
        map.insert("თხუთმეტ", 15);
        
        map.insert("თხუთმეტი", 15);
        
        map.insert("ნოლ", 0);
        
        map.insert("ნოლი", 0);
        
        map.insert("ნულ", 0);
        
        map.insert("ნული", 0);
        
        map.insert("ოთხ", 4);
        
        map.insert("ოთხი", 4);
        
        map.insert("ორ", 2);
        
        map.insert("ორი", 2);
        
        map.insert("რვ", 8);
        
        map.insert("რვა", 8);
        
        map.insert("სამ", 3);
        
        map.insert("სამი", 3);
        
        map.insert("შვიდ", 7);
        
        map.insert("შვიდი", 7);
        
        map.insert("ჩვიდმეტ", 17);
        
        map.insert("ჩვიდმეტი", 17);
        
        map.insert("ცამეტ", 13);
        
        map.insert("ცამეტი", 13);
        
        map.insert("ცხრ", 9);
        
        map.insert("ცხრა", 9);
        
        map.insert("ცხრამეტ", 19);
        
        map.insert("ცხრამეტი", 19);
        
        map.insert("ხუთ", 5);
        
        map.insert("ხუთი", 5);
        
        map
    };
}

// Dictionary: informal_dictionary
lazy_static! {
    static ref INFORMAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ერთი", 1);
        
        map.insert("რამდენიმე", 3);
        
        map.insert("რამოდენიმე", 3);
        
        map.insert("ცოტა", 3);
        
        map.insert("წყვილები", 2);
        
        map.insert("წყვილი", 2);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ოთხმოც", 80);
        
        map.insert("ოთხმოცდა", 80);
        
        map.insert("ოთხმოცდაათ", 90);
        
        map.insert("ოთხმოცდაათი", 90);
        
        map.insert("ოთხმოცი", 80);
        
        map.insert("ორმოც", 40);
        
        map.insert("ორმოცდა", 40);
        
        map.insert("ორმოცდაათ", 50);
        
        map.insert("ორმოცდაათი", 50);
        
        map.insert("ორმოცი", 40);
        
        map.insert("ოც", 20);
        
        map.insert("ოცდა", 20);
        
        map.insert("ოცდაათ", 30);
        
        map.insert("ოცდაათი", 30);
        
        map.insert("ოცი", 20);
        
        map.insert("სამოც", 60);
        
        map.insert("სამოცდა", 60);
        
        map.insert("სამოცდაათ", 70);
        
        map.insert("სამოცდაათი", 70);
        
        map.insert("სამოცი", 60);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ას", 100);
        
        map.insert("ასი", 100);
        
        map.insert("ექვს ას", 600);
        
        map.insert("ექვს ასი", 600);
        
        map.insert("ექვსას", 600);
        
        map.insert("ექვსასი", 600);
        
        map.insert("ოთხ ას", 400);
        
        map.insert("ოთხ ასი", 400);
        
        map.insert("ოთხას", 400);
        
        map.insert("ოთხასი", 400);
        
        map.insert("ორ ას", 200);
        
        map.insert("ორ ასი", 200);
        
        map.insert("ორას", 200);
        
        map.insert("ორასი", 200);
        
        map.insert("რვა ას", 800);
        
        map.insert("რვა ასი", 800);
        
        map.insert("რვაას", 800);
        
        map.insert("რვაასი", 800);
        
        map.insert("სამ ას", 300);
        
        map.insert("სამ ასი", 300);
        
        map.insert("სამას", 300);
        
        map.insert("სამასი", 300);
        
        map.insert("შვიდ ას", 700);
        
        map.insert("შვიდ ასი", 700);
        
        map.insert("შვიდას", 700);
        
        map.insert("შვიდასი", 700);
        
        map.insert("ცხრა ას", 900);
        
        map.insert("ცხრა ასი", 900);
        
        map.insert("ცხრაას", 900);
        
        map.insert("ცხრაასი", 900);
        
        map.insert("ხუთ ას", 500);
        
        map.insert("ხუთ ასი", 500);
        
        map.insert("ხუთას", 500);
        
        map.insert("ხუთასი", 500);
        
        map
    };
}


/// Build Numeral rules for ka locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 12 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: ნოლ, ნულ, ნული, ნოლი, ერთი
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ka:zeroNineteen_dictionary",
            b.reg(r"(?i)აათი|ათი|ერთი|ექვს|ექვსი|თერთმეტ|თერთმეტი|თექვსმეტ|თექვსმეტი|თვრამეტ|თვრამეტი|თოთხმეტ|თოთხმეტი|თორმეტ|თორმეტი|თხუთმეტ|თხუთმეტი|ნოლ|ნოლი|ნულ|ნული|ოთხ|ოთხი|ორ|ორი|რვ|რვა|სამ|სამი|შვიდ|შვიდი|ჩვიდმეტ|ჩვიდმეტი|ცამეტ|ცამეტი|ცხრ|ცხრა|ცხრამეტ|ცხრამეტი|ხუთ|ხუთი").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: informal_dictionary
    // Examples: ერთი, წყვილი, წყვილები, ცოტა, რამდენიმე
    {
        let dict = &*INFORMAL_DICTIONARY;
        b.rule_1_terminal(
            "ka:informal_dictionary",
            b.reg(r"(?i)ერთი|რამდენიმე|რამოდენიმე|ცოტა|წყვილები|წყვილი").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ოცი, ოცდა, ოც, ოცდაათ, ოცდაათი
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "ka:tens_dictionary",
            b.reg(r"(?i)ოთხმოც|ოთხმოცდა|ოთხმოცდაათ|ოთხმოცდაათი|ოთხმოცი|ორმოც|ორმოცდა|ორმოცდაათ|ორმოცდაათი|ორმოცი|ოც|ოცდა|ოცდაათ|ოცდაათი|ოცი|სამოც|სამოცდა|სამოცდაათ|სამოცდაათი|სამოცი").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: ასი, ორასი, სამასი, ოთხასი, ხუთასი
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "ka:hundreds_dictionary",
            b.reg(r"(?i)ას|ასი|ექვს ას|ექვს ასი|ექვსას|ექვსასი|ოთხ ას|ოთხ ასი|ოთხას|ოთხასი|ორ ას|ორ ასი|ორას|ორასი|რვა ას|რვა ასი|რვაას|რვაასი|სამ ას|სამ ასი|სამას|სამასი|შვიდ ას|შვიდ ასი|შვიდას|შვიდასი|ცხრა ას|ცხრა ასი|ცხრაას|ცხრაასი|ხუთ ას|ხუთ ასი|ხუთას|ხუთასი").unwrap(),
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
    

    // Rule: ToNineteen (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "ka:ToNineteen",
            b.reg(r"(?i)(წყვილ(ებ)?ი|ცოტა|რამდენიმე|რამოდენიმე|ნოლი?|ნული?|ერთი|ორი?|სამი?|ოთხი?|ხუთი?|ექვსი?|შვიდი?|რვა|თერთმეტი?|თორმეტი?|ცამეტი?|თოთხმეტი?|თხუთმეტი?|თექვსმეტი?|ჩვიდმეტი?|თვრამეტი?|ცხრამეტი?|ცხრა|ა?ათი)").unwrap(),
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
            "ka:Tens",
            b.reg(r"(?i)(ოცდაათი?|ორმოცდაათი?|სამოცდაათი?|ოთხმოცდაათი?|ოცდა|ორმოცდა|სამოცდა|ოთხმოცდა|ოცი?|ორმოცი?|სამოცი?|ოთხმოცი?)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Hundreds (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "ka:Hundreds",
            b.reg(r"(?i)(ასი?|ორ ?ასი?|სამ ?ასი?|ოთხ ?ასი?|ხუთ ?ასი?|ექვს ?ასი?|შვიდ ?ასი?|რვა ?ასი?|ცხრა ?ასი?)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (12)
    // ========================================
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: LeadingDotSpelledOut (regex)
    
    //   Original: point 77
    
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
    
    //   Original: integer 100..999
    
    // Manual implementation required
    

    // TODO: CompositeHundredsAndUnits (composite)
    
    //   Original: integer 100..999
    
    // Manual implementation required
    

    // TODO: CompositeHundredsAndTens (composite)
    
    //   Original: integer 100..999
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: SumAnd (composite)
    
    //   Original: intersect 2 numbers (with and)
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ka/numeral has 12 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ka_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ka_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(INFORMAL_DICTIONARY.len() > 0, "informal_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ka_numeral_stats() {
        // Generation statistics
        let total_rules = 19;
        let auto_generated = 7;
        let manual_needed = 12;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ka/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

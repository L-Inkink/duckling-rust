// Auto-generated from Duckling/Numeral/KO/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ko
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: integerForOrdinals_dictionary
lazy_static! {
    static ref INTEGERFORORDINALS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("네", 4);
        
        map.insert("두", 2);
        
        map.insert("세", 3);
        
        map.insert("첫", 1);
        
        map.insert("한", 1);
        
        map
    };
}

// Dictionary: integerTypeAndOrdinals_dictionary
lazy_static! {
    static ref INTEGERTYPEANDORDINALS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("마흔", 40);
        
        map.insert("서른", 30);
        
        map.insert("쉰", 50);
        
        map.insert("스물", 20);
        
        map.insert("아흔", 90);
        
        map.insert("여든", 80);
        
        map.insert("열", 10);
        
        map.insert("예순", 60);
        
        map.insert("일흔", 70);
        
        map
    };
}

// Dictionary: integerType1_dictionary
lazy_static! {
    static ref INTEGERTYPE1_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("구", 9);
        
        map.insert("사", 4);
        
        map.insert("삼", 3);
        
        map.insert("영", 0);
        
        map.insert("오", 5);
        
        map.insert("육", 6);
        
        map.insert("이", 2);
        
        map.insert("일", 1);
        
        map.insert("칠", 7);
        
        map.insert("팔", 8);
        
        map
    };
}

// Dictionary: integerType2_dictionary
lazy_static! {
    static ref INTEGERTYPE2_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("넷", 4);
        
        map.insert("다섯", 5);
        
        map.insert("둘", 2);
        
        map.insert("셋", 3);
        
        map.insert("아홉", 9);
        
        map.insert("여덟", 8);
        
        map.insert("여섯", 6);
        
        map.insert("일곱", 7);
        
        map.insert("하나", 1);
        
        map
    };
}


/// Build Numeral rules for ko locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 1 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: integerForOrdinals_dictionary
    // Examples: 한, 첫, 두, 세, 네
    {
        let dict = &*INTEGERFORORDINALS_DICTIONARY;
        b.rule_1_terminal(
            "ko:integerForOrdinals_dictionary",
            b.reg(r"(?i)네|두|세|첫|한").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integerTypeAndOrdinals_dictionary
    // Examples: 열, 스물, 서른, 마흔, 쉰
    {
        let dict = &*INTEGERTYPEANDORDINALS_DICTIONARY;
        b.rule_1_terminal(
            "ko:integerTypeAndOrdinals_dictionary",
            b.reg(r"(?i)마흔|서른|쉰|스물|아흔|여든|열|예순|일흔").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integerType1_dictionary
    // Examples: 영, 일, 이, 삼, 사
    {
        let dict = &*INTEGERTYPE1_DICTIONARY;
        b.rule_1_terminal(
            "ko:integerType1_dictionary",
            b.reg(r"(?i)구|사|삼|영|오|육|이|일|칠|팔").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integerType2_dictionary
    // Examples: 하나, 둘, 셋, 넷, 다섯
    {
        let dict = &*INTEGERTYPE2_DICTIONARY;
        b.rule_1_terminal(
            "ko:integerType2_dictionary",
            b.reg(r"(?i)넷|다섯|둘|셋|아홉|여덟|여섯|일곱|하나").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Constant Value Regex Rules (1)
    // ========================================
    

    // Rule: Few → 3
    b.rule_1_terminal(
        "ko:Few",
        b.reg(r"(?i)몇").unwrap(),
        |_| {
            Ok(Value::Integer(3))
        }
    );
    
    

    
    // ========================================
    // Dictionary-Reference Regex Rules (4)
    // ========================================
    

    // Rule: IntegerForOrdinals (refs: integerForOrdinalsMap)
    {
        
        
        let dict = &*INTEGERFORORDINALS_DICTIONARY;
        b.rule_1_terminal(
            "ko:IntegerForOrdinals",
            b.reg(r"(?i)(한|첫|두|세|네)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: IntegerTypeAndOrdinals (refs: integerTypeAndOrdinalsMap)
    {
        
        
        let dict = &*INTEGERTYPEANDORDINALS_DICTIONARY;
        b.rule_1_terminal(
            "ko:IntegerTypeAndOrdinals",
            b.reg(r"(?i)(열|스물|서른|마흔|쉰|예순|일흔|여든|아흔)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: IntegerType1 (refs: integerType1Map)
    {
        
        
        let dict = &*INTEGERTYPE1_DICTIONARY;
        b.rule_1_terminal(
            "ko:IntegerType1",
            b.reg(r"(?i)(영|일|이|삼|사|오|육|칠|팔|구)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: IntegerType2 (refs: integerType2Map)
    {
        
        
        let dict = &*INTEGERTYPE2_DICTIONARY;
        b.rule_1_terminal(
            "ko:IntegerType2",
            b.reg(r"(?i)(하나|둘|셋|넷|다섯|여섯|일곱|여덟|아홉)").unwrap(),
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
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithOr (regex)
    
    //   Original: numbers prefix with -, 마이너스, or 마이나스
    
    // Manual implementation required
    

    // TODO: Half (regex)
    
    //   Original: half - 반
    
    // Manual implementation required
    

    // TODO: Integer (regex)
    
    //   Original: integer 0
    
    // Manual implementation required
    

    // TODO: IntegerType1PowersOfTen (regex)
    
    //   Original: integer - TYPE 1: powers of ten
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: IntegerType3 (composite)
    
    //   Original: integer (21..99) - TYPE 2
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ko/numeral has 9 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ko_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ko_numeral_dictionaries() {
        
        assert!(INTEGERFORORDINALS_DICTIONARY.len() > 0, "integerForOrdinals_dictionary should not be empty");
        
        assert!(INTEGERTYPEANDORDINALS_DICTIONARY.len() > 0, "integerTypeAndOrdinals_dictionary should not be empty");
        
        assert!(INTEGERTYPE1_DICTIONARY.len() > 0, "integerType1_dictionary should not be empty");
        
        assert!(INTEGERTYPE2_DICTIONARY.len() > 0, "integerType2_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ko_numeral_stats() {
        // Generation statistics
        let total_rules = 18;
        let auto_generated = 9;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ko/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

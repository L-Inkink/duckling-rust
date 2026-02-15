// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/ZH/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: zh
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: integer_dictionary
lazy_static! {
    static ref INTEGER_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("〇", 0);
        
        map.insert("一", 1);
        
        map.insert("七", 7);
        
        map.insert("三", 3);
        
        map.insert("两", 2);
        
        map.insert("九", 9);
        
        map.insert("二", 2);
        
        map.insert("五", 5);
        
        map.insert("伍", 5);
        
        map.insert("兩", 2);
        
        map.insert("八", 8);
        
        map.insert("六", 6);
        
        map.insert("十", 10);
        
        map.insert("參", 3);
        
        map.insert("四", 4);
        
        map.insert("壹", 1);
        
        map.insert("拾", 10);
        
        map.insert("捌", 8);
        
        map.insert("柒", 7);
        
        map.insert("玖", 9);
        
        map.insert("肆", 4);
        
        map.insert("貳", 2);
        
        map.insert("陸", 6);
        
        map.insert("零", 0);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("卅", 30);
        
        map.insert("卌", 40);
        
        map.insert("廿", 20);
        
        map
    };
}


/// Build Numeral rules for zh locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 2 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 10 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: integer_dictionary
    // Examples: 〇, 零, 一, 壹, 兩
    {
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "zh:integer_dictionary",
            b.reg(r"(?i)〇|一|七|三|两|九|二|五|伍|兩|八|六|十|參|四|壹|拾|捌|柒|玖|肆|貳|陸|零").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: 廿, 卅, 卌
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "zh:tens_dictionary",
            b.reg(r"(?i)卅|卌|廿").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Constant Value Regex Rules (2)
    // ========================================
    

    // Rule: Dozen → 12
    b.rule_1_terminal(
        "zh:Dozen",
        b.reg(r"(?i)打").unwrap(),
        |_| {
            Ok(Value::Integer(12))
        }
    );
    

    // Rule: Pair → 2
    b.rule_1_terminal(
        "zh:Pair",
        b.reg(r"(?i)雙|對").unwrap(),
        |_| {
            Ok(Value::Integer(2))
        }
    );
    
    

    
    // ========================================
    // Dictionary-Reference Regex Rules (2)
    // ========================================
    

    // Rule: Integer (refs: integerMap)
    {
        
        
        let dict = &*INTEGER_DICTIONARY;
        b.rule_1_terminal(
            "zh:Integer",
            b.reg(r"(?i)(〇|零|一|二|两|兩|三|四|五|六|七|八|九|十|壹|貳|參|肆|伍|陸|柒|捌|玖|拾)").unwrap(),
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
            "zh:Tens",
            b.reg(r"(?i)(廿|卅|卌)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (10)
    // ========================================
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Half (regex)
    
    //   Original: half
    
    // Manual implementation required
    

    // TODO: HundredPrefix (regex)
    
    //   Original: one hundred and <integer> (short form)
    
    // Manual implementation required
    

    // TODO: ThousandPrefix (regex)
    
    //   Original: one thousand and <integer> (short form)
    
    // Manual implementation required
    

    // TODO: TenThousandPrefix (regex)
    
    //   Original: ten thousand and <integer> (short form)
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..49
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: NumeralsIntersectConsecutiveUnit (composite)
    
    //   Original: integer with consecutive unit modifiers
    
    // Manual implementation required
    

    
    eprintln!("⚠️  zh/numeral has 10 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_zh_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_zh_numeral_dictionaries() {
        
        assert!(INTEGER_DICTIONARY.len() > 0, "integer_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_zh_numeral_stats() {
        // Generation statistics
        let total_rules = 16;
        let auto_generated = 6;
        let manual_needed = 10;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("zh/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

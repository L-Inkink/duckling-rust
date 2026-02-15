// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/EN/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: en
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use crate::dim;
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("eight", 8);
        
        map.insert("eighteen", 18);
        
        map.insert("eleven", 11);
        
        map.insert("fifteen", 15);
        
        map.insert("five", 5);
        
        map.insert("four", 4);
        
        map.insert("fourteen", 14);
        
        map.insert("naught", 0);
        
        map.insert("nil", 0);
        
        map.insert("nine", 9);
        
        map.insert("nineteen", 19);
        
        map.insert("none", 0);
        
        map.insert("nought", 0);
        
        map.insert("one", 1);
        
        map.insert("seven", 7);
        
        map.insert("seventeen", 17);
        
        map.insert("six", 6);
        
        map.insert("sixteen", 16);
        
        map.insert("ten", 10);
        
        map.insert("thirteen", 13);
        
        map.insert("three", 3);
        
        map.insert("twelve", 12);
        
        map.insert("two", 2);
        
        map.insert("zero", 0);
        
        map.insert("zilch", 0);
        
        map
    };
}

// Dictionary: informal_dictionary
lazy_static! {
    static ref INFORMAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("a couple", 2);
        
        map.insert("a couple of", 2);
        
        map.insert("a few", 3);
        
        map.insert("a pair", 2);
        
        map.insert("a pair of", 2);
        
        map.insert("couple", 2);
        
        map.insert("couple of", 2);
        
        map.insert("couples", 2);
        
        map.insert("couples of", 2);
        
        map.insert("few", 3);
        
        map.insert("pair", 2);
        
        map.insert("pair of", 2);
        
        map.insert("pairs", 2);
        
        map.insert("pairs of", 2);
        
        map.insert("single", 1);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("eighty", 80);
        
        map.insert("fifty", 50);
        
        map.insert("forty", 40);
        
        map.insert("fourty", 40);
        
        map.insert("ninety", 90);
        
        map.insert("seventy", 70);
        
        map.insert("sixty", 60);
        
        map.insert("thirty", 30);
        
        map.insert("twenty", 20);
        
        map
    };
}

// Dictionary: powersOfTens_dictionary
lazy_static! {
    static ref POWERSOFTENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("billion", 9);
        
        map.insert("cr", 7);
        
        map.insert("crore", 7);
        
        map.insert("hundred", 2);
        
        map.insert("koti", 7);
        
        map.insert("kr", 7);
        
        map.insert("krore", 7);
        
        map.insert("l", 5);
        
        map.insert("lac", 5);
        
        map.insert("lakh", 5);
        
        map.insert("lkh", 5);
        
        map.insert("million", 6);
        
        map.insert("thousand", 3);
        
        map.insert("trillion", 12);
        
        map
    };
}


/// Build Numeral rules for en locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 1 constant regex rules
///   - 5 dictionary-reference regex rules
///   - 9 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: naught, nil, nought, none, zero
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "en:zeroNineteen_dictionary",
            b.reg(r"(?i)eight|eighteen|eleven|fifteen|five|four|fourteen|naught|nil|nine|nineteen|none|nought|one|seven|seventeen|six|sixteen|ten|thirteen|three|twelve|two|zero|zilch").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: informal_dictionary
    // Examples: single, a couple, a couple of, couple, couples
    {
        let dict = &*INFORMAL_DICTIONARY;
        b.rule_1_terminal(
            "en:informal_dictionary",
            b.reg(r"(?i)a couple|a couple of|a few|a pair|a pair of|couple|couple of|couples|couples of|few|pair|pair of|pairs|pairs of|single").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: twenty, thirty, forty, fourty, fifty
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "en:tens_dictionary",
            b.reg(r"(?i)eighty|fifty|forty|fourty|ninety|seventy|sixty|thirty|twenty").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: powersOfTens_dictionary
    // Examples: hundred, thousand, lakh, lkh, l
    {
        let dict = &*POWERSOFTENS_DICTIONARY;
        b.rule_1_terminal(
            "en:powersOfTens_dictionary",
            b.reg(r"(?i)billion|cr|crore|hundred|koti|kr|krore|l|lac|lakh|lkh|million|thousand|trillion").unwrap(),
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
    

    // Rule: Dozen → 12
    b.rule_1_terminal(
        "en:Dozen",
        b.reg(r"(?i)(?:a )?dozens?(?: of)?").unwrap(),
        |_| {
            Ok(Value::Integer(12))
        }
    );
    
    

    
    // ========================================
    // Dictionary-Reference Regex Rules (5)
    // ========================================
    

    // Rule: ToNineteen (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "en:ToNineteen",
            b.reg(r"(?i)(?:none|zilch|naught|nought|nil|zero|one|single|two|(?:a )?(?:pair|couple)s?(?: of)?|three|(?:a )?few|fourteen|four|fifteen|five|sixteen|six|seventeen|seven|eighteen|eight|nineteen|nine|ten|eleven|twelve|thirteen)").unwrap(),
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
            "en:Tens",
            b.reg(r"(?i)(twenty|thirty|fou?rty|fifty|sixty|seventy|eighty|ninety)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: PowersOfTen (refs: powersOfTensMap)
    {
        
        
        let dict = &*POWERSOFTENS_DICTIONARY;
        b.rule_1_terminal(
            "en:PowersOfTen",
            b.reg(r"(?i)(?:hundred|thousand|l(?:ac|(?:a?kh)?)|million|(?:(?:k|c)r(?:ore)?|koti)|billion)s?").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&exponent| {
                        // Convert exponent to actual value: 10^exponent
                        let value = 10_i64.pow(exponent as u32);
                        Value::Integer(value)
                    })
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: SkipHundreds1 (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "en:SkipHundreds1",
            b.reg(r"(?i)(one|two|three|four|five|six|seven|eight|nine)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: SkipHundreds2 (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "en:SkipHundreds2",
            b.reg(r"(?i)(one|two|three|four|five|six|seven|eight|nine)").unwrap(),
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
    

    // Rule: LeadingDotSpelledOut - "point 77" → 0.77, "dot 5" → 0.05
    b.rule_2(
        "en:leading_dot_spelled_out",
        b.reg(r"(?i)point|dot").unwrap(),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match positive numbers without grain (simple integers 0-99)
            matches!(v, Value::Integer(n) if *n >= 0 && *n < 100)
        })]),
        |_point, number| {
            if let Value::Integer(n) = number.value() {
                // Convert integer to 0.xx format
                // "point 77" → 0.77, "point 5" → 0.05
                let decimal = *n as f64 / 100.0;
                Ok(Value::Float(decimal))
            } else {
                Err(rustling_error!("LeadingDotSpelledOut: Expected integer"))
            }
        }
    );


    // Rule: Decimals - Parse decimal numbers like "0.5", "3.14", ".25"
    b.rule_1_terminal(
        "en:decimals",
        b.reg(r#"(?i)(\d*\.\d+)"#).unwrap(),
        |text_match| {
            let text = text_match.group(1);
            text.parse::<f64>()
                .map(|f| Value::Float(f))
                .map_err(|_| rustling_error!("Failed to parse decimal: {}", text))
        }
    );
    

    // Rule: Commas - Parse comma-separated numbers like "1,234" or "1,234.56"
    b.rule_1_terminal(
        "en:commas",
        b.reg(r#"(?i)\d+(?:,\d\d\d)+(?:\.\d+)?"#).unwrap(),
        |text_match| {
            let text = text_match.group(0).replace(",", "");
            text.parse::<f64>()
                .map(|f| {
                    if f.fract() == 0.0 {
                        Value::Integer(f as i64)
                    } else {
                        Value::Float(f)
                    }
                })
                .map_err(|_| rustling_error!("Failed to parse comma-separated number: {}", text))
        }
    );
    

    // Rule: Negative - "-5" → -5, "negative 10" → -10, "minus 5" → -5
    b.rule_2(
        "en:negative",
        b.reg(r"(?i)-|minus|negative").unwrap(),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match positive numbers (not already negative)
            match v {
                Value::Integer(n) if *n > 0 => true,
                Value::Float(f) if *f > 0.0 => true,
                _ => false,
            }
        })]),
        |_sign, number| {
            match number.value() {
                Value::Integer(n) => Ok(Value::Integer(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(rustling_error!("Negative: Expected positive number"))
            }
        }
    );


    // Rule: IntegerNumeric - Parse integer numbers like "5", "23", "100"
    // Note: Placed after Decimals and Commas to avoid matching parts of "0.5" or "1,234"
    b.rule_1_terminal(
        "en:integer_numeric",
        b.reg(r#"(\d+)"#).unwrap(),
        |text_match| {
            let text = text_match.group(1);
            text.parse::<i64>()
                .map(|i| Value::Integer(i))
                .map_err(|_| rustling_error!("Failed to parse integer: {}", text))
        }
    );


    // Rule: CompositeTens - Combine tens and units (e.g., "twenty three" = 23)
    b.rule_2(
        "en:composite_tens",
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match tens: 20, 30, 40, ..., 90
            matches!(v, Value::Integer(n) if *n >= 20 && *n <= 90 && *n % 10 == 0)
        })]),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match units: 1, 2, 3, ..., 9
            matches!(v, Value::Integer(n) if *n >= 1 && *n < 10)
        })]),
        |tens, units| {
            if let (Value::Integer(t), Value::Integer(u)) = (tens.value(), units.value()) {
                Ok(Value::Integer(t + u))
            } else {
                Err(rustling_error!("CompositeTens: Invalid value types"))
            }
        }
    );
    

    // TODO: Sum (composite)

    //   Original: intersect 2 numbers

    // Rule: Sum - Add two adjacent numbers without "and" (e.g., "one thousand two hundred" = 1200)
    // Pattern: larger number (>= 100) + smaller non-multipliable number
    b.rule_2(
        "en:sum",
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match numbers >= 100 (result of multiplication or large values)
            matches!(v, Value::Integer(n) if *n >= 100)
        })]),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match positive numbers that are NOT powers of 10 (not multipliable)
            if let Value::Integer(n) = v {
                *n > 0 && (*n < 100 || (*n as f64).log10().fract() != 0.0)
            } else {
                false
            }
        })]),
        |first, second| {
            if let (Value::Integer(a), Value::Integer(b)) = (first.value(), second.value()) {
                // Only sum if the second number is smaller than the first
                // This prevents incorrect sums like 100 + 1000
                if b < a {
                    Ok(Value::Integer(a + b))
                } else {
                    Err(rustling_error!("Sum: second value must be smaller than first"))
                }
            } else {
                Err(rustling_error!("Sum: Invalid value types"))
            }
        }
    );


    // Rule: SumAnd - Add numbers with "and" (e.g., "one hundred and twenty three" = 123)
    b.rule_3(
        "en:sum_and",
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match larger numbers (>= 100)
            matches!(v, Value::Integer(n) if *n >= 100)
        })]),
        b.reg(r"(?i)and").unwrap(),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match smaller numbers (< 100, not multipliable)
            matches!(v, Value::Integer(n) if *n > 0 && *n < 100)
        })]),
        |big, _and, small| {
            if let (Value::Integer(a), Value::Integer(b)) = (big.value(), small.value()) {
                Ok(Value::Integer(a + b))
            } else {
                Err(rustling_error!("SumAnd: Invalid value types"))
            }
        }
    );
    

    // Rule: Multiply - Compose by multiplication (e.g., "three hundred" = 300)
    b.rule_2(
        "en:multiply",
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match positive integers 1-99
            matches!(v, Value::Integer(n) if *n > 0 && *n < 100)
        })]),
        dim!(Value, vec![Box::new(|v: &Value| {
            // Match multipliable values (powers of 10: 100, 1000, etc.)
            matches!(v, Value::Integer(n) if *n >= 100 && (*n as f64).log10().fract() == 0.0)
        })]),
        |multiplicand, multiplier| {
            if let (Value::Integer(a), Value::Integer(b)) = (multiplicand.value(), multiplier.value()) {
                Ok(Value::Integer(a * b))
            } else {
                Err(rustling_error!("Multiply: Invalid value types"))
            }
        }
    );
    

    // TODO: LegalParentheses (composite)
    
    //   Original: <integer> '('<integer>')'
    
    // Manual implementation required
    

    
    eprintln!("⚠️  en/numeral has 9 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_en_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_en_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(INFORMAL_DICTIONARY.len() > 0, "informal_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(POWERSOFTENS_DICTIONARY.len() > 0, "powersOfTens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_en_numeral_stats() {
        // Generation statistics
        let total_rules = 19;
        let auto_generated = 10;
        let manual_needed = 9;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("en/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

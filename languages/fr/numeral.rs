// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/FR/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: fr
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: ruleNumeral2_dictionary
lazy_static! {
    static ref RULENUMERAL2_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinquante", 50);
        
        map.insert("quarante", 40);
        
        map.insert("soixante", 60);
        
        map.insert("trente", 30);
        
        map.insert("vingt", 20);
        
        map
    };
}

// Dictionary: ruleNumeral_dictionary
lazy_static! {
    static ref RULENUMERAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinq", 5);
        
        map.insert("deux", 2);
        
        map.insert("dix", 10);
        
        map.insert("douze", 12);
        
        map.insert("huit", 8);
        
        map.insert("neuf", 9);
        
        map.insert("onze", 11);
        
        map.insert("quatorze", 14);
        
        map.insert("quatre", 4);
        
        map.insert("quinze", 15);
        
        map.insert("seize", 16);
        
        map.insert("sept", 7);
        
        map.insert("six", 6);
        
        map.insert("treize", 13);
        
        map.insert("trois", 3);
        
        map.insert("un", 1);
        
        map.insert("une", 1);
        
        map.insert("zero", 0);
        
        map.insert("zéro", 0);
        
        map
    };
}


/// Build Numeral rules for fr locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 2 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: ruleNumeral2_dictionary
    // Examples: vingt, trente, quarante, cinquante, soixante
    {
        let dict = &*RULENUMERAL2_DICTIONARY;
        b.rule_1_terminal(
            "fr:ruleNumeral2_dictionary",
            b.reg(r"(?i)cinquante|quarante|soixante|trente|vingt").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: ruleNumeral_dictionary
    // Examples: zero, zéro, un, une, deux
    {
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "fr:ruleNumeral_dictionary",
            b.reg(r"(?i)cinq|deux|dix|douze|huit|neuf|onze|quatorze|quatre|quinze|seize|sept|six|treize|trois|un|une|zero|zéro").unwrap(),
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
    

    // Rule: Numeral2 (refs: ruleNumeral2Map)
    {
        
        
        let dict = &*RULENUMERAL2_DICTIONARY;
        b.rule_1_terminal(
            "fr:Numeral2",
            b.reg(r"(?i)(vingt|trente|quarante|cinquante|soixante)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Numeral (refs: ruleNumeralMap)
    {
        
        
        let dict = &*RULENUMERAL_DICTIONARY;
        b.rule_1_terminal(
            "fr:Numeral",
            b.reg(r"(?i)(z(e|é)ro|une?|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|onze|douze|treize|quatorze|quinze|seize)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (11)
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
    

    // TODO: Numeral4 (regex)
    
    //   Original: number 80
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Numerals4 (composite)
    
    //   Original: numbers 81
    
    // Manual implementation required
    

    // TODO: Numerals2 (composite)
    
    //   Original: numbers 22..29 32..39 .. 52..59
    
    // Manual implementation required
    

    // TODO: Numerals5 (composite)
    
    //   Original: numbers 62..69 .. 92..99
    
    // Manual implementation required
    

    // TODO: Numerals (composite)
    
    //   Original: numbers 21 31 41 51
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  fr/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_fr_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_fr_numeral_dictionaries() {
        
        assert!(RULENUMERAL2_DICTIONARY.len() > 0, "ruleNumeral2_dictionary should not be empty");
        
        assert!(RULENUMERAL_DICTIONARY.len() > 0, "ruleNumeral_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_fr_numeral_stats() {
        // Generation statistics
        let total_rules = 15;
        let auto_generated = 4;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("fr/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

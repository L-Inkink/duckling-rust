// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/NL/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: nl
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("acht", 8);
        
        map.insert("achttien", 18);
        
        map.insert("dertien", 13);
        
        map.insert("drie", 3);
        
        map.insert("een", 1);
        
        map.insert("elf", 11);
        
        map.insert("geen", 0);
        
        map.insert("negen", 9);
        
        map.insert("negentien", 19);
        
        map.insert("niks", 0);
        
        map.insert("nul", 0);
        
        map.insert("tien", 10);
        
        map.insert("twaalf", 12);
        
        map.insert("twee", 2);
        
        map.insert("veertien", 14);
        
        map.insert("vier", 4);
        
        map.insert("vijf", 5);
        
        map.insert("vijftien", 15);
        
        map.insert("zes", 6);
        
        map.insert("zestien", 16);
        
        map.insert("zeven", 7);
        
        map.insert("zeventien", 17);
        
        map.insert("één", 1);
        
        map
    };
}

// Dictionary: dozen_dictionary
lazy_static! {
    static ref DOZEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("dertig", 30);
        
        map.insert("negentig", 90);
        
        map.insert("tachtig", 80);
        
        map.insert("twintig", 20);
        
        map.insert("veertig", 40);
        
        map.insert("vijftig", 50);
        
        map.insert("zestig", 60);
        
        map.insert("zeventig", 70);
        
        map
    };
}


/// Build Numeral rules for nl locale
///
/// Auto-generated rules:
///   - 2 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (2)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: niks, nul, geen, één, een
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "nl:zeroNineteen_dictionary",
            b.reg(r"(?i)acht|achttien|dertien|drie|een|elf|geen|negen|negentien|niks|nul|tien|twaalf|twee|veertien|vier|vijf|vijftien|zes|zestien|zeven|zeventien|één").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: dozen_dictionary
    // Examples: twintig, dertig, veertig, vijftig, zestig
    {
        let dict = &*DOZEN_DICTIONARY;
        b.rule_1_terminal(
            "nl:dozen_dictionary",
            b.reg(r"(?i)dertig|negentig|tachtig|twintig|veertig|vijftig|zestig|zeventig").unwrap(),
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
    

    // Rule: Integer3 (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "nl:Integer3",
            b.reg(r"(?i)(een|twee|drie|vier|vijf|zes|zeven|acht|negen)(?:e|ë)n(twintig|dertig|veertig|vijftig|zestig|zeventig|tachtig|negentig)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "nl:Integer",
            b.reg(r"(?i)(geen|nul|niks|een|één|twee|drie|vier|vijftien|vijf|zestien|zes|zeventien|zeven|achttien|acht|negentien|negen|tien|elf|twaalf|dertien|veertien)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: dozenMap)
    {
        
        
        let dict = &*DOZEN_DICTIONARY;
        b.rule_1_terminal(
            "nl:Integer2",
            b.reg(r"(?i)(twintig|dertig|veertig|vijftig|zestig|zeventig|tachtig|negentig)").unwrap(),
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
    

    // TODO: Few (regex)
    
    //   Original: few
    
    // Manual implementation required
    

    // TODO: Ten (regex)
    
    //   Original: ten
    
    // Manual implementation required
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: Couple (regex)
    
    //   Original: couple
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: Gross (regex)
    
    //   Original: gros
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    
    eprintln!("⚠️  nl/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_nl_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_nl_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(DOZEN_DICTIONARY.len() > 0, "dozen_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_nl_numeral_stats() {
        // Generation statistics
        let total_rules = 16;
        let auto_generated = 5;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("nl/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

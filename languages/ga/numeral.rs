// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/GA/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: ga
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  




// Dictionary: oneToTen_dictionary
lazy_static! {
    static ref ONETOTEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("aon", 1);
        
        map.insert("ceithre", 4);
        
        map.insert("cuig", 5);
        
        map.insert("cúig", 5);
        
        map.insert("deich", 10);
        
        map.insert("dha", 2);
        
        map.insert("dhá", 2);
        
        map.insert("naoi", 9);
        
        map.insert("ocht", 8);
        
        map.insert("se", 6);
        
        map.insert("seacht", 7);
        
        map.insert("sé", 6);
        
        map.insert("tri", 3);
        
        map.insert("trí", 3);
        
        map
    };
}

// Dictionary: oldVigNumeralsS_dictionary
lazy_static! {
    static ref OLDVIGNUMERALSS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ceithre fichid", 80);
        
        map.insert("da fhichead", 40);
        
        map.insert("dha fhichead", 40);
        
        map.insert("dhá fhichead", 40);
        
        map.insert("dá fhichead", 40);
        
        map.insert("tri fichid", 60);
        
        map.insert("trí fichid", 60);
        
        map
    };
}

// Dictionary: twentyToNinety_dictionary
lazy_static! {
    static ref TWENTYTONINETY_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("caoga", 50);
        
        map.insert("daichead", 40);
        
        map.insert("fiche", 20);
        
        map.insert("nocha", 90);
        
        map.insert("nócha", 90);
        
        map.insert("ochto", 80);
        
        map.insert("ochtó", 80);
        
        map.insert("seachto", 70);
        
        map.insert("seachtó", 70);
        
        map.insert("seasca", 60);
        
        map.insert("triocha", 30);
        
        map.insert("tríocha", 30);
        
        map
    };
}

// Dictionary: countNumerals_dictionary
lazy_static! {
    static ref COUNTNUMERALS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ceathair", 4);
        
        map.insert("cuig", 5);
        
        map.insert("cúig", 5);
        
        map.insert("deich", 10);
        
        map.insert("do", 2);
        
        map.insert("dó", 2);
        
        map.insert("haon", 1);
        
        map.insert("hocht", 8);
        
        map.insert("naid", 0);
        
        map.insert("naoi", 9);
        
        map.insert("náid", 0);
        
        map.insert("se", 6);
        
        map.insert("seacht", 7);
        
        map.insert("sé", 6);
        
        map.insert("tri", 3);
        
        map.insert("trí", 3);
        
        map
    };
}


/// Build Numeral rules for ga locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 6 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: oneToTen_dictionary
    // Examples: aon, dha, dhá, trí, tri
    {
        let dict = &*ONETOTEN_DICTIONARY;
        b.rule_1_terminal(
            "ga:oneToTen_dictionary",
            b.reg(r"(?i)aon|ceithre|cuig|cúig|deich|dha|dhá|naoi|ocht|se|seacht|sé|tri|trí").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: oldVigNumeralsS_dictionary
    // Examples: dá fhichead, da fhichead, dhá fhichead, dha fhichead, trí fichid
    {
        let dict = &*OLDVIGNUMERALSS_DICTIONARY;
        b.rule_1_terminal(
            "ga:oldVigNumeralsS_dictionary",
            b.reg(r"(?i)ceithre fichid|da fhichead|dha fhichead|dhá fhichead|dá fhichead|tri fichid|trí fichid").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyToNinety_dictionary
    // Examples: fiche, triocha, tríocha, daichead, caoga
    {
        let dict = &*TWENTYTONINETY_DICTIONARY;
        b.rule_1_terminal(
            "ga:twentyToNinety_dictionary",
            b.reg(r"(?i)caoga|daichead|fiche|nocha|nócha|ochto|ochtó|seachto|seachtó|seasca|triocha|tríocha").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: countNumerals_dictionary
    // Examples: naid, náid, haon, dó, do
    {
        let dict = &*COUNTNUMERALS_DICTIONARY;
        b.rule_1_terminal(
            "ga:countNumerals_dictionary",
            b.reg(r"(?i)ceathair|cuig|cúig|deich|do|dó|haon|hocht|naid|naoi|náid|se|seacht|sé|tri|trí").unwrap(),
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
    

    // Rule: Numerals2 (refs: oneToTenMap)
    {
        
        
        let dict = &*ONETOTEN_DICTIONARY;
        b.rule_1_terminal(
            "ga:Numerals2",
            b.reg(r"(?i)(aon|dh(á|a)|tr(í|i)|ceithre|c(ú|u)ig|seacht|s(é|e)|ocht|naoi|deich)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: OldVigesimalNumeralsS (refs: oldVigNumeralsSMap)
    {
        
        
        let dict = &*OLDVIGNUMERALSS_DICTIONARY;
        b.rule_1_terminal(
            "ga:OldVigesimalNumeralsS",
            b.reg(r"(?i)(d[ée]ag )?is (dh?(á|a) fhichead|tr(í|i) fichid|ceithre fichid)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Numerals (refs: twentyToNinetyMap)
    {
        
        
        let dict = &*TWENTYTONINETY_DICTIONARY;
        b.rule_1_terminal(
            "ga:Numerals",
            b.reg(r"(?i)(fiche|tr(í|i)ocha|daichead|caoga|seasca|seacht(ó|o)|ocht(ó|o)|n(ó|o)cha)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: CountNumerals (refs: countNumeralsMap)
    {
        
        
        let dict = &*COUNTNUMERALS_DICTIONARY;
        b.rule_1_terminal(
            "ga:CountNumerals",
            b.reg(r"(?i)a (n(á|a)id|haon|d(ó|o)|tr(í|i)|ceathair|c(ú|u)ig|s(é|e)|seacht|hocht|naoi|deich)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (6)
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
    

    // TODO: Dag (regex)
    
    //   Original: déag
    
    // Manual implementation required
    

    // TODO: OldVigesimalFiche (regex)
    
    //   Original: old vigesimal 20 + 10
    
    // Manual implementation required
    

    // TODO: Amhin (regex)
    
    //   Original: amháin
    
    // Manual implementation required
    

    
    eprintln!("⚠️  ga/numeral has 6 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_ga_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_ga_numeral_dictionaries() {
        
        assert!(ONETOTEN_DICTIONARY.len() > 0, "oneToTen_dictionary should not be empty");
        
        assert!(OLDVIGNUMERALSS_DICTIONARY.len() > 0, "oldVigNumeralsS_dictionary should not be empty");
        
        assert!(TWENTYTONINETY_DICTIONARY.len() > 0, "twentyToNinety_dictionary should not be empty");
        
        assert!(COUNTNUMERALS_DICTIONARY.len() > 0, "countNumerals_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_ga_numeral_stats() {
        // Generation statistics
        let total_rules = 14;
        let auto_generated = 8;
        let manual_needed = 6;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("ga/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

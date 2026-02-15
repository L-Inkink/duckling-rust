// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/NB/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: nb
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroToNineteen_dictionary
lazy_static! {
    static ref ZEROTONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("atten", 18);
        
        map.insert("elleve", 11);
        
        map.insert("en", 1);
        
        map.insert("ett", 1);
        
        map.insert("fem", 5);
        
        map.insert("femten", 15);
        
        map.insert("fire", 4);
        
        map.insert("fjorten", 14);
        
        map.insert("ingen", 0);
        
        map.insert("intet", 0);
        
        map.insert("ni", 9);
        
        map.insert("nitten", 19);
        
        map.insert("null", 0);
        
        map.insert("seks", 6);
        
        map.insert("seksten", 16);
        
        map.insert("sju", 7);
        
        map.insert("sytten", 17);
        
        map.insert("syv", 7);
        
        map.insert("søtten", 17);
        
        map.insert("ti", 10);
        
        map.insert("to", 2);
        
        map.insert("tolv", 12);
        
        map.insert("tre", 3);
        
        map.insert("tretten", 13);
        
        map.insert("åtte", 8);
        
        map.insert("én", 1);
        
        map
    };
}

// Dictionary: twentyToHundred_dictionary
lazy_static! {
    static ref TWENTYTOHUNDRED_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("femtien", 51);
        
        map.insert("femtifem", 55);
        
        map.insert("femtifire", 54);
        
        map.insert("femtini", 59);
        
        map.insert("femtiseks", 56);
        
        map.insert("femtisju", 57);
        
        map.insert("femtisyv", 57);
        
        map.insert("femtito", 52);
        
        map.insert("femtitre", 53);
        
        map.insert("femtiåtte", 58);
        
        map.insert("femtién", 51);
        
        map.insert("førtien", 41);
        
        map.insert("førtifem", 45);
        
        map.insert("førtifire", 44);
        
        map.insert("førtini", 49);
        
        map.insert("førtiseks", 46);
        
        map.insert("førtisju", 47);
        
        map.insert("førtisyv", 47);
        
        map.insert("førtito", 42);
        
        map.insert("førtitre", 43);
        
        map.insert("førtiåtte", 48);
        
        map.insert("førtién", 41);
        
        map.insert("nittien", 91);
        
        map.insert("nittifem", 95);
        
        map.insert("nittifire", 94);
        
        map.insert("nittini", 99);
        
        map.insert("nittiseks", 96);
        
        map.insert("nittisju", 97);
        
        map.insert("nittisyv", 97);
        
        map.insert("nittito", 92);
        
        map.insert("nittitre", 93);
        
        map.insert("nittiåtte", 98);
        
        map.insert("nittién", 91);
        
        map.insert("sekstien", 61);
        
        map.insert("sekstifem", 65);
        
        map.insert("sekstifire", 64);
        
        map.insert("sekstini", 69);
        
        map.insert("sekstiseks", 66);
        
        map.insert("sekstisju", 67);
        
        map.insert("sekstisyv", 67);
        
        map.insert("sekstito", 62);
        
        map.insert("sekstitre", 63);
        
        map.insert("sekstiåtte", 68);
        
        map.insert("sekstién", 61);
        
        map.insert("syttien", 71);
        
        map.insert("syttifem", 75);
        
        map.insert("syttifire", 74);
        
        map.insert("syttini", 79);
        
        map.insert("syttiseks", 76);
        
        map.insert("syttisju", 77);
        
        map.insert("syttisyv", 77);
        
        map.insert("syttito", 72);
        
        map.insert("syttitre", 73);
        
        map.insert("syttiåtte", 78);
        
        map.insert("syttién", 71);
        
        map.insert("søttien", 71);
        
        map.insert("søttifem", 75);
        
        map.insert("søttifire", 74);
        
        map.insert("søttini", 79);
        
        map.insert("søttiseks", 76);
        
        map.insert("søttisju", 77);
        
        map.insert("søttisyv", 77);
        
        map.insert("søttito", 72);
        
        map.insert("søttitre", 73);
        
        map.insert("søttiåtte", 78);
        
        map.insert("søttién", 71);
        
        map.insert("tjueen", 21);
        
        map.insert("tjuefem", 25);
        
        map.insert("tjuefire", 24);
        
        map.insert("tjueni", 29);
        
        map.insert("tjueseks", 26);
        
        map.insert("tjuesju", 27);
        
        map.insert("tjuesyv", 27);
        
        map.insert("tjueto", 22);
        
        map.insert("tjuetre", 23);
        
        map.insert("tjueåtte", 28);
        
        map.insert("tjueén", 21);
        
        map.insert("trettien", 31);
        
        map.insert("trettifem", 35);
        
        map.insert("trettifire", 34);
        
        map.insert("trettini", 39);
        
        map.insert("trettiseks", 36);
        
        map.insert("trettisju", 37);
        
        map.insert("trettisyv", 37);
        
        map.insert("trettito", 32);
        
        map.insert("trettitre", 33);
        
        map.insert("trettiåtte", 38);
        
        map.insert("trettién", 31);
        
        map.insert("åttien", 81);
        
        map.insert("åttifem", 85);
        
        map.insert("åttifire", 84);
        
        map.insert("åttini", 89);
        
        map.insert("åttiseks", 86);
        
        map.insert("åttisju", 87);
        
        map.insert("åttisyv", 87);
        
        map.insert("åttito", 82);
        
        map.insert("åttitre", 83);
        
        map.insert("åttiåtte", 88);
        
        map.insert("åttién", 81);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("femti", 50);
        
        map.insert("førti", 40);
        
        map.insert("nitti", 90);
        
        map.insert("seksti", 60);
        
        map.insert("sytti", 70);
        
        map.insert("søtti", 70);
        
        map.insert("tjue", 20);
        
        map.insert("tredve", 30);
        
        map.insert("tretti", 30);
        
        map.insert("tyve", 20);
        
        map.insert("åtti", 80);
        
        map
    };
}


/// Build Numeral rules for nb locale
///
/// Auto-generated rules:
///   - 3 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 11 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (3)
    // ========================================
    

    // Rule: zeroToNineteen_dictionary
    // Examples: null, ingen, intet, en, ett
    {
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "nb:zeroToNineteen_dictionary",
            b.reg(r"(?i)atten|elleve|en|ett|fem|femten|fire|fjorten|ingen|intet|ni|nitten|null|seks|seksten|sju|sytten|syv|søtten|ti|to|tolv|tre|tretten|åtte|én").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyToHundred_dictionary
    // Examples: tjueen, tjueén, tjueto, tjuetre, tjuefire
    {
        let dict = &*TWENTYTOHUNDRED_DICTIONARY;
        b.rule_1_terminal(
            "nb:twentyToHundred_dictionary",
            b.reg(r"(?i)femtien|femtifem|femtifire|femtini|femtiseks|femtisju|femtisyv|femtito|femtitre|femtiåtte|femtién|førtien|førtifem|førtifire|førtini|førtiseks|førtisju|førtisyv|førtito|førtitre|førtiåtte|førtién|nittien|nittifem|nittifire|nittini|nittiseks|nittisju|nittisyv|nittito|nittitre|nittiåtte|nittién|sekstien|sekstifem|sekstifire|sekstini|sekstiseks|sekstisju|sekstisyv|sekstito|sekstitre|sekstiåtte|sekstién|syttien|syttifem|syttifire|syttini|syttiseks|syttisju|syttisyv|syttito|syttitre|syttiåtte|syttién|søttien|søttifem|søttifire|søttini|søttiseks|søttisju|søttisyv|søttito|søttitre|søttiåtte|søttién|tjueen|tjuefem|tjuefire|tjueni|tjueseks|tjuesju|tjuesyv|tjueto|tjuetre|tjueåtte|tjueén|trettien|trettifem|trettifire|trettini|trettiseks|trettisju|trettisyv|trettito|trettitre|trettiåtte|trettién|åttien|åttifem|åttifire|åttini|åttiseks|åttisju|åttisyv|åttito|åttitre|åttiåtte|åttién").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: tyve, tjue, tredve, tretti, førti
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "nb:tens_dictionary",
            b.reg(r"(?i)femti|førti|nitti|seksti|sytti|søtti|tjue|tredve|tretti|tyve|åtti").unwrap(),
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
    

    // Rule: Integer (refs: zeroToNineteenMap)
    {
        
        
        let dict = &*ZEROTONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "nb:Integer",
            b.reg(r"(?i)(intet|ingen|null|en|ett|én|to|tretten|tre|fire|femten|fem|seksten|seks|syv|sju|åtte|nitten|ni|ti|elleve|tolv|fjorten|sytten|søtten|atten)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer4 (refs: twentyToHundredMap)
    {
        
        
        let dict = &*TWENTYTOHUNDRED_DICTIONARY;
        b.rule_1_terminal(
            "nb:Integer4",
            b.reg(r"(?i)(tjueen|tjueén|tjueto|tjuetre|tjuefire|tjuefem|tjueseks|tjuesju|tjuesyv|tjueåtte|tjueni            |trettien|trettién|trettito|trettitre|trettifire|trettifem|trettiseks|trettisju|trettisyv|trettiåtte|trettini            |førtien|førtién|førtito|førtitre|førtifire|førtifem|førtiseks|førtisju|førtisyv|førtiåtte|førtini            |femtien|femtién|femtito|femtitre|femtifire|femtifem|femtiseks|femtisju|femtisyv|femtiåtte|femtini            |sekstien|sekstién|sekstito|sekstitre|sekstifire|sekstifem|sekstiseks|sekstisju|sekstisyv|sekstiåtte|sekstini            |syttien|syttién|syttito|syttitre|syttifire|syttifem|syttiseks|syttisju|syttisyv|syttiåtte|syttini            |søttien|søttién|søttito|søttitre|søttifire|søttifem|søttiseks|søttisju|søttisyv|søttiåtte|søttini            |åttien|åttién|åttito|åttitre|åttifire|åttifem|åttiseks|åttisju|åttisyv|åttiåtte|åttini            |nittien|nittién|nittito|nittitre|nittifire|nittifem|nittiseks|nittisju|nittisyv|nittiåtte|nittini            )").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer2 (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "nb:Integer2",
            b.reg(r"(?i)(tyve|tjue|tredve|tretti|førti|femti|seksti|sytti|søtti|åtti|nitti)").unwrap(),
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
    

    // TODO: DecimalWithThousandsSeparator (regex)
    
    //   Original: decimal with thousands separator
    
    // Manual implementation required
    

    // TODO: DecimalNumeral (regex)
    
    //   Original: decimal number
    
    // Manual implementation required
    

    // TODO: Single (regex)
    
    //   Original: single
    
    // Manual implementation required
    

    // TODO: PowersOfTen (regex)
    
    //   Original: powers of tens
    
    // Manual implementation required
    

    // TODO: APair (regex)
    
    //   Original: a pair
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Integer3 (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    
    eprintln!("⚠️  nb/numeral has 11 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_nb_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_nb_numeral_dictionaries() {
        
        assert!(ZEROTONINETEEN_DICTIONARY.len() > 0, "zeroToNineteen_dictionary should not be empty");
        
        assert!(TWENTYTOHUNDRED_DICTIONARY.len() > 0, "twentyToHundred_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_nb_numeral_stats() {
        // Generation statistics
        let total_rules = 17;
        let auto_generated = 6;
        let manual_needed = 11;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("nb/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

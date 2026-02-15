// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/IT/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: it
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  




// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinquanta", 50);
        
        map.insert("novanta", 90);
        
        map.insert("ottanta", 80);
        
        map.insert("quaranta", 40);
        
        map.insert("sessanta", 60);
        
        map.insert("settanta", 70);
        
        map.insert("trenta", 30);
        
        map.insert("venti", 20);
        
        map
    };
}

// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinque", 5);
        
        map.insert("diciannove", 19);
        
        map.insert("diciassette", 17);
        
        map.insert("diciotto", 18);
        
        map.insert("dieci", 10);
        
        map.insert("dodici", 12);
        
        map.insert("due", 2);
        
        map.insert("niente", 0);
        
        map.insert("nove", 9);
        
        map.insert("nulla", 0);
        
        map.insert("otto", 8);
        
        map.insert("quattordici", 14);
        
        map.insert("quattro", 4);
        
        map.insert("quindici", 15);
        
        map.insert("sedici", 16);
        
        map.insert("sei", 6);
        
        map.insert("sette", 7);
        
        map.insert("tre", 3);
        
        map.insert("tredici", 13);
        
        map.insert("un", 1);
        
        map.insert("undici", 11);
        
        map.insert("uno", 1);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cento", 100);
        
        map.insert("cinquecento", 500);
        
        map.insert("duecento", 200);
        
        map.insert("mila", 1000);
        
        map.insert("mille", 1000);
        
        map.insert("novecento", 900);
        
        map.insert("ottocento", 800);
        
        map.insert("quattrocento", 400);
        
        map.insert("seicento", 600);
        
        map.insert("settecento", 700);
        
        map.insert("trecento", 300);
        
        map
    };
}

// Dictionary: twentyoneNinetynine_dictionary
lazy_static! {
    static ref TWENTYONENINETYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cinquantacinque", 55);
        
        map.insert("cinquantadue", 52);
        
        map.insert("cinquantanove", 59);
        
        map.insert("cinquantaquattro", 54);
        
        map.insert("cinquantasei", 56);
        
        map.insert("cinquantasette", 57);
        
        map.insert("cinquantatre", 53);
        
        map.insert("cinquantatré", 53);
        
        map.insert("cinquantotto", 58);
        
        map.insert("cinquantuno", 51);
        
        map.insert("novantacinque", 95);
        
        map.insert("novantadue", 92);
        
        map.insert("novantanove", 99);
        
        map.insert("novantaquattro", 94);
        
        map.insert("novantasei", 96);
        
        map.insert("novantasette", 97);
        
        map.insert("novantatre", 93);
        
        map.insert("novantatré", 93);
        
        map.insert("novantotto", 98);
        
        map.insert("novantuno", 91);
        
        map.insert("ottantacinque", 85);
        
        map.insert("ottantadue", 82);
        
        map.insert("ottantanove", 89);
        
        map.insert("ottantaquattro", 84);
        
        map.insert("ottantasei", 86);
        
        map.insert("ottantasette", 87);
        
        map.insert("ottantatre", 83);
        
        map.insert("ottantatré", 83);
        
        map.insert("ottantotto", 88);
        
        map.insert("ottantuno", 81);
        
        map.insert("quarantacinque", 45);
        
        map.insert("quarantadue", 42);
        
        map.insert("quarantanove", 49);
        
        map.insert("quarantaquattro", 44);
        
        map.insert("quarantasei", 46);
        
        map.insert("quarantasette", 47);
        
        map.insert("quarantatre", 43);
        
        map.insert("quarantatré", 43);
        
        map.insert("quarantotto", 48);
        
        map.insert("quarantuno", 41);
        
        map.insert("sessantacinque", 65);
        
        map.insert("sessantadue", 62);
        
        map.insert("sessantanove", 69);
        
        map.insert("sessantaquattro", 64);
        
        map.insert("sessantasei", 66);
        
        map.insert("sessantasette", 67);
        
        map.insert("sessantatre", 63);
        
        map.insert("sessantatré", 63);
        
        map.insert("sessantotto", 68);
        
        map.insert("sessantuno", 61);
        
        map.insert("settantacinque", 75);
        
        map.insert("settantadue", 72);
        
        map.insert("settantanove", 79);
        
        map.insert("settantaquattro", 74);
        
        map.insert("settantasei", 76);
        
        map.insert("settantasette", 77);
        
        map.insert("settantatre", 73);
        
        map.insert("settantatré", 73);
        
        map.insert("settantotto", 78);
        
        map.insert("settantuno", 71);
        
        map.insert("trentacinque", 35);
        
        map.insert("trentadue", 32);
        
        map.insert("trentanove", 39);
        
        map.insert("trentaquattro", 34);
        
        map.insert("trentasei", 36);
        
        map.insert("trentasette", 37);
        
        map.insert("trentatre", 33);
        
        map.insert("trentatré", 33);
        
        map.insert("trentotto", 38);
        
        map.insert("trentuno", 31);
        
        map.insert("venticinque", 25);
        
        map.insert("ventidue", 22);
        
        map.insert("ventinove", 29);
        
        map.insert("ventiquattro", 24);
        
        map.insert("ventisei", 26);
        
        map.insert("ventisette", 27);
        
        map.insert("ventitre", 23);
        
        map.insert("ventitré", 23);
        
        map.insert("ventotto", 28);
        
        map.insert("ventuno", 21);
        
        map
    };
}


/// Build Numeral rules for it locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 5 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: tens_dictionary
    // Examples: venti, trenta, quaranta, cinquanta, sessanta
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "it:tens_dictionary",
            b.reg(r"(?i)cinquanta|novanta|ottanta|quaranta|sessanta|settanta|trenta|venti").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: zeroNineteen_dictionary
    // Examples: zero, niente, nulla, un, uno
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "it:zeroNineteen_dictionary",
            b.reg(r"(?i)cinque|diciannove|diciassette|diciotto|dieci|dodici|due|niente|nove|nulla|otto|quattordici|quattro|quindici|sedici|sei|sette|tre|tredici|un|undici|uno|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundreds_dictionary
    // Examples: cento, duecento, trecento, quattrocento, cinquecento
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "it:hundreds_dictionary",
            b.reg(r"(?i)cento|cinquecento|duecento|mila|mille|novecento|ottocento|quattrocento|seicento|settecento|trecento").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: twentyoneNinetynine_dictionary
    // Examples: ventuno, ventidue, ventitre, ventitré, ventiquattro
    {
        let dict = &*TWENTYONENINETYNINE_DICTIONARY;
        b.rule_1_terminal(
            "it:twentyoneNinetynine_dictionary",
            b.reg(r"(?i)cinquantacinque|cinquantadue|cinquantanove|cinquantaquattro|cinquantasei|cinquantasette|cinquantatre|cinquantatré|cinquantotto|cinquantuno|novantacinque|novantadue|novantanove|novantaquattro|novantasei|novantasette|novantatre|novantatré|novantotto|novantuno|ottantacinque|ottantadue|ottantanove|ottantaquattro|ottantasei|ottantasette|ottantatre|ottantatré|ottantotto|ottantuno|quarantacinque|quarantadue|quarantanove|quarantaquattro|quarantasei|quarantasette|quarantatre|quarantatré|quarantotto|quarantuno|sessantacinque|sessantadue|sessantanove|sessantaquattro|sessantasei|sessantasette|sessantatre|sessantatré|sessantotto|sessantuno|settantacinque|settantadue|settantanove|settantaquattro|settantasei|settantasette|settantatre|settantatré|settantotto|settantuno|trentacinque|trentadue|trentanove|trentaquattro|trentasei|trentasette|trentatre|trentatré|trentotto|trentuno|venticinque|ventidue|ventinove|ventiquattro|ventisei|ventisette|ventitre|ventitré|ventotto|ventuno").unwrap(),
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
    

    // Rule: Numeral2 (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "it:Numeral2",
            b.reg(r"(?i)(venti|trenta|quaranta|cinquanta|sessanta|settanta|ottanta|novanta)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Numeral (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "it:Numeral",
            b.reg(r"(?i)(zero|nulla|niente|uno|due|tredici|tre|quattro|cinque|sei|sette|otto|nove|dieci|undici|dodici|quattordici|quindici|sedici|diciassette|diciotto|diciannove|un)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Numeral5 (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "it:Numeral5",
            b.reg(r"(?i)(due|tre|quattro|cinque|sei|sette|otto|nove)?cento|mil(a|le)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Numeral4 (refs: twentyoneNinetynineMap)
    {
        
        
        let dict = &*TWENTYONENINETYNINE_DICTIONARY;
        b.rule_1_terminal(
            "it:Numeral4",
            b.reg(r"(?i)((venti|trenta|quaranta|cinquanta|sessanta|settanta|ottanta|novanta)(due|tre|tré|quattro|cinque|sei|sette|nove))|((vent|trent|quarant|cinquant|sessant|settant|ottant|novant)(uno|otto))").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (5)
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
    

    // TODO: Numeral3 (composite)
    
    //   Original: number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)
    
    // Manual implementation required
    

    // TODO: Numerals (composite)
    
    //   Original: numbers 200..999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  it/numeral has 5 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_it_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_it_numeral_dictionaries() {
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
        assert!(TWENTYONENINETYNINE_DICTIONARY.len() > 0, "twentyoneNinetynine_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_it_numeral_stats() {
        // Generation statistics
        let total_rules = 13;
        let auto_generated = 8;
        let manual_needed = 5;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("it/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

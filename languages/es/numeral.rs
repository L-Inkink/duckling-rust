// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/ES/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: es
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroToFifteen_dictionary
lazy_static! {
    static ref ZEROTOFIFTEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("catorce", 14);
        
        map.insert("cero", 0);
        
        map.insert("cinco", 5);
        
        map.insert("cuatro", 4);
        
        map.insert("dies", 10);
        
        map.insert("diez", 10);
        
        map.insert("doce", 12);
        
        map.insert("dos", 2);
        
        map.insert("nueve", 9);
        
        map.insert("ocho", 8);
        
        map.insert("once", 11);
        
        map.insert("quince", 15);
        
        map.insert("seis", 6);
        
        map.insert("siete", 7);
        
        map.insert("séis", 6);
        
        map.insert("trece", 13);
        
        map.insert("tres", 3);
        
        map.insert("trés", 3);
        
        map.insert("un", 1);
        
        map.insert("una", 1);
        
        map.insert("uno", 1);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: sixteenToTwentyNine_dictionary
lazy_static! {
    static ref SIXTEENTOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("diecinueve", 19);
        
        map.insert("dieciocho", 18);
        
        map.insert("dieciseis", 16);
        
        map.insert("diecisiete", 17);
        
        map.insert("dieciséis", 16);
        
        map.insert("diesiseis", 16);
        
        map.insert("diesiséis", 16);
        
        map.insert("veinticinco", 25);
        
        map.insert("veinticuatro", 24);
        
        map.insert("veintidos", 22);
        
        map.insert("veintidós", 22);
        
        map.insert("veintinueve", 29);
        
        map.insert("veintiocho", 28);
        
        map.insert("veintiseis", 26);
        
        map.insert("veintisiete", 27);
        
        map.insert("veintiséis", 26);
        
        map.insert("veintitres", 23);
        
        map.insert("veintitrés", 23);
        
        map.insert("veintiuna", 21);
        
        map.insert("veintiuno", 21);
        
        map
    };
}

// Dictionary: byTens_dictionary
lazy_static! {
    static ref BYTENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cincuenta", 50);
        
        map.insert("cuarenta", 40);
        
        map.insert("noventa", 90);
        
        map.insert("ochenta", 80);
        
        map.insert("sesenta", 60);
        
        map.insert("setenta", 70);
        
        map.insert("treinta", 30);
        
        map.insert("veinte", 20);
        
        map
    };
}

// Dictionary: bigNumbers_dictionary
lazy_static! {
    static ref BIGNUMBERS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cien", 100);
        
        map.insert("ciento", 100);
        
        map.insert("cientos", 100);
        
        map.insert("cuatrocientos", 400);
        
        map.insert("doscientos", 200);
        
        map.insert("mil", 1000);
        
        map.insert("millon", 1000000);
        
        map.insert("millones", 1000000);
        
        map.insert("millón", 1000000);
        
        map.insert("novecientos", 900);
        
        map.insert("ochocientos", 800);
        
        map.insert("quinientos", 500);
        
        map.insert("seiscientos", 600);
        
        map.insert("setecientos", 700);
        
        map.insert("trescientos", 300);
        
        map.insert("un millon", 1000000);
        
        map.insert("un millón", 1000000);
        
        map
    };
}


/// Build Numeral rules for es locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 5 dictionary-reference regex rules
///   - 8 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroToFifteen_dictionary
    // Examples: zero, cero, un, una, uno
    {
        let dict = &*ZEROTOFIFTEEN_DICTIONARY;
        b.rule_1_terminal(
            "es:zeroToFifteen_dictionary",
            b.reg(r"(?i)catorce|cero|cinco|cuatro|dies|diez|doce|dos|nueve|ocho|once|quince|seis|siete|séis|trece|tres|trés|un|una|uno|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: sixteenToTwentyNine_dictionary
    // Examples: dieciseis, diesiséis, diesiseis, dieciséis, diecisiete
    {
        let dict = &*SIXTEENTOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "es:sixteenToTwentyNine_dictionary",
            b.reg(r"(?i)diecinueve|dieciocho|dieciseis|diecisiete|dieciséis|diesiseis|diesiséis|veinticinco|veinticuatro|veintidos|veintidós|veintinueve|veintiocho|veintiseis|veintisiete|veintiséis|veintitres|veintitrés|veintiuna|veintiuno").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: byTens_dictionary
    // Examples: veinte, treinta, cuarenta, cincuenta, sesenta
    {
        let dict = &*BYTENS_DICTIONARY;
        b.rule_1_terminal(
            "es:byTens_dictionary",
            b.reg(r"(?i)cincuenta|cuarenta|noventa|ochenta|sesenta|setenta|treinta|veinte").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: bigNumbers_dictionary
    // Examples: cien, cientos, ciento, doscientos, trescientos
    {
        let dict = &*BIGNUMBERS_DICTIONARY;
        b.rule_1_terminal(
            "es:bigNumbers_dictionary",
            b.reg(r"(?i)cien|ciento|cientos|cuatrocientos|doscientos|mil|millon|millones|millón|novecientos|ochocientos|quinientos|seiscientos|setecientos|trescientos|un millon|un millón").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (5)
    // ========================================
    

    // Rule: NumeralZeroToFifteen (refs: zeroToFifteenMap)
    {
        
        
        let dict = &*ZEROTOFIFTEEN_DICTIONARY;
        b.rule_1_terminal(
            "es:NumeralZeroToFifteen",
            b.reg(r"(?i)((c|z)ero|un(o|a)?|dos|tr(é|e)s|cuatro|cinco|s(e|é)is|siete|ocho|nueve|die(z|s)|once|doce|trece|catorce|quince)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: NumeralSixteenToTwentyNine (refs: sixteenToTwentyNineMap)
    {
        
        
        let dict = &*SIXTEENTOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "es:NumeralSixteenToTwentyNine",
            b.reg(r"(?i)(die(c|s)is(é|e)is|diecisiete|dieciocho|diecinueve|veintiun(o|a)|veintid(o|ó)s|veintitr(é|e)s|veinticuatro|veinticinco|veintis(é|e)is|veintisiete|veintiocho|veintinueve|treinta)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: NumeralTwentyToNinetyTens (refs: byTensMap)
    {
        
        
        let dict = &*BYTENS_DICTIONARY;
        b.rule_1_terminal(
            "es:NumeralTwentyToNinetyTens",
            b.reg(r"(?i)(veinte|treinta|cuarenta|cincuenta|sesenta|setenta|ochenta|noventa)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: BigNumeral (refs: bigNumbersMap)
    {
        
        
        let dict = &*BIGNUMBERS_DICTIONARY;
        b.rule_1_terminal(
            "es:BigNumeral",
            b.reg(r"(?i)(cien(to|tos)?|doscientos|trescientos|cuatrocientos|quinientos|seiscientos|setecientos|ochocientos|novecientos|(un )?mill(o|ó)n)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: BigNumeralMultipliable (refs: bigNumbersMap)
    {
        
        
        let dict = &*BIGNUMBERS_DICTIONARY;
        b.rule_1_terminal(
            "es:BigNumeralMultipliable",
            b.reg(r"(?i)(mil(lones)?)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (8)
    // ========================================
    

    // TODO: BelowTenWithTwoDigits (regex)
    
    //   Original: integer (0-9) with two digits
    
    // Manual implementation required
    

    // TODO: LeadingDotNumeral (regex)
    
    //   Original: dot number
    
    // Manual implementation required
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: NumeralTwentyOneToNinetyNine (composite)
    
    //   Original: number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)
    
    // Manual implementation required
    

    // TODO: NumeralHundredsAndSmaller (composite)
    
    //   Original: <hundreds> 0..99
    
    // Manual implementation required
    

    // TODO: NumeralMultiply (composite)
    
    //   Original: 2..999 <multipliable>
    
    // Manual implementation required
    

    // TODO: NumeralThousandsAnd (composite)
    
    //   Original: <thousands> 0..999
    
    // Manual implementation required
    

    // TODO: NumeralMillionsAnd (composite)
    
    //   Original: <millions> 0..999999
    
    // Manual implementation required
    

    
    eprintln!("⚠️  es/numeral has 8 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_es_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_es_numeral_dictionaries() {
        
        assert!(ZEROTOFIFTEEN_DICTIONARY.len() > 0, "zeroToFifteen_dictionary should not be empty");
        
        assert!(SIXTEENTOTWENTYNINE_DICTIONARY.len() > 0, "sixteenToTwentyNine_dictionary should not be empty");
        
        assert!(BYTENS_DICTIONARY.len() > 0, "byTens_dictionary should not be empty");
        
        assert!(BIGNUMBERS_DICTIONARY.len() > 0, "bigNumbers_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_es_numeral_stats() {
        // Generation statistics
        let total_rules = 17;
        let auto_generated = 9;
        let manual_needed = 8;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("es/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

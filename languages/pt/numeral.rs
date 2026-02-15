// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/PT/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: pt
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("catorze", 14);
        
        map.insert("cinco", 5);
        
        map.insert("dez", 10);
        
        map.insert("dezanove", 19);
        
        map.insert("dezasseis", 16);
        
        map.insert("dezassete", 17);
        
        map.insert("dezenove", 19);
        
        map.insert("dezesseis", 16);
        
        map.insert("dezessete", 17);
        
        map.insert("dezoito", 18);
        
        map.insert("dois", 2);
        
        map.insert("doze", 12);
        
        map.insert("duas", 2);
        
        map.insert("nove", 9);
        
        map.insert("oito", 8);
        
        map.insert("onze", 11);
        
        map.insert("quatorze", 14);
        
        map.insert("quatro", 4);
        
        map.insert("quinze", 15);
        
        map.insert("seis", 6);
        
        map.insert("sete", 7);
        
        map.insert("tres", 3);
        
        map.insert("treze", 13);
        
        map.insert("três", 3);
        
        map.insert("um", 1);
        
        map.insert("uma", 1);
        
        map.insert("zero", 0);
        
        map
    };
}

// Dictionary: informal_dictionary
lazy_static! {
    static ref INFORMAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("par", 2);
        
        map.insert("par de", 2);
        
        map.insert("pares", 2);
        
        map.insert("pares de", 2);
        
        map.insert("pouco", 3);
        
        map.insert("um par", 2);
        
        map.insert("um par de", 2);
        
        map.insert("um pouco", 3);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cincoenta", 50);
        
        map.insert("cinquenta", 50);
        
        map.insert("cinqüenta", 50);
        
        map.insert("noventa", 90);
        
        map.insert("oitenta", 80);
        
        map.insert("quarenta", 40);
        
        map.insert("sessenta", 60);
        
        map.insert("setenta", 70);
        
        map.insert("trinta", 30);
        
        map.insert("vinte", 20);
        
        map
    };
}

// Dictionary: cents_dictionary
lazy_static! {
    static ref CENTS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("cem", 100);
        
        map.insert("cento", 100);
        
        map.insert("duzentos", 200);
        
        map.insert("novecentos", 900);
        
        map.insert("oitocentos", 800);
        
        map.insert("quatrocentos", 400);
        
        map.insert("quinhetos", 500);
        
        map.insert("seiscentos", 600);
        
        map.insert("setecentos", 700);
        
        map.insert("trezentos", 300);
        
        map
    };
}


/// Build Numeral rules for pt locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 3 dictionary-reference regex rules
///   - 13 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: zero, um, uma, dois, duas
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "pt:zeroNineteen_dictionary",
            b.reg(r"(?i)catorze|cinco|dez|dezanove|dezasseis|dezassete|dezenove|dezesseis|dezessete|dezoito|dois|doze|duas|nove|oito|onze|quatorze|quatro|quinze|seis|sete|tres|treze|três|um|uma|zero").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: informal_dictionary
    // Examples: um par, um par de, par, pares, par de
    {
        let dict = &*INFORMAL_DICTIONARY;
        b.rule_1_terminal(
            "pt:informal_dictionary",
            b.reg(r"(?i)par|par de|pares|pares de|pouco|um par|um par de|um pouco").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: vinte, trinta, quarenta, cincoenta, cinquenta
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "pt:tens_dictionary",
            b.reg(r"(?i)cincoenta|cinquenta|cinqüenta|noventa|oitenta|quarenta|sessenta|setenta|trinta|vinte").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: cents_dictionary
    // Examples: cem, cento, duzentos, trezentos, quatrocentos
    {
        let dict = &*CENTS_DICTIONARY;
        b.rule_1_terminal(
            "pt:cents_dictionary",
            b.reg(r"(?i)cem|cento|duzentos|novecentos|oitocentos|quatrocentos|quinhetos|seiscentos|setecentos|trezentos").unwrap(),
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
            "pt:ToNineteen",
            b.reg(r"(?i)(zero|d(oi|ua)s|(uma? )?par(es)?( de)?|tr(e|ê)s|(um )?pouco|uma?|(c|qu)atorze|quatro|quinze|cinco|dez[ea]sseis|seis|dez[ea]ssete|sete|dezoito|oito|dez[ea]nove|nove|dez|onze|doze|treze)").unwrap(),
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
            "pt:Tens",
            b.reg(r"(?i)(vinte|trinta|quarenta|cin(co|q[uü])enta|sessenta|setenta|oitenta|noventa)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Cent (refs: centsMap)
    {
        
        
        let dict = &*CENTS_DICTIONARY;
        b.rule_1_terminal(
            "pt:Cent",
            b.reg(r"(?i)(cem|cento|duzentos|trezentos|quatrocentos|quinhetos|seiscentos|setecentos|oitocentos|novecentos)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (13)
    // ========================================
    

    // TODO: Dozen (regex)
    
    //   Original: a dozen of
    
    // Manual implementation required
    

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
    
    //   Original: dot-separated numbers
    
    // Manual implementation required
    

    // TODO: Negative (regex)
    
    //   Original: negative numbers
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: DecsAnd (composite)
    
    //   Original: number (21..29 31..39 .. 91..99)
    
    // Manual implementation required
    

    // TODO: CompositeCents (composite)
    
    //   Original: integer 101..999
    
    // Manual implementation required
    

    // TODO: CentsAnd (composite)
    
    //   Original: number (101..199 201..299 .. 901..999)
    
    // Manual implementation required
    

    // TODO: SkipHundreds (composite)
    
    //   Original: one twenty two
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  pt/numeral has 13 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_pt_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_pt_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(INFORMAL_DICTIONARY.len() > 0, "informal_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(CENTS_DICTIONARY.len() > 0, "cents_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_pt_numeral_stats() {
        // Generation statistics
        let total_rules = 20;
        let auto_generated = 7;
        let manual_needed = 13;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("pt/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

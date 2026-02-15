// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/TH/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: th
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: zeroNineteen_dictionary
lazy_static! {
    static ref ZERONINETEEN_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ศูนย์", 0);
        
        map.insert("สอง", 2);
        
        map.insert("สาม", 3);
        
        map.insert("สิบ", 10);
        
        map.insert("สิบสอง", 12);
        
        map.insert("สิบสาม", 13);
        
        map.insert("สิบสี่", 14);
        
        map.insert("สิบหก", 16);
        
        map.insert("สิบหนึ่ง", 11);
        
        map.insert("สิบห้า", 15);
        
        map.insert("สิบเก้า", 19);
        
        map.insert("สิบเจ็ด", 17);
        
        map.insert("สิบเอ็ด", 11);
        
        map.insert("สิบแปด", 18);
        
        map.insert("สี่", 4);
        
        map.insert("หก", 6);
        
        map.insert("หนึ่ง", 1);
        
        map.insert("ห้า", 5);
        
        map.insert("เก้า", 9);
        
        map.insert("เจ็ด", 7);
        
        map.insert("เอ็ด", 1);
        
        map.insert("แปด", 8);
        
        map.insert("ไม่มี", 0);
        
        map
    };
}

// Dictionary: informal_dictionary
lazy_static! {
    static ref INFORMAL_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("คู่ของ", 2);
        
        map.insert("คู่นึง", 2);
        
        map.insert("อันนึง", 1);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ยี่สิบ", 20);
        
        map.insert("สามสิบ", 30);
        
        map.insert("สี่สิบ", 40);
        
        map.insert("หกสิบ", 60);
        
        map.insert("ห้าสิบ", 50);
        
        map.insert("เก้าสิบ", 90);
        
        map.insert("เจ็ดสิบ", 70);
        
        map.insert("แปดสิบ", 80);
        
        map
    };
}

// Dictionary: digitsHundredTwentyToTwentyNine_dictionary
lazy_static! {
    static ref DIGITSHUNDREDTWENTYTOTWENTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("ร้อยยี่สิบ", 120);
        
        map.insert("ร้อยยี่สิบสอง", 122);
        
        map.insert("ร้อยยี่สิบสาม", 123);
        
        map.insert("ร้อยยี่สิบสี่", 124);
        
        map.insert("ร้อยยี่สิบหก", 126);
        
        map.insert("ร้อยยี่สิบหนึ่ง", 121);
        
        map.insert("ร้อยยี่สิบห้า", 125);
        
        map.insert("ร้อยยี่สิบเก้า", 129);
        
        map.insert("ร้อยยี่สิบเจ็ด", 127);
        
        map.insert("ร้อยยี่สิบเอ็ด", 121);
        
        map.insert("ร้อยยี่สิบแปด", 128);
        
        map.insert("สองร้อยยี่สิบ", 220);
        
        map.insert("สองร้อยยี่สิบสอง", 222);
        
        map.insert("สองร้อยยี่สิบสาม", 223);
        
        map.insert("สองร้อยยี่สิบสี่", 224);
        
        map.insert("สองร้อยยี่สิบหก", 226);
        
        map.insert("สองร้อยยี่สิบหนึ่ง", 221);
        
        map.insert("สองร้อยยี่สิบห้า", 225);
        
        map.insert("สองร้อยยี่สิบเก้า", 229);
        
        map.insert("สองร้อยยี่สิบเจ็ด", 227);
        
        map.insert("สองร้อยยี่สิบเอ็ด", 221);
        
        map.insert("สองร้อยยี่สิบแปด", 228);
        
        map.insert("สามร้อยยี่สิบ", 320);
        
        map.insert("สามร้อยยี่สิบสอง", 322);
        
        map.insert("สามร้อยยี่สิบสาม", 323);
        
        map.insert("สามร้อยยี่สิบสี่", 324);
        
        map.insert("สามร้อยยี่สิบหก", 326);
        
        map.insert("สามร้อยยี่สิบหนึ่ง", 321);
        
        map.insert("สามร้อยยี่สิบห้า", 325);
        
        map.insert("สามร้อยยี่สิบเก้า", 329);
        
        map.insert("สามร้อยยี่สิบเจ็ด", 327);
        
        map.insert("สามร้อยยี่สิบเอ็ด", 321);
        
        map.insert("สามร้อยยี่สิบแปด", 328);
        
        map.insert("สี่ร้อยยี่สิบ", 420);
        
        map.insert("สี่ร้อยยี่สิบสอง", 422);
        
        map.insert("สี่ร้อยยี่สิบสาม", 423);
        
        map.insert("สี่ร้อยยี่สิบสี่", 424);
        
        map.insert("สี่ร้อยยี่สิบหก", 426);
        
        map.insert("สี่ร้อยยี่สิบหนึ่ง", 421);
        
        map.insert("สี่ร้อยยี่สิบห้า", 425);
        
        map.insert("สี่ร้อยยี่สิบเก้า", 429);
        
        map.insert("สี่ร้อยยี่สิบเจ็ด", 427);
        
        map.insert("สี่ร้อยยี่สิบเอ็ด", 421);
        
        map.insert("สี่ร้อยยี่สิบแปด", 428);
        
        map.insert("หกร้อยยี่สิบ", 620);
        
        map.insert("หกร้อยยี่สิบสอง", 622);
        
        map.insert("หกร้อยยี่สิบสาม", 623);
        
        map.insert("หกร้อยยี่สิบสี่", 624);
        
        map.insert("หกร้อยยี่สิบหก", 626);
        
        map.insert("หกร้อยยี่สิบหนึ่ง", 621);
        
        map.insert("หกร้อยยี่สิบห้า", 625);
        
        map.insert("หกร้อยยี่สิบเก้า", 629);
        
        map.insert("หกร้อยยี่สิบเจ็ด", 627);
        
        map.insert("หกร้อยยี่สิบเอ็ด", 621);
        
        map.insert("หกร้อยยี่สิบแปด", 628);
        
        map.insert("หนึ่งร้อยยี่สิบ", 120);
        
        map.insert("หนึ่งร้อยยี่สิบสอง", 122);
        
        map.insert("หนึ่งร้อยยี่สิบสาม", 123);
        
        map.insert("หนึ่งร้อยยี่สิบสี่", 124);
        
        map.insert("หนึ่งร้อยยี่สิบหก", 126);
        
        map.insert("หนึ่งร้อยยี่สิบหนึ่ง", 121);
        
        map.insert("หนึ่งร้อยยี่สิบห้า", 125);
        
        map.insert("หนึ่งร้อยยี่สิบเก้า", 129);
        
        map.insert("หนึ่งร้อยยี่สิบเจ็ด", 127);
        
        map.insert("หนึ่งร้อยยี่สิบเอ็ด", 121);
        
        map.insert("หนึ่งร้อยยี่สิบแปด", 128);
        
        map.insert("ห้าร้อยยี่สิบ", 520);
        
        map.insert("ห้าร้อยยี่สิบสอง", 522);
        
        map.insert("ห้าร้อยยี่สิบสาม", 523);
        
        map.insert("ห้าร้อยยี่สิบสี่", 524);
        
        map.insert("ห้าร้อยยี่สิบหก", 526);
        
        map.insert("ห้าร้อยยี่สิบหนึ่ง", 521);
        
        map.insert("ห้าร้อยยี่สิบห้า", 525);
        
        map.insert("ห้าร้อยยี่สิบเก้า", 529);
        
        map.insert("ห้าร้อยยี่สิบเจ็ด", 527);
        
        map.insert("ห้าร้อยยี่สิบเอ็ด", 521);
        
        map.insert("ห้าร้อยยี่สิบแปด", 528);
        
        map.insert("เก้าร้อยยี่สิบ", 920);
        
        map.insert("เก้าร้อยยี่สิบสอง", 922);
        
        map.insert("เก้าร้อยยี่สิบสาม", 923);
        
        map.insert("เก้าร้อยยี่สิบสี่", 924);
        
        map.insert("เก้าร้อยยี่สิบหก", 926);
        
        map.insert("เก้าร้อยยี่สิบหนึ่ง", 921);
        
        map.insert("เก้าร้อยยี่สิบห้า", 925);
        
        map.insert("เก้าร้อยยี่สิบเก้า", 929);
        
        map.insert("เก้าร้อยยี่สิบเจ็ด", 927);
        
        map.insert("เก้าร้อยยี่สิบเอ็ด", 921);
        
        map.insert("เก้าร้อยยี่สิบแปด", 928);
        
        map.insert("เจ็ดร้อยยี่สิบ", 720);
        
        map.insert("เจ็ดร้อยยี่สิบสอง", 722);
        
        map.insert("เจ็ดร้อยยี่สิบสาม", 723);
        
        map.insert("เจ็ดร้อยยี่สิบสี่", 724);
        
        map.insert("เจ็ดร้อยยี่สิบหก", 726);
        
        map.insert("เจ็ดร้อยยี่สิบหนึ่ง", 721);
        
        map.insert("เจ็ดร้อยยี่สิบห้า", 725);
        
        map.insert("เจ็ดร้อยยี่สิบเก้า", 729);
        
        map.insert("เจ็ดร้อยยี่สิบเจ็ด", 727);
        
        map.insert("เจ็ดร้อยยี่สิบเอ็ด", 721);
        
        map.insert("เจ็ดร้อยยี่สิบแปด", 728);
        
        map.insert("แปดร้อยยี่สิบ", 820);
        
        map.insert("แปดร้อยยี่สิบสอง", 822);
        
        map.insert("แปดร้อยยี่สิบสาม", 823);
        
        map.insert("แปดร้อยยี่สิบสี่", 824);
        
        map.insert("แปดร้อยยี่สิบหก", 826);
        
        map.insert("แปดร้อยยี่สิบหนึ่ง", 821);
        
        map.insert("แปดร้อยยี่สิบห้า", 825);
        
        map.insert("แปดร้อยยี่สิบเก้า", 829);
        
        map.insert("แปดร้อยยี่สิบเจ็ด", 827);
        
        map.insert("แปดร้อยยี่สิบเอ็ด", 821);
        
        map.insert("แปดร้อยยี่สิบแปด", 828);
        
        map
    };
}


/// Build Numeral rules for th locale
///
/// Auto-generated rules:
///   - 4 dictionary rules
///   - 0 constant regex rules
///   - 4 dictionary-reference regex rules
///   - 10 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (4)
    // ========================================
    

    // Rule: zeroNineteen_dictionary
    // Examples: ไม่มี, ศูนย์, หนึ่ง, เอ็ด, สอง
    {
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "th:zeroNineteen_dictionary",
            b.reg(r"(?i)ศูนย์|สอง|สาม|สิบ|สิบสอง|สิบสาม|สิบสี่|สิบหก|สิบหนึ่ง|สิบห้า|สิบเก้า|สิบเจ็ด|สิบเอ็ด|สิบแปด|สี่|หก|หนึ่ง|ห้า|เก้า|เจ็ด|เอ็ด|แปด|ไม่มี").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: informal_dictionary
    // Examples: อันนึง, คู่นึง, คู่ของ
    {
        let dict = &*INFORMAL_DICTIONARY;
        b.rule_1_terminal(
            "th:informal_dictionary",
            b.reg(r"(?i)คู่ของ|คู่นึง|อันนึง").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: ยี่สิบ, สามสิบ, สี่สิบ, ห้าสิบ, หกสิบ
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "th:tens_dictionary",
            b.reg(r"(?i)ยี่สิบ|สามสิบ|สี่สิบ|หกสิบ|ห้าสิบ|เก้าสิบ|เจ็ดสิบ|แปดสิบ").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: digitsHundredTwentyToTwentyNine_dictionary
    // Examples: ร้อยยี่สิบ, ร้อยยี่สิบเอ็ด, ร้อยยี่สิบหนึ่ง, ร้อยยี่สิบสอง, ร้อยยี่สิบสาม
    {
        let dict = &*DIGITSHUNDREDTWENTYTOTWENTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "th:digitsHundredTwentyToTwentyNine_dictionary",
            b.reg(r"(?i)ร้อยยี่สิบ|ร้อยยี่สิบสอง|ร้อยยี่สิบสาม|ร้อยยี่สิบสี่|ร้อยยี่สิบหก|ร้อยยี่สิบหนึ่ง|ร้อยยี่สิบห้า|ร้อยยี่สิบเก้า|ร้อยยี่สิบเจ็ด|ร้อยยี่สิบเอ็ด|ร้อยยี่สิบแปด|สองร้อยยี่สิบ|สองร้อยยี่สิบสอง|สองร้อยยี่สิบสาม|สองร้อยยี่สิบสี่|สองร้อยยี่สิบหก|สองร้อยยี่สิบหนึ่ง|สองร้อยยี่สิบห้า|สองร้อยยี่สิบเก้า|สองร้อยยี่สิบเจ็ด|สองร้อยยี่สิบเอ็ด|สองร้อยยี่สิบแปด|สามร้อยยี่สิบ|สามร้อยยี่สิบสอง|สามร้อยยี่สิบสาม|สามร้อยยี่สิบสี่|สามร้อยยี่สิบหก|สามร้อยยี่สิบหนึ่ง|สามร้อยยี่สิบห้า|สามร้อยยี่สิบเก้า|สามร้อยยี่สิบเจ็ด|สามร้อยยี่สิบเอ็ด|สามร้อยยี่สิบแปด|สี่ร้อยยี่สิบ|สี่ร้อยยี่สิบสอง|สี่ร้อยยี่สิบสาม|สี่ร้อยยี่สิบสี่|สี่ร้อยยี่สิบหก|สี่ร้อยยี่สิบหนึ่ง|สี่ร้อยยี่สิบห้า|สี่ร้อยยี่สิบเก้า|สี่ร้อยยี่สิบเจ็ด|สี่ร้อยยี่สิบเอ็ด|สี่ร้อยยี่สิบแปด|หกร้อยยี่สิบ|หกร้อยยี่สิบสอง|หกร้อยยี่สิบสาม|หกร้อยยี่สิบสี่|หกร้อยยี่สิบหก|หกร้อยยี่สิบหนึ่ง|หกร้อยยี่สิบห้า|หกร้อยยี่สิบเก้า|หกร้อยยี่สิบเจ็ด|หกร้อยยี่สิบเอ็ด|หกร้อยยี่สิบแปด|หนึ่งร้อยยี่สิบ|หนึ่งร้อยยี่สิบสอง|หนึ่งร้อยยี่สิบสาม|หนึ่งร้อยยี่สิบสี่|หนึ่งร้อยยี่สิบหก|หนึ่งร้อยยี่สิบหนึ่ง|หนึ่งร้อยยี่สิบห้า|หนึ่งร้อยยี่สิบเก้า|หนึ่งร้อยยี่สิบเจ็ด|หนึ่งร้อยยี่สิบเอ็ด|หนึ่งร้อยยี่สิบแปด|ห้าร้อยยี่สิบ|ห้าร้อยยี่สิบสอง|ห้าร้อยยี่สิบสาม|ห้าร้อยยี่สิบสี่|ห้าร้อยยี่สิบหก|ห้าร้อยยี่สิบหนึ่ง|ห้าร้อยยี่สิบห้า|ห้าร้อยยี่สิบเก้า|ห้าร้อยยี่สิบเจ็ด|ห้าร้อยยี่สิบเอ็ด|ห้าร้อยยี่สิบแปด|เก้าร้อยยี่สิบ|เก้าร้อยยี่สิบสอง|เก้าร้อยยี่สิบสาม|เก้าร้อยยี่สิบสี่|เก้าร้อยยี่สิบหก|เก้าร้อยยี่สิบหนึ่ง|เก้าร้อยยี่สิบห้า|เก้าร้อยยี่สิบเก้า|เก้าร้อยยี่สิบเจ็ด|เก้าร้อยยี่สิบเอ็ด|เก้าร้อยยี่สิบแปด|เจ็ดร้อยยี่สิบ|เจ็ดร้อยยี่สิบสอง|เจ็ดร้อยยี่สิบสาม|เจ็ดร้อยยี่สิบสี่|เจ็ดร้อยยี่สิบหก|เจ็ดร้อยยี่สิบหนึ่ง|เจ็ดร้อยยี่สิบห้า|เจ็ดร้อยยี่สิบเก้า|เจ็ดร้อยยี่สิบเจ็ด|เจ็ดร้อยยี่สิบเอ็ด|เจ็ดร้อยยี่สิบแปด|แปดร้อยยี่สิบ|แปดร้อยยี่สิบสอง|แปดร้อยยี่สิบสาม|แปดร้อยยี่สิบสี่|แปดร้อยยี่สิบหก|แปดร้อยยี่สิบหนึ่ง|แปดร้อยยี่สิบห้า|แปดร้อยยี่สิบเก้า|แปดร้อยยี่สิบเจ็ด|แปดร้อยยี่สิบเอ็ด|แปดร้อยยี่สิบแปด").unwrap(),
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
    

    // Rule: ToNineteen (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "th:ToNineteen",
            b.reg(r"(?i)(ไม่มี|ศูนย์|สิบหนึ่ง|หนึ่ง|(คู่)s?( ของ)?|(คู่)s?( นึง)?|สิบเอ็ด|เอ็ด|สิบสอง|สิบสาม|สิบสี่|สิบห้า|สิบหก|สิบเจ็ด|สิบแปด|สิบเก้า|สอง|สาม|สี่|ห้า|หก|เจ็ด|แปด|เก้า|สิบ)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: SumTenDigits (refs: tensMap)
    {
        
        
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "th:SumTenDigits",
            b.reg(r"(?i)(ยี่สิบ|สามสิบ|สี่สิบ|ห้าสิบ|หกสิบ|เจ็ดสิบ|แปดสิบ|เก้าสิบ)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: SkipHundreds1 (refs: zeroNineteenMap)
    {
        
        
        let dict = &*ZERONINETEEN_DICTIONARY;
        b.rule_1_terminal(
            "th:SkipHundreds1",
            b.reg(r"(?i)(หนึ่ง|สอง|สาม|สี่|ห้า|หก|เจ็ด|แปด|เก้า)").unwrap(),
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
            "th:SkipHundreds2",
            b.reg(r"(?i)(หนึ่ง|สอง|สาม|สี่|ห้า|หก|เจ็ด|แปด|เก้า)").unwrap(),
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
    
    //   Original: comma-separated numbers
    
    // Manual implementation required
    

    // TODO: Negative (regex)
    
    //   Original: negative numbers
    
    // Manual implementation required
    

    // TODO: CompositeTens (composite)
    
    //   Original: integer 21..99
    
    // Manual implementation required
    

    // TODO: Sum (composite)
    
    //   Original: intersect 2 numbers
    
    // Manual implementation required
    

    // TODO: SumAnd (composite)
    
    //   Original: intersect 2 numbers (with and)
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    
    eprintln!("⚠️  th/numeral has 10 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_th_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_th_numeral_dictionaries() {
        
        assert!(ZERONINETEEN_DICTIONARY.len() > 0, "zeroNineteen_dictionary should not be empty");
        
        assert!(INFORMAL_DICTIONARY.len() > 0, "informal_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
        assert!(DIGITSHUNDREDTWENTYTOTWENTYNINE_DICTIONARY.len() > 0, "digitsHundredTwentyToTwentyNine_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_th_numeral_stats() {
        // Generation statistics
        let total_rules = 18;
        let auto_generated = 8;
        let manual_needed = 10;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("th/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

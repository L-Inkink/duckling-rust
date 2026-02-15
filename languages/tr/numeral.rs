// Auto-generated from /Users/link/Project/duckling/Duckling/Numeral/TR/Rules.hs
// DO NOT EDIT MANUALLY - Use tools/migration/codegen.rs
//
// Dimension: Numeral
// Locale: tr
// Generator: codegen v3 (improved automation)

use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};
use std::collections::HashMap;
use lazy_static::lazy_static;








  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
      
      
    
  

  
    
  

  
    
  

  
    
  




// Dictionary: hundreds_dictionary
lazy_static! {
    static ref HUNDREDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altıyüz", 600);
        
        map.insert("beşyüz", 500);
        
        map.insert("dokuzyüz", 900);
        
        map.insert("dörtyüz", 400);
        
        map.insert("ikiyüz", 200);
        
        map.insert("sekizyüz", 800);
        
        map.insert("yediyüz", 700);
        
        map.insert("yüz", 100);
        
        map.insert("üçyüz", 300);
        
        map
    };
}

// Dictionary: numeralSuffixesHalfsuffixText_dictionary
lazy_static! {
    static ref NUMERALSUFFIXESHALFSUFFIXTEXT_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altıbuçuk", 6.5);
        
        map.insert("be\351buçuk", 5.5);
        
        map.insert("bibuçuk", 1.5);
        
        map.insert("birbuçuk", 1.5);
        
        map.insert("dokuzbuçuk", 9.5);
        
        map.insert("dörtbuçuk", 4.5);
        
        map.insert("ikibuçuk", 2.5);
        
        map.insert("sekizbuçuk", 8.5);
        
        map.insert("yedibuçuk", 7.5);
        
        map.insert("ü\231buçuk", 3.5);
        
        map
    };
}

// Dictionary: tenToNintynine_dictionary
lazy_static! {
    static ref TENTONINTYNINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altmışaltı", 66);
        
        map.insert("altmışbeş", 65);
        
        map.insert("altmışbir", 61);
        
        map.insert("altmışdokuz", 69);
        
        map.insert("altmışdört", 64);
        
        map.insert("altmışiki", 62);
        
        map.insert("altmışsekiz", 68);
        
        map.insert("altmışyedi", 67);
        
        map.insert("altmışüç", 63);
        
        map.insert("atmışaltı", 66);
        
        map.insert("atmışbeş", 65);
        
        map.insert("atmışbir", 61);
        
        map.insert("atmışdokuz", 69);
        
        map.insert("atmışdört", 64);
        
        map.insert("atmışiki", 62);
        
        map.insert("atmışsekiz", 68);
        
        map.insert("atmışyedi", 67);
        
        map.insert("atmışüç", 63);
        
        map.insert("doksanaltı", 96);
        
        map.insert("doksanbeş", 95);
        
        map.insert("doksanbi", 91);
        
        map.insert("doksanbir", 91);
        
        map.insert("doksandokuz", 99);
        
        map.insert("doksandört", 94);
        
        map.insert("doksaniki", 92);
        
        map.insert("doksansekiz", 98);
        
        map.insert("doksanyedi", 97);
        
        map.insert("doksanüç", 93);
        
        map.insert("ellialtı", 56);
        
        map.insert("ellibeş", 55);
        
        map.insert("ellibi", 51);
        
        map.insert("ellibir", 51);
        
        map.insert("ellidokuz", 59);
        
        map.insert("ellidört", 54);
        
        map.insert("elliiki", 52);
        
        map.insert("ellisekiz", 58);
        
        map.insert("elliyedi", 57);
        
        map.insert("elliüç", 53);
        
        map.insert("kırkaltı", 46);
        
        map.insert("kırkbeş", 45);
        
        map.insert("kırkbi", 41);
        
        map.insert("kırkbir", 41);
        
        map.insert("kırkdokuz", 49);
        
        map.insert("kırkdört", 44);
        
        map.insert("kırkiki", 42);
        
        map.insert("kırksekiz", 48);
        
        map.insert("kırkyedi", 47);
        
        map.insert("kırküç", 43);
        
        map.insert("onaltı", 16);
        
        map.insert("onbeş", 15);
        
        map.insert("onbi", 11);
        
        map.insert("onbir", 11);
        
        map.insert("ondokuz", 19);
        
        map.insert("ondört", 14);
        
        map.insert("oniki", 12);
        
        map.insert("onsekiz", 18);
        
        map.insert("onyedi", 17);
        
        map.insert("onüç", 13);
        
        map.insert("otuzaltı", 36);
        
        map.insert("otuzbeş", 35);
        
        map.insert("otuzbi", 31);
        
        map.insert("otuzbir", 31);
        
        map.insert("otuzdokuz", 39);
        
        map.insert("otuzdört", 34);
        
        map.insert("otuziki", 32);
        
        map.insert("otuzsekiz", 38);
        
        map.insert("otuzyedi", 37);
        
        map.insert("otuzüç", 33);
        
        map.insert("seksenaltı", 86);
        
        map.insert("seksenbeş", 85);
        
        map.insert("seksenbi", 81);
        
        map.insert("seksenbir", 81);
        
        map.insert("seksendokuz", 89);
        
        map.insert("seksendört", 84);
        
        map.insert("sekseniki", 82);
        
        map.insert("seksensekiz", 88);
        
        map.insert("seksenyedi", 87);
        
        map.insert("seksenüç", 83);
        
        map.insert("yetmişaltı", 76);
        
        map.insert("yetmişbeş", 75);
        
        map.insert("yetmişbi", 71);
        
        map.insert("yetmişbir", 71);
        
        map.insert("yetmişdokuz", 79);
        
        map.insert("yetmişdört", 74);
        
        map.insert("yetmişiki", 72);
        
        map.insert("yetmişsekiz", 78);
        
        map.insert("yetmişyedi", 77);
        
        map.insert("yetmişüç", 73);
        
        map.insert("yirmialtı", 26);
        
        map.insert("yirmibeş", 25);
        
        map.insert("yirmibi", 21);
        
        map.insert("yirmibir", 21);
        
        map.insert("yirmidokuz", 29);
        
        map.insert("yirmidört", 24);
        
        map.insert("yirmiiki", 22);
        
        map.insert("yirmisekiz", 28);
        
        map.insert("yirmiyedi", 27);
        
        map.insert("yirmiüç", 23);
        
        map
    };
}

// Dictionary: thousands_dictionary
lazy_static! {
    static ref THOUSANDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altıbin", 6000);
        
        map.insert("beşbin", 5000);
        
        map.insert("bin", 1000);
        
        map.insert("dokuzbin", 9000);
        
        map.insert("dörtbin", 4000);
        
        map.insert("ikibin", 2000);
        
        map.insert("sekizbin", 8000);
        
        map.insert("yedibin", 7000);
        
        map.insert("üçbin", 3000);
        
        map
    };
}

// Dictionary: tenThousands_dictionary
lazy_static! {
    static ref TENTHOUSANDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altmışbin", 60000);
        
        map.insert("atmışbin", 60000);
        
        map.insert("doksanbin", 90000);
        
        map.insert("ellibin", 50000);
        
        map.insert("kırkbin", 40000);
        
        map.insert("onbin", 10000);
        
        map.insert("otuzbin", 30000);
        
        map.insert("seksenbin", 80000);
        
        map.insert("yetmişbin", 70000);
        
        map.insert("yirmibin", 20000);
        
        map
    };
}

// Dictionary: hundredThousands_dictionary
lazy_static! {
    static ref HUNDREDTHOUSANDS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altıyüzbin", 600000);
        
        map.insert("beşyüzbin", 500000);
        
        map.insert("dokuzyüzbin", 900000);
        
        map.insert("dörtyüzbin", 400000);
        
        map.insert("ikiyüzbin", 200000);
        
        map.insert("sekizyüzbin", 800000);
        
        map.insert("yediyüzbin", 700000);
        
        map.insert("yüzbin", 100000);
        
        map.insert("üçyüzbin", 300000);
        
        map
    };
}

// Dictionary: integer9_dictionary
lazy_static! {
    static ref INTEGER9_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altmışaltıbuçuk", 66.5);
        
        map.insert("altmışbeşbuçuk", 65.5);
        
        map.insert("altmışbirbuçuk", 61.5);
        
        map.insert("altmışdokuzbuçuk", 69.5);
        
        map.insert("altmışdörtbuçuk", 64.5);
        
        map.insert("altmışikibuçuk", 62.5);
        
        map.insert("altmışsekizbuçuk", 68.5);
        
        map.insert("altmışyedibuçuk", 67.5);
        
        map.insert("altmışüçbuçuk", 63.5);
        
        map.insert("atmışaltıbuçuk", 66.5);
        
        map.insert("atmışbeşbuçuk", 65.5);
        
        map.insert("atmışbirbuçuk", 61.5);
        
        map.insert("atmışdokuzbuçuk", 69.5);
        
        map.insert("atmışdörtbuçuk", 64.5);
        
        map.insert("atmışikibuçuk", 62.5);
        
        map.insert("atmışsekizbuçuk", 68.5);
        
        map.insert("atmışyedibuçuk", 67.5);
        
        map.insert("atmışüçbuçuk", 63.5);
        
        map.insert("doksanaltıbuçuk", 96.5);
        
        map.insert("doksanbeşbuçuk", 95.5);
        
        map.insert("doksanbibuçuk", 91.5);
        
        map.insert("doksanbirbuçuk", 91.5);
        
        map.insert("doksandokuzbuçuk", 99.5);
        
        map.insert("doksandörtbuçuk", 94.5);
        
        map.insert("doksanikibuçuk", 92.5);
        
        map.insert("doksansekizbuçuk", 98.5);
        
        map.insert("doksanyedibuçuk", 97.5);
        
        map.insert("doksanüçbuçuk", 93.5);
        
        map.insert("ellialtıbuçuk", 56.5);
        
        map.insert("ellibeşbuçuk", 55.5);
        
        map.insert("ellibibuçuk", 51.5);
        
        map.insert("ellibirbuçuk", 51.5);
        
        map.insert("ellidokuzbuçuk", 59.5);
        
        map.insert("ellidörtbuçuk", 54.5);
        
        map.insert("elliikibuçuk", 52.5);
        
        map.insert("ellisekizbuçuk", 58.5);
        
        map.insert("elliyedibuçuk", 57.5);
        
        map.insert("elliüçbuçuk", 53.5);
        
        map.insert("kırkaltıbuçuk", 46.5);
        
        map.insert("kırkbeşbuçuk", 45.5);
        
        map.insert("kırkbibuçuk", 41.5);
        
        map.insert("kırkbirbuçuk", 41.5);
        
        map.insert("kırkdokuzbuçuk", 49.5);
        
        map.insert("kırkdörtbuçuk", 44.5);
        
        map.insert("kırkikibuçuk", 42.5);
        
        map.insert("kırksekizbuçuk", 48.5);
        
        map.insert("kırkyedibuçuk", 47.5);
        
        map.insert("kırküçbuçuk", 43.5);
        
        map.insert("onaltıbuçuk", 16.5);
        
        map.insert("onbeşbuçuk", 15.5);
        
        map.insert("onbibuçuk", 11.5);
        
        map.insert("onbirbuçuk", 11.5);
        
        map.insert("ondokuzbuçuk", 19.5);
        
        map.insert("ondörtbuçuk", 14.5);
        
        map.insert("onikibuçuk", 12.5);
        
        map.insert("onsekizbuçuk", 18.5);
        
        map.insert("onyedibuçuk", 17.5);
        
        map.insert("onüçbuçuk", 13.5);
        
        map.insert("otuzaltıbuçuk", 36.5);
        
        map.insert("otuzbeşbuçuk", 35.5);
        
        map.insert("otuzbibuçuk", 31.5);
        
        map.insert("otuzbirbuçuk", 31.5);
        
        map.insert("otuzdokuzbuçuk", 39.5);
        
        map.insert("otuzdörtbuçuk", 34.5);
        
        map.insert("otuzikibuçuk", 32.5);
        
        map.insert("otuzsekizbuçuk", 38.5);
        
        map.insert("otuzyedibuçuk", 37.5);
        
        map.insert("otuzüçbuçuk", 33.5);
        
        map.insert("seksenaltıbuçuk", 86.5);
        
        map.insert("seksenbeşbuçuk", 85.5);
        
        map.insert("seksenbibuçuk", 81.5);
        
        map.insert("seksenbirbuçuk", 81.5);
        
        map.insert("seksendokuzbuçuk", 89.5);
        
        map.insert("seksendörtbuçuk", 84.5);
        
        map.insert("seksenikibuçuk", 82.5);
        
        map.insert("seksensekizbuçuk", 88.5);
        
        map.insert("seksenyedibuçuk", 87.5);
        
        map.insert("seksenüçbuçuk", 83.5);
        
        map.insert("yetmişaltıbuçuk", 76.5);
        
        map.insert("yetmişbeşbuçuk", 75.5);
        
        map.insert("yetmişbibuçuk", 71.5);
        
        map.insert("yetmişbirbuçuk", 71.5);
        
        map.insert("yetmişdokuzbuçuk", 79.5);
        
        map.insert("yetmişdörtbuçuk", 74.5);
        
        map.insert("yetmişikibuçuk", 72.5);
        
        map.insert("yetmişsekizbuçuk", 78.5);
        
        map.insert("yetmişyedibuçuk", 77.5);
        
        map.insert("yetmişüçbuçuk", 73.5);
        
        map.insert("yirmialtıbuçuk", 26.5);
        
        map.insert("yirmibeşbuçuk", 25.5);
        
        map.insert("yirmibibuçuk", 21.5);
        
        map.insert("yirmibirbuçuk", 21.5);
        
        map.insert("yirmidokuzbuçuk", 29.5);
        
        map.insert("yirmidörtbuçuk", 24.5);
        
        map.insert("yirmiikibuçuk", 22.5);
        
        map.insert("yirmisekizbuçuk", 28.5);
        
        map.insert("yirmiyedibuçuk", 27.5);
        
        map.insert("yirmiüçbuçuk", 23.5);
        
        map
    };
}

// Dictionary: oneToNine_dictionary
lazy_static! {
    static ref ONETONINE_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altı", 6);
        
        map.insert("beş", 5);
        
        map.insert("bi", 1);
        
        map.insert("bir", 1);
        
        map.insert("dokuz", 9);
        
        map.insert("dört", 4);
        
        map.insert("hiç", 0);
        
        map.insert("iki", 2);
        
        map.insert("s\305f\305r", 0);
        
        map.insert("sekiz", 8);
        
        map.insert("tek", 1);
        
        map.insert("yedi", 7);
        
        map.insert("yek", 1);
        
        map.insert("yok", 0);
        
        map.insert("üç", 3);
        
        map.insert("üçü", 3);
        
        map
    };
}

// Dictionary: numeralSuffixesHalfsuffixText2_dictionary
lazy_static! {
    static ref NUMERALSUFFIXESHALFSUFFIXTEXT2_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altmışbuçuk", 60.5);
        
        map.insert("atmışbuçuk", 60.5);
        
        map.insert("doksanbuçuk", 90.5);
        
        map.insert("ellibuçuk", 50.5);
        
        map.insert("kırkbuçuk", 40.5);
        
        map.insert("onbuçuk", 10.5);
        
        map.insert("otuzbuçuk", 30.5);
        
        map.insert("seksenbuçuk", 80.5);
        
        map.insert("yetmişbuçuk", 70.5);
        
        map.insert("yirmibuçuk", 20.5);
        
        map
    };
}

// Dictionary: tens_dictionary
lazy_static! {
    static ref TENS_DICTIONARY: HashMap<&'static str, i64> = {
        let mut map = HashMap::new();
        
        map.insert("altmış", 60);
        
        map.insert("atmış", 60);
        
        map.insert("doksan", 90);
        
        map.insert("elli", 50);
        
        map.insert("kırk", 40);
        
        map.insert("on", 10);
        
        map.insert("otuz", 30);
        
        map.insert("seksen", 80);
        
        map.insert("yetmiş", 70);
        
        map.insert("yirmi", 20);
        
        map
    };
}


/// Build Numeral rules for tr locale
///
/// Auto-generated rules:
///   - 10 dictionary rules
///   - 0 constant regex rules
///   - 10 dictionary-reference regex rules
///   - 14 complex rules (manual implementation required)
pub fn rules(b: &RuleSetBuilder<Value>) {
    
    // ========================================
    // Dictionary Rules (10)
    // ========================================
    

    // Rule: hundreds_dictionary
    // Examples: yüz, ikiyüz, üçyüz, dörtyüz, beşyüz
    {
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:hundreds_dictionary",
            b.reg(r"(?i)altıyüz|beşyüz|dokuzyüz|dörtyüz|ikiyüz|sekizyüz|yediyüz|yüz|üçyüz").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: numeralSuffixesHalfsuffixText_dictionary
    // Examples: birbuçuk, bibuçuk, ikibuçuk, ü\231buçuk, dörtbuçuk
    {
        let dict = &*NUMERALSUFFIXESHALFSUFFIXTEXT_DICTIONARY;
        b.rule_1_terminal(
            "tr:numeralSuffixesHalfsuffixText_dictionary",
            b.reg(r"(?i)altıbuçuk|be\351buçuk|bibuçuk|birbuçuk|dokuzbuçuk|dörtbuçuk|ikibuçuk|sekizbuçuk|yedibuçuk|ü\231buçuk").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tenToNintynine_dictionary
    // Examples: onbi, onbir, oniki, onüç, ondört
    {
        let dict = &*TENTONINTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "tr:tenToNintynine_dictionary",
            b.reg(r"(?i)altmışaltı|altmışbeş|altmışbir|altmışdokuz|altmışdört|altmışiki|altmışsekiz|altmışyedi|altmışüç|atmışaltı|atmışbeş|atmışbir|atmışdokuz|atmışdört|atmışiki|atmışsekiz|atmışyedi|atmışüç|doksanaltı|doksanbeş|doksanbi|doksanbir|doksandokuz|doksandört|doksaniki|doksansekiz|doksanyedi|doksanüç|ellialtı|ellibeş|ellibi|ellibir|ellidokuz|ellidört|elliiki|ellisekiz|elliyedi|elliüç|kırkaltı|kırkbeş|kırkbi|kırkbir|kırkdokuz|kırkdört|kırkiki|kırksekiz|kırkyedi|kırküç|onaltı|onbeş|onbi|onbir|ondokuz|ondört|oniki|onsekiz|onyedi|onüç|otuzaltı|otuzbeş|otuzbi|otuzbir|otuzdokuz|otuzdört|otuziki|otuzsekiz|otuzyedi|otuzüç|seksenaltı|seksenbeş|seksenbi|seksenbir|seksendokuz|seksendört|sekseniki|seksensekiz|seksenyedi|seksenüç|yetmişaltı|yetmişbeş|yetmişbi|yetmişbir|yetmişdokuz|yetmişdört|yetmişiki|yetmişsekiz|yetmişyedi|yetmişüç|yirmialtı|yirmibeş|yirmibi|yirmibir|yirmidokuz|yirmidört|yirmiiki|yirmisekiz|yirmiyedi|yirmiüç").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: thousands_dictionary
    // Examples: bin, ikibin, üçbin, dörtbin, beşbin
    {
        let dict = &*THOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:thousands_dictionary",
            b.reg(r"(?i)altıbin|beşbin|bin|dokuzbin|dörtbin|ikibin|sekizbin|yedibin|üçbin").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tenThousands_dictionary
    // Examples: onbin, yirmibin, otuzbin, kırkbin, ellibin
    {
        let dict = &*TENTHOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:tenThousands_dictionary",
            b.reg(r"(?i)altmışbin|atmışbin|doksanbin|ellibin|kırkbin|onbin|otuzbin|seksenbin|yetmişbin|yirmibin").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: hundredThousands_dictionary
    // Examples: yüzbin, ikiyüzbin, üçyüzbin, dörtyüzbin, beşyüzbin
    {
        let dict = &*HUNDREDTHOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:hundredThousands_dictionary",
            b.reg(r"(?i)altıyüzbin|beşyüzbin|dokuzyüzbin|dörtyüzbin|ikiyüzbin|sekizyüzbin|yediyüzbin|yüzbin|üçyüzbin").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: integer9_dictionary
    // Examples: onbirbuçuk, onbibuçuk, onikibuçuk, onüçbuçuk, ondörtbuçuk
    {
        let dict = &*INTEGER9_DICTIONARY;
        b.rule_1_terminal(
            "tr:integer9_dictionary",
            b.reg(r"(?i)altmışaltıbuçuk|altmışbeşbuçuk|altmışbirbuçuk|altmışdokuzbuçuk|altmışdörtbuçuk|altmışikibuçuk|altmışsekizbuçuk|altmışyedibuçuk|altmışüçbuçuk|atmışaltıbuçuk|atmışbeşbuçuk|atmışbirbuçuk|atmışdokuzbuçuk|atmışdörtbuçuk|atmışikibuçuk|atmışsekizbuçuk|atmışyedibuçuk|atmışüçbuçuk|doksanaltıbuçuk|doksanbeşbuçuk|doksanbibuçuk|doksanbirbuçuk|doksandokuzbuçuk|doksandörtbuçuk|doksanikibuçuk|doksansekizbuçuk|doksanyedibuçuk|doksanüçbuçuk|ellialtıbuçuk|ellibeşbuçuk|ellibibuçuk|ellibirbuçuk|ellidokuzbuçuk|ellidörtbuçuk|elliikibuçuk|ellisekizbuçuk|elliyedibuçuk|elliüçbuçuk|kırkaltıbuçuk|kırkbeşbuçuk|kırkbibuçuk|kırkbirbuçuk|kırkdokuzbuçuk|kırkdörtbuçuk|kırkikibuçuk|kırksekizbuçuk|kırkyedibuçuk|kırküçbuçuk|onaltıbuçuk|onbeşbuçuk|onbibuçuk|onbirbuçuk|ondokuzbuçuk|ondörtbuçuk|onikibuçuk|onsekizbuçuk|onyedibuçuk|onüçbuçuk|otuzaltıbuçuk|otuzbeşbuçuk|otuzbibuçuk|otuzbirbuçuk|otuzdokuzbuçuk|otuzdörtbuçuk|otuzikibuçuk|otuzsekizbuçuk|otuzyedibuçuk|otuzüçbuçuk|seksenaltıbuçuk|seksenbeşbuçuk|seksenbibuçuk|seksenbirbuçuk|seksendokuzbuçuk|seksendörtbuçuk|seksenikibuçuk|seksensekizbuçuk|seksenyedibuçuk|seksenüçbuçuk|yetmişaltıbuçuk|yetmişbeşbuçuk|yetmişbibuçuk|yetmişbirbuçuk|yetmişdokuzbuçuk|yetmişdörtbuçuk|yetmişikibuçuk|yetmişsekizbuçuk|yetmişyedibuçuk|yetmişüçbuçuk|yirmialtıbuçuk|yirmibeşbuçuk|yirmibibuçuk|yirmibirbuçuk|yirmidokuzbuçuk|yirmidörtbuçuk|yirmiikibuçuk|yirmisekizbuçuk|yirmiyedibuçuk|yirmiüçbuçuk").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: oneToNine_dictionary
    // Examples: s\305f\305r, yok, hiç, bir, bi
    {
        let dict = &*ONETONINE_DICTIONARY;
        b.rule_1_terminal(
            "tr:oneToNine_dictionary",
            b.reg(r"(?i)altı|beş|bi|bir|dokuz|dört|hiç|iki|s\305f\305r|sekiz|tek|yedi|yek|yok|üç|üçü").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: numeralSuffixesHalfsuffixText2_dictionary
    // Examples: onbuçuk, yirmibuçuk, otuzbuçuk, kırkbuçuk, ellibuçuk
    {
        let dict = &*NUMERALSUFFIXESHALFSUFFIXTEXT2_DICTIONARY;
        b.rule_1_terminal(
            "tr:numeralSuffixesHalfsuffixText2_dictionary",
            b.reg(r"(?i)altmışbuçuk|atmışbuçuk|doksanbuçuk|ellibuçuk|kırkbuçuk|onbuçuk|otuzbuçuk|seksenbuçuk|yetmişbuçuk|yirmibuçuk").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: tens_dictionary
    // Examples: on, yirmi, otuz, kırk, elli
    {
        let dict = &*TENS_DICTIONARY;
        b.rule_1_terminal(
            "tr:tens_dictionary",
            b.reg(r"(?i)altmış|atmış|doksan|elli|kırk|on|otuz|seksen|yetmiş|yirmi").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    

    
    // ========================================
    // Dictionary-Reference Regex Rules (10)
    // ========================================
    

    // Rule: Integer5 (refs: hundredsMap)
    {
        
        
        let dict = &*HUNDREDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer5",
            b.reg(r"(?i)(yüz|ikiyüz|üçyüz|dörtyüz|beşyüz|altıyüz|yediyüz|sekizyüz|dokuzyüz)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: NumeralSuffixesHalfsuffixText (refs: numeralSuffixesHalfsuffixTextMap)
    {
        
        
        let dict = &*NUMERALSUFFIXESHALFSUFFIXTEXT_DICTIONARY;
        b.rule_1_terminal(
            "tr:NumeralSuffixesHalfsuffixText",
            b.reg(r"(?i)((bir?|iki|üçü?|dört|beş|altı|yedi|sekiz|dokuz)(buçuk))").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer3 (refs: tenToNintynineMap)
    {
        
        
        let dict = &*TENTONINTYNINE_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer3",
            b.reg(r"(?i)((on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)(bir|bi|iki|üç|dört|beş|altı|yedi|sekiz|dokuz))").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer6 (refs: thousandsMap)
    {
        
        
        let dict = &*THOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer6",
            b.reg(r"(?i)(bin|ikibin|üçbin|dörtbin|beşbin|altıbin|yedibin|sekizbin|dokuzbin)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer7 (refs: tenThousandsMap)
    {
        
        
        let dict = &*TENTHOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer7",
            b.reg(r"(?i)(onbin|yirmibin|otuzbin|kırkbin|ellibin|atmışbin|altmışbin|yetmişbin|seksenbin|doksanbin)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer8 (refs: hundredThousandsMap)
    {
        
        
        let dict = &*HUNDREDTHOUSANDS_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer8",
            b.reg(r"(?i)(yüzbin|ikiyüzbin|üçyüzbin|dörtyüzbin|beşyüzbin|altıyüzbin|yediyüzbin|sekizyüzbin|dokuzyüzbin)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer9 (refs: integer9Map)
    {
        
        
        let dict = &*INTEGER9_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer9",
            b.reg(r"(?i)((on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)(bir|bi|iki|üçü?|dört|beş|altı|yedi|sekiz|dokuz)(buçuk))").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: Integer (refs: oneToNineMap)
    {
        
        
        let dict = &*ONETONINE_DICTIONARY;
        b.rule_1_terminal(
            "tr:Integer",
            b.reg(r"(?i)(yok|hi(ç)|s(ı)f(ı)r|bir?|[ty]ek|iki|(ü)(ç)(ü)?|d(ö)rt|be(ş)|alt(ı)|yedi|sekiz|dokuz)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    

    // Rule: NumeralSuffixesHalfsuffixText2 (refs: numeralSuffixesHalfsuffixText2Map)
    {
        
        
        let dict = &*NUMERALSUFFIXESHALFSUFFIXTEXT2_DICTIONARY;
        b.rule_1_terminal(
            "tr:NumeralSuffixesHalfsuffixText2",
            b.reg(r"(?i)((on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)(buçuk))").unwrap(),
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
            "tr:Integer2",
            b.reg(r"(?i)(on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)").unwrap(),
            move |text_match| {
                let text = text_match.group(0).to_lowercase();
                dict.get(text.as_str())
                    .map(|&value| Value::Integer(value))
                    .ok_or_else(|| rustling_error!("Dictionary lookup failed for: {}", text))
            }
        );
    }
    
    

    
    // ========================================
    // Complex Rules - Manual Implementation Required (14)
    // ========================================
    

    // TODO: NumeralsPrefixWithNegativeOrMinus (regex)
    
    //   Original: numbers prefix with -, negative or minus
    
    // Manual implementation required
    

    // TODO: ACoupleOf (regex)
    
    //   Original: a couple (of)
    
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
    

    // TODO: Half (regex)
    
    //   Original: half
    
    // Manual implementation required
    

    // TODO: Dozen (regex)
    
    //   Original: dozen
    
    // Manual implementation required
    

    // TODO: GroupOfTens (regex)
    
    //   Original: group of ten(s)
    
    // Manual implementation required
    

    // TODO: Quarter (regex)
    
    //   Original: quarter
    
    // Manual implementation required
    

    // TODO: Multiply (composite)
    
    //   Original: compose by multiplication
    
    // Manual implementation required
    

    // TODO: Intersect (composite)
    
    //   Original: intersect
    
    // Manual implementation required
    

    // TODO: Integer4 (composite)
    
    //   Original: integer 11..99
    
    // Manual implementation required
    

    
    eprintln!("⚠️  tr/numeral has 14 unimplemented complex rules");
    
    
}

// ========================================
// Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::{RuleSetBuilder, BoundariesChecker};

    #[test]
    fn test_tr_numeral_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    
    #[test]
    fn test_tr_numeral_dictionaries() {
        
        assert!(HUNDREDS_DICTIONARY.len() > 0, "hundreds_dictionary should not be empty");
        
        assert!(NUMERALSUFFIXESHALFSUFFIXTEXT_DICTIONARY.len() > 0, "numeralSuffixesHalfsuffixText_dictionary should not be empty");
        
        assert!(TENTONINTYNINE_DICTIONARY.len() > 0, "tenToNintynine_dictionary should not be empty");
        
        assert!(THOUSANDS_DICTIONARY.len() > 0, "thousands_dictionary should not be empty");
        
        assert!(TENTHOUSANDS_DICTIONARY.len() > 0, "tenThousands_dictionary should not be empty");
        
        assert!(HUNDREDTHOUSANDS_DICTIONARY.len() > 0, "hundredThousands_dictionary should not be empty");
        
        assert!(INTEGER9_DICTIONARY.len() > 0, "integer9_dictionary should not be empty");
        
        assert!(ONETONINE_DICTIONARY.len() > 0, "oneToNine_dictionary should not be empty");
        
        assert!(NUMERALSUFFIXESHALFSUFFIXTEXT2_DICTIONARY.len() > 0, "numeralSuffixesHalfsuffixText2_dictionary should not be empty");
        
        assert!(TENS_DICTIONARY.len() > 0, "tens_dictionary should not be empty");
        
    }
    

    #[test]
    fn test_tr_numeral_stats() {
        // Generation statistics
        let total_rules = 34;
        let auto_generated = 20;
        let manual_needed = 14;

        assert_eq!(total_rules, auto_generated + manual_needed);

        // Log stats (visible with --nocapture)
        eprintln!("tr/numeral Stats:");
        eprintln!("  Total rules: {}", total_rules);
        eprintln!("  Auto-generated: {} ({:.1}%)", auto_generated, (auto_generated as f64 / total_rules as f64) * 100.0);
        eprintln!("  Manual needed: {} ({:.1}%)", manual_needed, (manual_needed as f64 / total_rules as f64) * 100.0);
    }
}

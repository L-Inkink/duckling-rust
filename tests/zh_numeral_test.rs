// Test Chinese (ZH) Numeral implementation
//
// Validates that auto-generated ZH numeral rules work correctly

use rustling::values::Value;
use rustling_core::{BoundariesChecker, RuleSetBuilder};

#[test]
fn test_zh_basic_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::zh::numeral::rules(&b);
    let ruleset = b.build();

    // Test basic numbers 0-10
    let test_cases = vec![
        ("零", 0),
        ("一", 1),
        ("二", 2),
        ("三", 3),
        ("四", 4),
        ("五", 5),
        ("六", 6),
        ("七", 7),
        ("八", 8),
        ("九", 9),
        ("十", 10),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_zh_teens() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::zh::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("十一", 11),
        ("十二", 12),
        ("十五", 15),
        ("十九", 19),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_zh_tens() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::zh::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("二十", 20),
        ("三十", 30),
        ("五十", 50),
        ("九十", 90),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_zh_hundreds() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::zh::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("一百", 100),
        ("二百", 200),
        ("五百", 500),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_zh_variant_forms() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::zh::numeral::rules(&b);
    let ruleset = b.build();

    // Test variant forms (traditional, financial)
    let test_cases = vec![
        ("壹", 1),      // Financial form of 一
        ("貳", 2),      // Financial form of 二
        ("兩", 2),      // Alternative form of 二
        ("零", 0),      // Standard zero
        ("〇", 0),      // Circle zero
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

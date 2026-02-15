// Test Spanish (ES) Numeral implementation
//
// Validates that auto-generated ES numeral rules work correctly

use rustling::values::Value;
use rustling_core::{BoundariesChecker, RuleSetBuilder};

#[test]
fn test_es_basic_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::es::numeral::rules(&b);
    let ruleset = b.build();

    // Test basic numbers 0-10
    let test_cases = vec![
        ("cero", 0),
        ("uno", 1),
        ("dos", 2),
        ("tres", 3),
        ("cuatro", 4),
        ("cinco", 5),
        ("seis", 6),
        ("siete", 7),
        ("ocho", 8),
        ("nueve", 9),
        ("diez", 10),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_es_teens() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::es::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("once", 11),
        ("doce", 12),
        ("trece", 13),
        ("catorce", 14),
        ("quince", 15),
        ("dieciséis", 16),
        ("diecisiete", 17),
        ("dieciocho", 18),
        ("diecinueve", 19),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_es_tens() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::es::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("veinte", 20),
        ("treinta", 30),
        ("cuarenta", 40),
        ("cincuenta", 50),
        ("sesenta", 60),
        ("setenta", 70),
        ("ochenta", 80),
        ("noventa", 90),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_es_hundreds() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::es::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("cien", 100),
        ("doscientos", 200),
        ("trescientos", 300),
        ("quinientos", 500),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

#[test]
fn test_es_case_insensitive() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rustling::languages::es::numeral::rules(&b);
    let ruleset = b.build();

    let test_cases = vec![
        ("CINCO", 5),
        ("Diez", 10),
        ("VEINTE", 20),
    ];

    for (text, expected) in test_cases {
        let results = ruleset.apply_all(text).unwrap();
        let has_value = results.iter().any(|r| r.value == Value::Integer(expected));
        assert!(has_value, "Failed to parse '{}' → {}", text, expected);
    }
}

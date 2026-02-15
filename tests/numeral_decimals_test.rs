use rustling::values::Value;
use rustling::languages::en::numeral::rules;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_simple_number() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "5"
    let results = ruleset.apply_all("5").unwrap();
    eprintln!("\n=== Test '5' ===");
    eprintln!("Matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    assert!(!results.is_empty(), "Should match '5'");
}

#[test]
fn test_composite_tens() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test cases for composite tens (20-99)
    let test_cases = vec![
        ("twenty three", Value::Integer(23)),
        ("forty five", Value::Integer(45)),
        ("ninety nine", Value::Integer(99)),
        ("thirty one", Value::Integer(31)),
    ];

    for (input, expected) in test_cases {
        let result = ruleset.apply_all(input).unwrap();
        assert!(!result.is_empty(), "Failed to parse: {}", input);

        eprintln!("\nInput: {}", input);
        eprintln!("Total matches: {}", result.len());
        for (i, r) in result.iter().enumerate() {
            eprintln!("  Match {}: {:?}", i, r.value);
        }

        // Find the match that equals expected value
        let matching_result = result.iter()
            .find(|r| &r.value == &expected)
            .expect(&format!("No match found for expected value {:?} in input: {}", expected, input));

        assert_eq!(&matching_result.value, &expected, "Input: {}", input);
    }
}

#[test]
fn test_decimal_parsing() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test cases for decimals
    let test_cases = vec![
        ("0.5", Value::Float(0.5)),
        ("3.14", Value::Float(3.14)),
        (".25", Value::Float(0.25)),
        ("123.456", Value::Float(123.456)),
    ];

    for (input, expected) in test_cases {
        let result = ruleset.apply_all(input).unwrap();
        assert!(!result.is_empty(), "Failed to parse: {}", input);

        let parsed_value = &result[0].value;
        assert_eq!(parsed_value, &expected, "Input: {}", input);
    }
}

#[test]
fn test_comma_separated_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test cases for comma-separated numbers
    let test_cases = vec![
        ("1,234", Value::Integer(1234)),
        ("10,000", Value::Integer(10000)),
        ("1,234.56", Value::Float(1234.56)),
        ("999,999,999", Value::Integer(999999999)),
    ];

    for (input, expected) in test_cases {
        let result = ruleset.apply_all(input).unwrap();
        assert!(!result.is_empty(), "Failed to parse: {}", input);

        eprintln!("\nInput: {}", input);
        eprintln!("Total matches: {}", result.len());
        for (i, r) in result.iter().enumerate() {
            eprintln!("  Match {}: {:?}", i, r.value);
        }

        // Find the match that equals expected value
        let matching_result = result.iter()
            .find(|r| &r.value == &expected)
            .expect(&format!("No match found for expected value {:?} in input: {}", expected, input));

        assert_eq!(&matching_result.value, &expected, "Input: {}", input);
    }
}

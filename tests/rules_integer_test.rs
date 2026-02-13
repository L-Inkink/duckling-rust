use rustling::rules;
use rustling::values::Value;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_integer_rule_single_digit() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("5").unwrap();
    assert_eq!(results.len(), 1);

    if let Value::Integer(n) = results[0].value {
        assert_eq!(n, 5);
    } else {
        panic!("Expected Integer value");
    }
}

#[test]
fn test_integer_rule_multi_digit() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("123").unwrap();
    assert_eq!(results.len(), 1);

    if let Value::Integer(n) = results[0].value {
        assert_eq!(n, 123);
    } else {
        panic!("Expected Integer value");
    }
}

#[test]
fn test_integer_rule_in_sentence() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::integer::rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("I have 42 apples").unwrap();
    assert!(results.len() >= 1);

    // Find Integer result
    let int_result = results.iter().find(|r| {
        matches!(r.value, Value::Integer(_))
    }).expect("Should find Integer");

    if let Value::Integer(n) = int_result.value {
        assert_eq!(n, 42);
    }
}

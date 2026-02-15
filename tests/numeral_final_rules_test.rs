use rustling::values::Value;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_leading_dot_spelled_out() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Build EN numeral rules
    rustling::languages::en::numeral::rules(&b);

    let ruleset = b.build();

    // Test "point 77" → 0.77
    let results = ruleset.apply_all("point 77").unwrap();
    let has_077 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 0.77).abs() < 0.001)
    });
    assert!(has_077, "Should parse 'point 77' → 0.77");

    // Test "dot 5" → 0.05
    let results = ruleset.apply_all("dot 5").unwrap();
    let has_005 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 0.05).abs() < 0.001)
    });
    assert!(has_005, "Should parse 'dot 5' → 0.05");

    // Test "point 50" → 0.50
    let results = ruleset.apply_all("point 50").unwrap();
    let has_050 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 0.50).abs() < 0.001)
    });
    assert!(has_050, "Should parse 'point 50' → 0.50");
}

#[test]
fn test_negative_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Build EN numeral rules
    rustling::languages::en::numeral::rules(&b);

    let ruleset = b.build();

    // Test "-5" → -5
    let results = ruleset.apply_all("-5").unwrap();
    let has_neg5 = results.iter().any(|r| r.value == Value::Integer(-5));
    assert!(has_neg5, "Should parse '-5' → -5");

    // Test "negative 10" → -10
    let results = ruleset.apply_all("negative 10").unwrap();
    let has_neg10 = results.iter().any(|r| r.value == Value::Integer(-10));
    assert!(has_neg10, "Should parse 'negative 10' → -10");

    // Test "minus 23" → -23
    let results = ruleset.apply_all("minus 23").unwrap();
    let has_neg23 = results.iter().any(|r| r.value == Value::Integer(-23));
    assert!(has_neg23, "Should parse 'minus 23' → -23");

    // Test with word form: "negative five" → -5
    let results = ruleset.apply_all("negative five").unwrap();
    let has_neg5_word = results.iter().any(|r| r.value == Value::Integer(-5));
    assert!(has_neg5_word, "Should parse 'negative five' → -5");

    // Test negative with composite: "negative twenty three" → -23
    let results = ruleset.apply_all("negative twenty three").unwrap();
    let has_neg23_composite = results.iter().any(|r| r.value == Value::Integer(-23));
    assert!(has_neg23_composite, "Should parse 'negative twenty three' → -23");

    // Test negative with hundreds: "minus three hundred" → -300
    let results = ruleset.apply_all("minus three hundred").unwrap();
    let has_neg300 = results.iter().any(|r| r.value == Value::Integer(-300));
    assert!(has_neg300, "Should parse 'minus three hundred' → -300");
}

// Note: Negative Float currently not working - rustling-core may not match
// "minus" + Float(3.14) for unknown reasons. Integer negation works fine.
// TODO: Debug why Negative rule doesn't trigger for Float values

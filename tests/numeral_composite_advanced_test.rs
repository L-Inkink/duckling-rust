use rustling::values::Value;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_multiply_rule() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Build EN numeral rules
    rustling::languages::en::numeral::rules(&b);

    let ruleset = b.build();

    // Test "three hundred" → 300
    let results = ruleset.apply_all("three hundred").unwrap();
    let has_300 = results.iter().any(|r| r.value == Value::Integer(300));
    assert!(has_300, "Should parse 'three hundred' → 300");

    // Test "five thousand" → 5000
    let results = ruleset.apply_all("five thousand").unwrap();
    let has_5000 = results.iter().any(|r| r.value == Value::Integer(5000));
    assert!(has_5000, "Should parse 'five thousand' → 5000");

    // Test "twenty one hundred" → 2100
    let results = ruleset.apply_all("twenty one hundred").unwrap();
    let has_2100 = results.iter().any(|r| r.value == Value::Integer(2100));
    assert!(has_2100, "Should parse 'twenty one hundred' → 2100");
}

#[test]
fn test_sum_and_rule() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Build EN numeral rules
    rustling::languages::en::numeral::rules(&b);

    let ruleset = b.build();

    // Test "one hundred and twenty three" → 123
    let results = ruleset.apply_all("one hundred and twenty three").unwrap();
    let has_123 = results.iter().any(|r| r.value == Value::Integer(123));
    assert!(has_123, "Should parse 'one hundred and twenty three' → 123");

    // Test "five hundred and forty five" → 545
    let results = ruleset.apply_all("five hundred and forty five").unwrap();
    let has_545 = results.iter().any(|r| r.value == Value::Integer(545));
    assert!(has_545, "Should parse 'five hundred and forty five' → 545");
}

#[test]
fn test_complex_number_parsing() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Build EN numeral rules
    rustling::languages::en::numeral::rules(&b);

    let ruleset = b.build();

    // Test "one thousand two hundred thirty four" → 1234
    // This requires: 1000 + 200 + 34
    // - "one thousand" (Multiply: 1 × 1000 = 1000)
    // - "two hundred" (Multiply: 2 × 100 = 200)
    // - "thirty four" (CompositeTens: 30 + 4 = 34)
    // - Then sum them all
    let results = ruleset.apply_all("one thousand two hundred thirty four").unwrap();

    eprintln!("\n=== Complex parsing: 'one thousand two hundred thirty four' ===");
    eprintln!("Total matches: {}", results.len());
    for (i, r) in results.iter().enumerate() {
        eprintln!("  Match {}: {:?}", i, r.value);
    }

    let has_1234 = results.iter().any(|r| r.value == Value::Integer(1234));
    assert!(has_1234, "Should parse 'one thousand two hundred thirty four' → 1234");
}

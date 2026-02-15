use rustling::values::Value;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::rustling_error;
use rustling::dim;

#[test]
fn test_simple_rule2() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Simple rule to match "twenty" → 20
    b.rule_1_terminal(
        "twenty",
        b.reg(r"(?i)twenty").unwrap(),
        |_| Ok(Value::Integer(20))
    );

    // Simple rule to match "three" → 3
    b.rule_1_terminal(
        "three",
        b.reg(r"(?i)three").unwrap(),
        |_| Ok(Value::Integer(3))
    );

    // Composite rule: 20 + 3 → 23
    b.rule_2(
        "composite",
        dim!(Value, vec![Box::new(|v: &Value| matches!(v, Value::Integer(20)))]),
        dim!(Value, vec![Box::new(|v: &Value| matches!(v, Value::Integer(3)))]),
        |tens, units| {
            if let (Value::Integer(t), Value::Integer(u)) = (tens.value(), units.value()) {
                Ok(Value::Integer(t + u))
            } else {
                Err(rustling_error!("Invalid types"))
            }
        }
    );

    let ruleset = b.build();

    let result = ruleset.apply_all("twenty three").unwrap();

    eprintln!("\nInput: 'twenty three'");
    eprintln!("Total matches: {}", result.len());
    for (i, r) in result.iter().enumerate() {
        eprintln!("  Match {}: {:?}", i, r.value);
    }

    // Should find 20, 3, and 23
    let has_20 = result.iter().any(|r| r.value == Value::Integer(20));
    let has_3 = result.iter().any(|r| r.value == Value::Integer(3));
    let has_23 = result.iter().any(|r| r.value == Value::Integer(23));

    assert!(has_20, "Should match 'twenty' → 20");
    assert!(has_3, "Should match 'three' → 3");
    assert!(has_23, "Should combine to 23");
}

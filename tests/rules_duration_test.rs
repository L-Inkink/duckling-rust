use rustling::rules;
use rustling::values::{Value, DurationValue, TimeUnit};
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_duration_minutes() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("5 minutes").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 5);
        assert_eq!(d.unit, TimeUnit::Minute);
    }
}

#[test]
fn test_duration_hours() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("2 hours").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 2);
        assert_eq!(d.unit, TimeUnit::Hour);
    }
}

#[test]
fn test_duration_singular() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let results = ruleset.apply_all("1 minute").unwrap();

    let dur_result = results.iter().find(|r| {
        matches!(r.value, Value::Duration(_))
    }).expect("Should find Duration");

    if let Value::Duration(d) = &dur_result.value {
        assert_eq!(d.amount, 1);
        assert_eq!(d.unit, TimeUnit::Minute);
    }
}

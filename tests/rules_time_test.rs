use rustling::rules;
use rustling::values::{Value, TimeValue};
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use chrono::{Utc, Duration};

#[test]
fn test_in_5_minutes() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let now = Utc::now();
    let results = ruleset.apply_all("in 5 minutes").unwrap();

    let time_result = results.iter().find(|r| {
        matches!(r.value, Value::Time(_))
    }).expect("Should find Time");

    if let Value::Time(t) = &time_result.value {
        let expected = now + Duration::minutes(5);
        let diff = (t.timestamp - expected).num_seconds().abs();
        assert!(diff < 2, "Time should be ~5 minutes from now");
    }
}

#[test]
fn test_in_2_hours() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    rules::register_all_rules(&b);
    let ruleset = b.build();

    let now = Utc::now();
    let results = ruleset.apply_all("in 2 hours").unwrap();

    let time_result = results.iter().find(|r| {
        matches!(r.value, Value::Time(_))
    }).expect("Should find Time");

    if let Value::Time(t) = &time_result.value {
        let expected = now + Duration::hours(2);
        let diff = (t.timestamp - expected).num_seconds().abs();
        assert!(diff < 2, "Time should be ~2 hours from now");
    }
}

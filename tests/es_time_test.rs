//! Spanish Time Integration Tests

use rustling::values::Value;
use rustling::languages::es::time as es_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, TimeContext};
use chrono::{TimeZone, Utc};
use std::sync::Arc;

fn setup_ruleset_with_context(ref_time: Option<chrono::DateTime<chrono::Utc>>) -> rustling_core::RuleSet<Value> {
    let ctx = if let Some(time) = ref_time {
        Some(Arc::new(TimeContext::for_test(time)))
    } else {
        None
    };

    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    es_time::rules(&b, ctx);
    b.build()
}

fn parse_and_get_datetime(ruleset: &rustling_core::RuleSet<Value>, text: &str) -> Option<chrono::DateTime<Utc>> {
    let results = ruleset.apply_all(text).ok()?;

    for result in results.iter() {
        if let Value::Time(TimeValue::Instant(td)) = &result.value {
            return Some(td.datetime);
        }
    }
    None
}

// ============================================================
// Instant Tests
// ============================================================

#[test]
fn test_ahora() {
    // "ahora" = now
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 6, 15, 10, 30, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "ahora");
    assert!(result.is_some(), "Should parse 'ahora'");
}

#[test]
fn test_hoy() {
    // "hoy" = today
    let ref_time = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
    let ruleset = setup_ruleset_with_context(Some(ref_time));

    let result = parse_and_get_datetime(&ruleset, "hoy");
    assert!(result.is_some(), "Should parse 'hoy'");

    // Should return reference date
    if let Some(dt) = result {
        assert_eq!(dt.date_naive(), ref_time.date_naive());
    }
}

#[test]
fn test_manana() {
    // "mañana" = tomorrow
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "mañana");
    assert!(result.is_some(), "Should parse 'mañana'");

    // Should return tomorrow's date
    if let Some(dt) = result {
        let expected = Utc.with_ymd_and_hms(2024, 6, 16, 0, 0, 0).unwrap().date_naive();
        assert_eq!(dt.date_naive(), expected);
    }
}

#[test]
fn test_ayer() {
    // "ayer" = yesterday
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "ayer");
    assert!(result.is_some(), "Should parse 'ayer'");

    // Should return yesterday's date
    if let Some(dt) = result {
        let expected = Utc.with_ymd_and_hms(2024, 6, 14, 0, 0, 0).unwrap().date_naive();
        assert_eq!(dt.date_naive(), expected);
    }
}

// ============================================================
// Day of Week Tests
// ============================================================

#[test]
fn test_lunes() {
    // "lunes" = Monday
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap())); // Saturday

    let result = parse_and_get_datetime(&ruleset, "lunes");
    assert!(result.is_some(), "Should parse 'lunes'");
}

#[test]
fn test_martes() {
    // "martes" = Tuesday
    let ruleset = setup_ruleset_with_context(None);

    let result = parse_and_get_datetime(&ruleset, "martes");
    assert!(result.is_some(), "Should parse 'martes'");
}

// ============================================================
// Month Tests
// ============================================================

#[test]
fn test_enero() {
    // "enero" = January
    let ruleset = setup_ruleset_with_context(None);

    let result = parse_and_get_datetime(&ruleset, "enero");
    assert!(result.is_some(), "Should parse 'enero'");
}

#[test]
fn test_diciembre() {
    // "diciembre" = December
    let ruleset = setup_ruleset_with_context(None);

    let result = parse_and_get_datetime(&ruleset, "diciembre");
    assert!(result.is_some(), "Should parse 'diciembre'");
}

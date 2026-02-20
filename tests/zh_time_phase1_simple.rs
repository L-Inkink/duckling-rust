// Phase 1 Tests: Chinese Time - Basic & Simple Patterns (25 rules)
// Following TDD: Tests written BEFORE implementation
//
// Simplified test version using correct pattern from EN tests

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain};
use chrono::{Utc, Datelike, Weekday};

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// Simple Time References (7 rules tested)
// ============================================

#[test]
fn test_now_simplified() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("现在").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '现在' (now)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Second);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_today() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("今天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '今天' (today)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
        assert_eq!(td.datetime.day(), Utc::now().day());
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_tomorrow() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("明天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '明天' (tomorrow)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_yesterday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("昨天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '昨天' (yesterday)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_day_after_tomorrow() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("后天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '后天' (day after tomorrow)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_three_days_from_now() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("大后天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '大后天' (3 days from now)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_day_before_yesterday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("前天").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '前天' (day before yesterday)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

// ============================================
// Days of Week (7 rules tested)
// ============================================

#[test]
fn test_monday() {
    let ruleset = setup_ruleset();

    // Test primary format: 星期一
    let results = ruleset.apply_all("星期一").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '星期一' (Monday)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.weekday(), Weekday::Mon);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_tuesday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("星期二").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Tue
        } else {
            false
        }
    }), "Should parse '星期二' as Tuesday");
}

#[test]
fn test_wednesday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("星期三").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Wed
        } else {
            false
        }
    }), "Should parse '星期三' as Wednesday");
}

#[test]
fn test_thursday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("星期四").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Thu
        } else {
            false
        }
    }), "Should parse '星期四' as Thursday");
}

#[test]
fn test_friday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("星期五").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Fri
        } else {
            false
        }
    }), "Should parse '星期五' as Friday");
}

#[test]
fn test_saturday() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("星期六").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Sat
        } else {
            false
        }
    }), "Should parse '星期六' as Saturday");
}

#[test]
fn test_sunday() {
    let ruleset = setup_ruleset();
    // Note: Sunday can be 星期日 or 星期天
    let results = ruleset.apply_all("星期日").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Sun
        } else {
            false
        }
    }), "Should parse '星期日' as Sunday");
}

// ============================================
// Relative Time Periods (6 rules tested)
// ============================================

#[test]
fn test_last_week() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("上周").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '上周' (last week)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Week);
        assert!(td.datetime < Utc::now(), "Should be in the past");
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_next_week() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("下周").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '下周' (next week)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Week);
        assert!(td.datetime > Utc::now(), "Should be in the future");
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_last_month() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("上个月").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '上个月' (last month)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Month);
        assert!(td.datetime < Utc::now(), "Should be in the past");
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_next_month() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("下个月").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '下个月' (next month)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Month);
        assert!(td.datetime > Utc::now(), "Should be in the future");
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_last_year() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("去年").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '去年' (last year)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Year);
        let expected_year = Utc::now().year() - 1;
        assert_eq!(td.datetime.year(), expected_year);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_next_year() {
    let ruleset = setup_ruleset();
    let results = ruleset.apply_all("明年").unwrap();

    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '明年' (next year)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Year);
        let expected_year = Utc::now().year() + 1;
        assert_eq!(td.datetime.year(), expected_year);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

// Phase 4 Tests: Chinese Time - 持续时间 & 区间 (10 rules)
// 按照TDD原则：先写测试，再实现代码
//
// 测试覆盖:
// - 持续时间模式 (6条规则): X天前/后、X小时前/后、X分钟前/后
// - 区间模式 (4条规则): 从X到Y、X至Y、这周末

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain};
use chrono::{Datelike, Duration, Timelike};

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// 持续时间 - 天 (Days Duration) - 测试"X天前/后"
// ============================================

#[test]
fn test_days_ago_numeric() {
    let ruleset = setup_ruleset();

    // "3天前" - 3 days ago
    let results = ruleset.apply_all("3天前").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '3天前'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        // Should be 3 days in the past
        let today = chrono::Utc::now().date_naive();
        let expected_date = today - Duration::days(3);
        assert_eq!(td.datetime.date_naive(), expected_date);
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_days_ago_chinese() {
    let ruleset = setup_ruleset();

    // "三天前" - 3 days ago (Chinese numeral)
    let results = ruleset.apply_all("三天前").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let today = chrono::Utc::now().date_naive();
            let expected_date = today - Duration::days(3);
            td.datetime.date_naive() == expected_date
        } else {
            false
        }
    }), "应该解析 '三天前' 为3天前");
}

#[test]
fn test_days_from_now_numeric() {
    let ruleset = setup_ruleset();

    // "5天后" - 5 days from now
    let variants = vec!["5天后", "5天後"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let today = chrono::Utc::now().date_naive();
                let expected_date = today + Duration::days(5);
                td.datetime.date_naive() == expected_date
            } else {
                false
            }
        }), "应该解析 '{}' 为5天后", variant);
    }
}

#[test]
fn test_days_from_now_chinese() {
    let ruleset = setup_ruleset();

    // "五天后" - 5 days from now (Chinese numeral)
    let results = ruleset.apply_all("五天后").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let today = chrono::Utc::now().date_naive();
            let expected_date = today + Duration::days(5);
            td.datetime.date_naive() == expected_date
        } else {
            false
        }
    }), "应该解析 '五天后' 为5天后");
}

#[test]
fn test_days_range() {
    let ruleset = setup_ruleset();

    // Test various day counts
    let test_cases = vec![1, 2, 3, 5, 7, 10, 30];

    for days in test_cases {
        let input = format!("{}天前", days);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let today = chrono::Utc::now().date_naive();
                let expected_date = today - Duration::days(days);
                td.datetime.date_naive() == expected_date
            } else {
                false
            }
        }), "应该解析 '{}天前'", days);
    }
}

// ============================================
// 持续时间 - 小时 (Hours Duration) - 测试"X小时前/后"
// ============================================

#[test]
fn test_hours_ago_numeric() {
    let ruleset = setup_ruleset();

    // "2小时前" - 2 hours ago
    let variants = vec!["2小时前", "2小時前"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let now = chrono::Utc::now();
                let expected = now - Duration::hours(2);
                // Allow 1 minute tolerance for test execution time
                let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
                diff < 60 && td.grain == Grain::Hour
            } else {
                false
            }
        }), "应该解析 '{}' 为2小时前", variant);
    }
}

#[test]
fn test_hours_ago_chinese() {
    let ruleset = setup_ruleset();

    // "两小时前" / "二小时前" - 2 hours ago (Chinese)
    let variants = vec!["两小时前", "二小时前"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let now = chrono::Utc::now();
                let expected = now - Duration::hours(2);
                let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
                diff < 60
            } else {
                false
            }
        }), "应该解析 '{}' 为2小时前", variant);
    }
}

#[test]
fn test_hours_from_now_numeric() {
    let ruleset = setup_ruleset();

    // "3小时后" - 3 hours from now
    let variants = vec!["3小时后", "3小時後"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let now = chrono::Utc::now();
                let expected = now + Duration::hours(3);
                let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
                diff < 60 && td.grain == Grain::Hour
            } else {
                false
            }
        }), "应该解析 '{}' 为3小时后", variant);
    }
}

#[test]
fn test_hours_from_now_chinese() {
    let ruleset = setup_ruleset();

    // "三小时后" - 3 hours from now (Chinese)
    let results = ruleset.apply_all("三小时后").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let now = chrono::Utc::now();
            let expected = now + Duration::hours(3);
            let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
            diff < 60
        } else {
            false
        }
    }), "应该解析 '三小时后' 为3小时后");
}

// ============================================
// 持续时间 - 分钟 (Minutes Duration) - 测试"X分钟前/后"
// ============================================

#[test]
fn test_minutes_ago_numeric() {
    let ruleset = setup_ruleset();

    // "15分钟前" - 15 minutes ago
    let variants = vec!["15分钟前", "15分鐘前"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let now = chrono::Utc::now();
                let expected = now - Duration::minutes(15);
                // Allow 5 second tolerance
                let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
                diff < 5 && td.grain == Grain::Minute
            } else {
                false
            }
        }), "应该解析 '{}' 为15分钟前", variant);
    }
}

#[test]
fn test_minutes_ago_chinese() {
    let ruleset = setup_ruleset();

    // "十五分钟前" - 15 minutes ago (Chinese)
    let results = ruleset.apply_all("十五分钟前").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let now = chrono::Utc::now();
            let expected = now - Duration::minutes(15);
            let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
            diff < 5
        } else {
            false
        }
    }), "应该解析 '十五分钟前' 为15分钟前");
}

#[test]
fn test_minutes_from_now_numeric() {
    let ruleset = setup_ruleset();

    // "30分钟后" - 30 minutes from now
    let variants = vec!["30分钟后", "30分鐘後"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let now = chrono::Utc::now();
                let expected = now + Duration::minutes(30);
                let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
                diff < 5 && td.grain == Grain::Minute
            } else {
                false
            }
        }), "应该解析 '{}' 为30分钟后", variant);
    }
}

#[test]
fn test_minutes_from_now_chinese() {
    let ruleset = setup_ruleset();

    // "三十分钟后" - 30 minutes from now (Chinese)
    let results = ruleset.apply_all("三十分钟后").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let now = chrono::Utc::now();
            let expected = now + Duration::minutes(30);
            let diff = (td.datetime.timestamp() - expected.timestamp()).abs();
            diff < 5
        } else {
            false
        }
    }), "应该解析 '三十分钟后' 为30分钟后");
}

// ============================================
// 时间区间 (Time Intervals) - 测试"从X到Y"
// ============================================

#[test]
fn test_hour_interval_from_to() {
    let ruleset = setup_ruleset();

    // "从3点到5点" - from 3 to 5
    let variants = vec!["从3点到5点", "從3點到5點"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
                from.datetime.hour() == 3 && to.datetime.hour() == 5
            } else {
                false
            }
        }), "应该解析 '{}' 为3点到5点的区间", variant);
    }
}

#[test]
fn test_hour_interval_zhi() {
    let ruleset = setup_ruleset();

    // "3点至5点" - 3 to 5 (using 至)
    let results = ruleset.apply_all("3点至5点").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
            from.datetime.hour() == 3 && to.datetime.hour() == 5
        } else {
            false
        }
    }), "应该解析 '3点至5点' 为区间");
}

#[test]
fn test_date_interval() {
    let ruleset = setup_ruleset();

    // "从3月15日到3月20日" - from Mar 15 to Mar 20
    let results = ruleset.apply_all("从3月15日到3月20日").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
            from.datetime.month() == 3 && from.datetime.day() == 15 &&
            to.datetime.month() == 3 && to.datetime.day() == 20
        } else {
            false
        }
    }), "应该解析 '从3月15日到3月20日' 为日期区间");
}

#[test]
fn test_day_interval_chinese() {
    let ruleset = setup_ruleset();

    // "从星期一到星期五" - Monday to Friday
    let results = ruleset.apply_all("从星期一到星期五").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
            from.datetime.weekday() == chrono::Weekday::Mon &&
            to.datetime.weekday() == chrono::Weekday::Fri
        } else {
            false
        }
    }), "应该解析 '从星期一到星期五' 为工作日区间");
}

// ============================================
// 特殊区间 - 这周末 (Special Intervals)
// ============================================

#[test]
fn test_this_weekend() {
    let ruleset = setup_ruleset();

    // "这周末" / "這週末" - this weekend
    let variants = vec!["这周末", "這週末", "这个周末", "這個週末"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
                // Weekend should be Saturday-Sunday
                from.datetime.weekday() == chrono::Weekday::Sat &&
                to.datetime.weekday() == chrono::Weekday::Sun
            } else {
                false
            }
        }), "应该解析 '{}' 为本周末区间", variant);
    }
}

#[test]
fn test_next_weekend() {
    let ruleset = setup_ruleset();

    // "下周末" / "下週末" - next weekend
    let variants = vec!["下周末", "下週末"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            matches!(r.value, Value::Time(TimeValue::Interval { .. }))
        }), "应该解析 '{}' 为下周末区间", variant);
    }
}

#[test]
fn test_last_weekend() {
    let ruleset = setup_ruleset();

    // "上周末" / "上週末" - last weekend
    let variants = vec!["上周末", "上週末"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            matches!(r.value, Value::Time(TimeValue::Interval { .. }))
        }), "应该解析 '{}' 为上周末区间", variant);
    }
}

// ============================================
// 组合变体测试
// ============================================

#[test]
fn test_duration_all_units() {
    let ruleset = setup_ruleset();

    // Test various time units with "前"
    let test_cases = vec![
        ("1天前", Grain::Day),
        ("1小时前", Grain::Hour),
        ("1分钟前", Grain::Minute),
    ];

    for (input, expected_grain) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.grain == expected_grain
            } else {
                false
            }
        }), "应该解析 '{}' 并使用正确的grain", input);
    }
}

#[test]
fn test_duration_simplified_traditional() {
    let ruleset = setup_ruleset();

    // Test simplified vs traditional characters
    let test_pairs = vec![
        ("3天后", "3天後"),
        ("2小时前", "2小時前"),
        ("15分钟后", "15分鐘後"),
    ];

    for (simplified, traditional) in test_pairs {
        let results_s = ruleset.apply_all(simplified).unwrap();
        let results_t = ruleset.apply_all(traditional).unwrap();

        assert!(results_s.iter().any(|r| matches!(r.value, Value::Time(_))),
               "应该解析简体 '{}'", simplified);
        assert!(results_t.iter().any(|r| matches!(r.value, Value::Time(_))),
               "应该解析繁体 '{}'", traditional);
    }
}

#[test]
fn test_interval_all_connectors() {
    let ruleset = setup_ruleset();

    // Test different interval connectors
    let test_cases = vec![
        "从3点到5点",   // 从...到
        "從3點到5點",   // 從...到 (traditional)
        "3点至5点",     // 至
    ];

    for input in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            matches!(r.value, Value::Time(TimeValue::Interval { .. }))
        }), "应该解析 '{}' 为区间", input);
    }
}

// ============================================
// 边界值测试
// ============================================

#[test]
fn test_large_duration_values() {
    let ruleset = setup_ruleset();

    // Test larger duration values
    let test_cases = vec![
        "30天前",
        "24小时后",
        "120分钟前",
    ];

    for input in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            matches!(r.value, Value::Time(TimeValue::Instant(_)))
        }), "应该解析大数值 '{}'", input);
    }
}

#[test]
fn test_interval_across_midnight() {
    let ruleset = setup_ruleset();

    // "从23点到1点" - interval across midnight
    let results = ruleset.apply_all("从23点到1点").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Interval { from, to }) = &r.value {
            from.datetime.hour() == 23 && to.datetime.hour() == 1
        } else {
            false
        }
    }), "应该解析跨午夜的区间");
}

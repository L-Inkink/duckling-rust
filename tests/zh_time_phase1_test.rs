// Phase 1 Tests: Chinese Time - Basic & Simple Patterns (25 rules)
// Following TDD: Tests written BEFORE implementation
//
// Coverage:
// - Simple time references (12 rules): now, today, tomorrow, yesterday, etc.
// - Days of week (7 rules): Monday-Sunday in Chinese
// - Relative time periods (6 rules): last/next week/month/year

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain};
use chrono::{Duration, Utc, Datelike, Weekday};

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// Simple Time References (12 rules)
// ============================================

#[test]
fn test_now_simplified() {
    let ruleset = setup_ruleset();

    // 现在 (xiànzài) - now
    let results = ruleset.apply_all("现在").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("Should find Time value for '现在'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.grain, Grain::Second);
    } else {
        panic!("Expected Instant TimeValue");
    }
}

#[test]
fn test_now_traditional() {
    let ruleset = setup_ruleset();

    // 現在 (traditional)
    let results = ruleset.apply_all("現在").unwrap();
    assert!(results.iter().any(|r| matches!(r.value, Value::Time(_))), "Should parse '現在'");
}

#[test]
fn test_now_variants() {
    let ruleset = setup_ruleset();

    // Test all variants: 此时/此刻/当前/當前
    let variants = vec!["此时", "此刻", "当前", "當前"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| matches!(r.value, Value::Time(_))), "Failed: {}", variant);
    }
}

#[test]
fn test_now_cantonese() {
    let ruleset = setup_ruleset();

    // Cantonese variants: 宜家/而家/依家
    let variants = vec!["宜家", "而家", "依家"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| matches!(r.value, Value::Time(_))), "Failed Cantonese: {}", variant);
    }
}

#[test]
fn test_today() {
    let ruleset = setup_ruleset();
    let now = Utc::now();

    // 今天
    let values = parse("今天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '今天' (today)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        // Should be today's date
        assert_eq!(tv.instant.day(), now.day());
    } else {
        panic!("Expected Time value");
    }

    // 今日
    let values = parse("今日", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '今日' (today alt)");
}

#[test]
fn test_tomorrow() {
    let ruleset = setup_ruleset();
    let tomorrow = Utc::now() + Duration::days(1);

    // 明天
    let values = parse("明天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '明天' (tomorrow)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        assert_eq!(tv.instant.day(), tomorrow.day());
    } else {
        panic!("Expected Time value");
    }

    // 明日
    assert_eq!(parse("明日", &ruleset).len(), 1);

    // 聽日 (Cantonese)
    assert_eq!(parse("聽日", &ruleset).len(), 1);
}

#[test]
fn test_yesterday() {
    let ruleset = setup_ruleset();
    let yesterday = Utc::now() - Duration::days(1);

    // 昨天
    let values = parse("昨天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '昨天' (yesterday)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        assert_eq!(tv.instant.day(), yesterday.day());
    } else {
        panic!("Expected Time value");
    }

    // 昨日
    assert_eq!(parse("昨日", &ruleset).len(), 1);

    // 尋日 (Cantonese)
    assert_eq!(parse("尋日", &ruleset).len(), 1);
}

#[test]
fn test_day_after_tomorrow() {
    let ruleset = setup_ruleset();
    let target = Utc::now() + Duration::days(2);

    // 后天 (simplified)
    let values = parse("后天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '后天' (day after tomorrow)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        assert_eq!(tv.instant.day(), target.day());
    } else {
        panic!("Expected Time value");
    }

    // 後天 (traditional)
    assert_eq!(parse("後天", &ruleset).len(), 1);

    // 後日 (traditional variant)
    assert_eq!(parse("後日", &ruleset).len(), 1);
}

#[test]
fn test_three_days_from_now() {
    let ruleset = setup_ruleset();
    let target = Utc::now() + Duration::days(3);

    // 大后天 (simplified)
    let values = parse("大后天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '大后天' (3 days from now)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        assert_eq!(tv.instant.day(), target.day());
    } else {
        panic!("Expected Time value");
    }

    // 大後天 (traditional)
    assert_eq!(parse("大後天", &ruleset).len(), 1);

    // 大後日 (traditional variant)
    assert_eq!(parse("大後日", &ruleset).len(), 1);
}

#[test]
fn test_day_before_yesterday() {
    let ruleset = setup_ruleset();
    let target = Utc::now() - Duration::days(2);

    // 前天
    let values = parse("前天", &ruleset);
    assert_eq!(values.len(), 1, "Should parse '前天' (day before yesterday)");

    if let Value::Time(tv) = &values[0] {
        assert_eq!(tv.grain, Grain::Day);
        assert_eq!(tv.instant.day(), target.day());
    } else {
        panic!("Expected Time value");
    }

    // 前日
    assert_eq!(parse("前日", &ruleset).len(), 1);
}

// ============================================
// Days of Week (7 rules)
// ============================================

#[test]
fn test_monday() {
    let ruleset = setup_ruleset();

    // Test multiple formats: 星期一, 礼拜一, 周一, 禮拜一, 週一
    let formats = vec!["星期一", "礼拜一", "周一", "禮拜一", "週一"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed to parse: {}", format);

        if let Value::Time(tv) = &values[0] {
            // Should resolve to a Monday
            assert_eq!(tv.instant.weekday(), Weekday::Mon, "Wrong weekday for: {}", format);
        } else {
            panic!("Expected Time value for: {}", format);
        }
    }
}

#[test]
fn test_tuesday() {
    let ruleset = setup_ruleset();
    let formats = vec!["星期二", "礼拜二", "周二", "禮拜二", "週二"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Tue);
        }
    }
}

#[test]
fn test_wednesday() {
    let ruleset = setup_ruleset();
    let formats = vec!["星期三", "礼拜三", "周三", "禮拜三", "週三"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Wed);
        }
    }
}

#[test]
fn test_thursday() {
    let ruleset = setup_ruleset();
    let formats = vec!["星期四", "礼拜四", "周四", "禮拜四", "週四"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Thu);
        }
    }
}

#[test]
fn test_friday() {
    let ruleset = setup_ruleset();
    let formats = vec!["星期五", "礼拜五", "周五", "禮拜五", "週五"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Fri);
        }
    }
}

#[test]
fn test_saturday() {
    let ruleset = setup_ruleset();
    let formats = vec!["星期六", "礼拜六", "周六", "禮拜六", "週六"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Sat);
        }
    }
}

#[test]
fn test_sunday() {
    let ruleset = setup_ruleset();
    // Note: Sunday can be 星期日 or 星期天
    let formats = vec!["星期日", "星期天", "礼拜日", "礼拜天", "周日", "周天", "禮拜日", "禮拜天", "週日", "週天"];

    for format in formats {
        let values = parse(format, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", format);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.instant.weekday(), Weekday::Sun);
        }
    }
}

// ============================================
// Relative Time Periods (6 rules)
// ============================================

#[test]
fn test_last_week() {
    let ruleset = setup_ruleset();

    // 上周, 上週, 上个星期
    let variants = vec!["上周", "上週", "上个星期", "上個星期"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Week);
            // Should be before today
            assert!(tv.instant < Utc::now());
        } else {
            panic!("Expected Time value");
        }
    }
}

#[test]
fn test_next_week() {
    let ruleset = setup_ruleset();

    // 下周, 下週, 下个星期
    let variants = vec!["下周", "下週", "下个星期", "下個星期"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Week);
            // Should be after today
            assert!(tv.instant > Utc::now());
        } else {
            panic!("Expected Time value");
        }
    }
}

#[test]
fn test_last_month() {
    let ruleset = setup_ruleset();

    // 上个月, 上個月
    let variants = vec!["上个月", "上個月"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Month);
            assert!(tv.instant < Utc::now());
        } else {
            panic!("Expected Time value");
        }
    }
}

#[test]
fn test_next_month() {
    let ruleset = setup_ruleset();

    // 下个月, 下個月
    let variants = vec!["下个月", "下個月"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Month);
            assert!(tv.instant > Utc::now());
        } else {
            panic!("Expected Time value");
        }
    }
}

#[test]
fn test_last_year() {
    let ruleset = setup_ruleset();

    // 去年, 上年
    let variants = vec!["去年", "上年"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Year);
            assert!(tv.instant < Utc::now());
            // Should be exactly last year
            let expected_year = Utc::now().year() - 1;
            assert_eq!(tv.instant.year(), expected_year);
        } else {
            panic!("Expected Time value");
        }
    }
}

#[test]
fn test_next_year() {
    let ruleset = setup_ruleset();

    // 明年, 下年
    let variants = vec!["明年", "下年"];

    for variant in variants {
        let values = parse(variant, &ruleset);
        assert_eq!(values.len(), 1, "Failed: {}", variant);

        if let Value::Time(tv) = &values[0] {
            assert_eq!(tv.grain, Grain::Year);
            assert!(tv.instant > Utc::now());
            // Should be exactly next year
            let expected_year = Utc::now().year() + 1;
            assert_eq!(tv.instant.year(), expected_year);
        } else {
            panic!("Expected Time value");
        }
    }
}

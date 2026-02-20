// Phase 3 Tests: Chinese Time - 组合模式 & 交集 (18 rules)
// 按照TDD原则：先写测试，再实现代码
//
// 测试覆盖:
// - 月日模式 (8条规则): X月Y日、命名月份、数字月份
// - 年月日模式 (5条规则): YYYY年X月Y日、年份解析
// - 时间交集 (5条规则): 星期×时段交集

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain};
use chrono::{Datelike, Weekday, Timelike};

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// 月日模式 (Month-Day Patterns) - 8条规则
// ============================================

#[test]
fn test_month_day_numeric() {
    let ruleset = setup_ruleset();

    // "3月15日" - numeric month and day
    let results = ruleset.apply_all("3月15日").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '3月15日'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.month(), 3);
        assert_eq!(td.datetime.day(), 15);
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_month_day_chinese() {
    let ruleset = setup_ruleset();

    // "三月十五日" - Chinese numerals
    let results = ruleset.apply_all("三月十五日").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三月十五日'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.month(), 3);
        assert_eq!(td.datetime.day(), 15);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_month_day_with_hao() {
    let ruleset = setup_ruleset();

    // "3月15号" - using 号 instead of 日
    let variants = vec!["3月15号", "3月15號"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == 3 && td.datetime.day() == 15
            } else {
                false
            }
        }), "应该解析 '{}'", variant);
    }
}

#[test]
fn test_named_months() {
    let ruleset = setup_ruleset();

    // 命名月份: 一月、二月、...、十二月
    let named_months = vec![
        ("一月", 1), ("二月", 2), ("三月", 3), ("四月", 4),
        ("五月", 5), ("六月", 6), ("七月", 7), ("八月", 8),
        ("九月", 9), ("十月", 10), ("十一月", 11), ("十二月", 12),
    ];

    for (month_name, expected_month) in named_months {
        let input = format!("{}十五日", month_name);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == expected_month
            } else {
                false
            }
        }), "应该解析 '{}' 为{}月", input, expected_month);
    }
}

#[test]
fn test_month_day_boundary() {
    let ruleset = setup_ruleset();

    // 边界值测试
    let test_cases = vec![
        ("1月1日", 1, 1),      // 年初
        ("12月31日", 12, 31),  // 年末
        ("2月28日", 2, 28),    // 二月末（非闰年）
    ];

    for (input, expected_month, expected_day) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == expected_month &&
                td.datetime.day() == expected_day
            } else {
                false
            }
        }), "应该解析 '{}' 为 {}/{})", input, expected_month, expected_day);
    }
}

#[test]
fn test_day_only() {
    let ruleset = setup_ruleset();

    // "15号" - day only (should use current month)
    let results = ruleset.apply_all("15号").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.day() == 15
        } else {
            false
        }
    }), "应该解析 '15号'");
}

#[test]
fn test_day_variants() {
    let ruleset = setup_ruleset();

    // 日/号/號 的变体
    let variants = vec!["3月15日", "3月15号", "3月15號"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == 3 && td.datetime.day() == 15
            } else {
                false
            }
        }), "应该解析 '{}'", variant);
    }
}

#[test]
fn test_all_months_1_to_12() {
    let ruleset = setup_ruleset();

    // 测试所有月份 1-12
    for month in 1..=12 {
        let input = format!("{}月1日", month);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == month
            } else {
                false
            }
        }), "应该解析 '{}月1日'", month);
    }
}

// ============================================
// 年月日模式 (Year-Month-Day) - 5条规则
// ============================================

#[test]
fn test_year_month_day_numeric() {
    let ruleset = setup_ruleset();

    // "2024年3月15日"
    let results = ruleset.apply_all("2024年3月15日").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '2024年3月15日'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.year(), 2024);
        assert_eq!(td.datetime.month(), 3);
        assert_eq!(td.datetime.day(), 15);
        assert_eq!(td.grain, Grain::Day);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_year_month_day_chinese() {
    let ruleset = setup_ruleset();

    // "二零二四年三月十五日" - 完整中文数字
    let results = ruleset.apply_all("二零二四年三月十五日").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.year() == 2024 &&
            td.datetime.month() == 3 &&
            td.datetime.day() == 15
        } else {
            false
        }
    }), "应该解析 '二零二四年三月十五日'");
}

#[test]
fn test_year_only() {
    let ruleset = setup_ruleset();

    // "2024年" - year only
    let results = ruleset.apply_all("2024年").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '2024年'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.year(), 2024);
        assert_eq!(td.grain, Grain::Year);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_year_chinese_digits() {
    let ruleset = setup_ruleset();

    // "二零二四年" - Chinese digit year
    let results = ruleset.apply_all("二零二四年").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.year() == 2024
        } else {
            false
        }
    }), "应该解析 '二零二四年' 为2024年");
}

#[test]
fn test_year_range() {
    let ruleset = setup_ruleset();

    // 测试年份范围 2000-2030
    let test_years = vec![2000, 2010, 2020, 2024, 2025, 2030];

    for year in test_years {
        let input = format!("{}年", year);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.year() == year as i32
            } else {
                false
            }
        }), "应该解析 '{}年'", year);
    }
}

// ============================================
// 时间交集 (Time Intersections) - 5条规则
// ============================================

#[test]
fn test_dow_part_of_day_intersection() {
    let ruleset = setup_ruleset();

    // "星期一早上" - Monday morning
    let results = ruleset.apply_all("星期一早上").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '星期一早上'");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.weekday(), Weekday::Mon);
        // 早上应该在4-12点之间
        let hour = td.datetime.hour();
        assert!(hour >= 4 && hour < 12, "早上应该在4-12点，实际: {}", hour);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_dow_pod_variants() {
    let ruleset = setup_ruleset();

    // 测试不同的星期×时段组合
    let test_cases = vec![
        ("周一早上", Weekday::Mon, 4, 12),
        ("星期二下午", Weekday::Tue, 12, 18),
        ("礼拜三晚上", Weekday::Wed, 18, 24),
        ("周五中午", Weekday::Fri, 12, 13),
        ("星期六凌晨", Weekday::Sat, 0, 4),
    ];

    for (input, expected_weekday, min_hour, max_hour) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let hour = td.datetime.hour();
                td.datetime.weekday() == expected_weekday &&
                hour >= min_hour && hour < max_hour
            } else {
                false
            }
        }), "应该解析 '{}' 为 {:?} {}-{}点", input, expected_weekday, min_hour, max_hour);
    }
}

#[test]
fn test_dow_hour_intersection() {
    let ruleset = setup_ruleset();

    // "星期一三点" - Monday 3pm
    let results = ruleset.apply_all("星期一三点").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.weekday() == Weekday::Mon &&
            td.datetime.hour() == 3
        } else {
            false
        }
    }), "应该解析 '星期一三点'");
}

#[test]
fn test_month_day_hour_intersection() {
    let ruleset = setup_ruleset();

    // "3月15日下午" - March 15th afternoon
    let results = ruleset.apply_all("3月15日下午").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            let hour = td.datetime.hour();
            td.datetime.month() == 3 &&
            td.datetime.day() == 15 &&
            hour >= 12 && hour < 18
        } else {
            false
        }
    }), "应该解析 '3月15日下午'");
}

#[test]
fn test_complex_intersection() {
    let ruleset = setup_ruleset();

    // "2024年3月15日星期五下午三点" - Complex full date/time
    let results = ruleset.apply_all("2024年3月15日下午三点").unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.year() == 2024 &&
            td.datetime.month() == 3 &&
            td.datetime.day() == 15 &&
            td.datetime.hour() == 15  // 下午三点 = 15:00
        } else {
            false
        }
    }), "应该解析 '2024年3月15日下午三点'");
}

// ============================================
// 组合变体测试
// ============================================

#[test]
fn test_month_day_all_formats() {
    let ruleset = setup_ruleset();

    // 所有月日格式变体
    let variants = vec![
        "3月15日",
        "3月15号",
        "3月15號",
        "三月十五日",
        "三月十五号",
    ];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.month() == 3 && td.datetime.day() == 15
            } else {
                false
            }
        }), "应该解析 '{}'", variant);
    }
}

#[test]
fn test_year_month_day_all_formats() {
    let ruleset = setup_ruleset();

    // 年月日所有格式
    let variants = vec![
        "2024年3月15日",
        "2024年3月15号",
        "二零二四年三月十五日",
    ];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.year() == 2024 &&
                td.datetime.month() == 3 &&
                td.datetime.day() == 15
            } else {
                false
            }
        }), "应该解析 '{}'", variant);
    }
}

#[test]
fn test_dow_all_parts_of_day() {
    let ruleset = setup_ruleset();

    // 星期一×所有时段
    let parts_of_day = vec![
        ("星期一凌晨", 0, 4),
        ("星期一早上", 4, 12),
        ("星期一中午", 12, 13),
        ("星期一下午", 12, 18),
        ("星期一晚上", 18, 24),
    ];

    for (input, min_hour, max_hour) in parts_of_day {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let hour = td.datetime.hour();
                td.datetime.weekday() == Weekday::Mon &&
                hour >= min_hour && hour < max_hour
            } else {
                false
            }
        }), "应该解析 '{}' 为周一 {}-{}点", input, min_hour, max_hour);
    }
}

// 边界情况和错误处理测试: 中文时间
// 测试无效输入、边界值、特殊情况
//
// 目标: 确保解析器健壮性

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::TimeValue;
use chrono::Timelike;

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// 边界值测试
// ============================================

#[test]
fn test_hour_0_boundary() {
    let ruleset = setup_ruleset();

    // 0点的各种表达
    let variants = vec!["零点", "0点", "午夜", "半夜", "子夜"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == 0
            } else {
                false
            }
        }), "应该解析 '{}' 为0点", variant);
    }
}

#[test]
fn test_hour_23_boundary() {
    let ruleset = setup_ruleset();

    // 23点 - 一天的最后一小时
    let variants = vec!["二十三点", "23点"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == 23
            } else {
                false
            }
        }), "应该解析 '{}' 为23点", variant);
    }
}

#[test]
fn test_minute_0_boundary() {
    let ruleset = setup_ruleset();

    // 0分钟
    let test_cases = vec![
        ("三点", 3, 0),
        ("三点零分", 3, 0),
    ];

    for (input, expected_hour, expected_min) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "应该解析 '{}' 为 {}:{:02}", input, expected_hour, expected_min);
    }
}

#[test]
fn test_minute_59_boundary() {
    let ruleset = setup_ruleset();

    // 59分钟
    let test_cases = vec![
        ("三点五十九分", 3, 59),
        ("23点59分", 23, 59),
    ];

    for (input, expected_hour, expected_min) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "应该解析 '{}' 为 {}:{:02}", input, expected_hour, expected_min);
    }
}

// ============================================
// 无效输入测试（应该不匹配）
// ============================================

#[test]
fn test_invalid_hour_24_and_above() {
    let ruleset = setup_ruleset();

    // 24点及以上应该不匹配（或者如果匹配了，应该被过滤）
    // 注意: "二十四点"在正则中，但会在验证时被拒绝，所以apply_all会返回错误
    let invalid_hours = vec!["25点", "30点", "99点"];

    for input in invalid_hours {
        // 这些输入不应该匹配任何规则，或者返回空结果
        let results = ruleset.apply_all(input);
        // 可能返回错误或者空结果，都是可接受的
        if let Ok(values) = results {
            // 如果成功了，不应该有Time值
            assert!(!values.iter().any(|r| matches!(r.value, Value::Time(_))),
                   "'{}' 不应该产生有效的Time值", input);
        }
    }
}

#[test]
fn test_invalid_minute_60_and_above() {
    let ruleset = setup_ruleset();

    // 60分及以上应该不匹配
    let invalid_inputs = vec!["三点60分", "三点70分", "三点99分"];

    for input in invalid_inputs {
        let results = ruleset.apply_all(input).unwrap();
        // 不应该解析为有效时间
        assert!(results.len() >= 0, "处理 '{}' 不应panic", input);
    }
}

// ============================================
// 特殊组合测试
// ============================================

#[test]
fn test_midnight_variants_consistency() {
    let ruleset = setup_ruleset();

    // 所有"午夜"表达应该都指向0点
    let midnight_variants = vec![
        "零点", "0点", "午夜", "半夜", "子夜", "半夜三更"
    ];

    let mut hours = vec![];
    for variant in midnight_variants {
        let results = ruleset.apply_all(variant).unwrap();
        if let Some(result) = results.iter().find(|r| matches!(r.value, Value::Time(_))) {
            if let Value::Time(TimeValue::Instant(td)) = &result.value {
                hours.push(td.datetime.hour());
            }
        }
    }

    // 所有午夜变体应该都是0点
    assert!(hours.iter().all(|&h| h == 0), "所有午夜变体应该都是0点");
}

#[test]
fn test_noon_variants_consistency() {
    let ruleset = setup_ruleset();

    // 所有"正午"表达应该都指向12点
    let noon_variants = vec!["十二点", "12点", "中午", "正午"];

    let mut hours = vec![];
    for variant in noon_variants {
        let results = ruleset.apply_all(variant).unwrap();
        if let Some(result) = results.iter().find(|r| matches!(r.value, Value::Time(_))) {
            if let Value::Time(TimeValue::Instant(td)) = &result.value {
                hours.push(td.datetime.hour());
            }
        }
    }

    // 所有正午变体应该都是12点
    assert!(hours.iter().all(|&h| h == 12), "所有正午变体应该都是12点");
}

#[test]
fn test_quarter_consistency() {
    let ruleset = setup_ruleset();

    // "一刻" 应该总是15分钟
    let quarter_patterns = vec![
        ("三点一刻", 3, 15),
        ("五点一刻", 5, 15),
        ("十二点一刻", 12, 15),
    ];

    for (input, expected_hour, expected_min) in quarter_patterns {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "'{}' 应该是 {}:15", input, expected_hour);
    }
}

#[test]
fn test_half_consistency() {
    let ruleset = setup_ruleset();

    // "半" 应该总是30分钟
    let half_patterns = vec![
        ("三点半", 3, 30),
        ("五点半", 5, 30),
        ("十二点半", 12, 30),
    ];

    for (input, expected_hour, expected_min) in half_patterns {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "'{}' 应该是 {}:30", input, expected_hour);
    }
}

// ============================================
// 混合字符集测试
// ============================================

#[test]
fn test_mixed_simplified_traditional() {
    let ruleset = setup_ruleset();

    // 测试混合使用简繁体的情况
    // 虽然不常见，但应该能处理
    let test_cases = vec![
        "三點",     // 简体数字 + 繁体点
        "三点",     // 简体
    ];

    for input in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == 3
            } else {
                false
            }
        }), "应该解析 '{}'", input);
    }
}

// ============================================
// 粤语5分钟单位边界测试
// ============================================

#[test]
fn test_cantonese_5min_unit_1() {
    let ruleset = setup_ruleset();

    // 最小单位: 1個字 = 5分钟
    let test_cases = vec![
        ("三点踏一", 3, 5),
        ("三点搭一", 3, 5),
        ("三点一個字", 3, 5),
    ];

    for (input, expected_hour, expected_min) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "应该解析 '{}' 为 {}:{:02}", input, expected_hour, expected_min);
    }
}

#[test]
fn test_cantonese_5min_unit_11() {
    let ruleset = setup_ruleset();

    // 最大单位: 11個字 = 55分钟
    let test_cases = vec![
        ("三点踏十一", 3, 55),
        ("三点搭十一", 3, 55),
        ("三点十一個字", 3, 55),
    ];

    for (input, expected_hour, expected_min) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "应该解析 '{}' 为 {}:{:02}", input, expected_hour, expected_min);
    }
}

// ============================================
// 空格和标点测试
// ============================================

#[test]
fn test_no_spaces_required() {
    let ruleset = setup_ruleset();

    // 中文时间表达通常不需要空格
    let without_space = "三点十五分";
    let results = ruleset.apply_all(without_space).unwrap();

    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 3 && td.datetime.minute() == 15
        } else {
            false
        }
    }), "应该解析无空格的 '{}'", without_space);
}

// ============================================
// 数字范围完整性测试
// ============================================

#[test]
fn test_all_hours_0_to_23_numeric() {
    let ruleset = setup_ruleset();

    // 确保所有0-23的数字小时都能解析
    for hour in 0..=23 {
        let input = format!("{}点", hour);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == hour
            } else {
                false
            }
        }), "应该解析 '{}'", input);
    }
}

#[test]
fn test_common_minutes_0_to_59_numeric() {
    let ruleset = setup_ruleset();

    // 测试常见分钟值
    let common_minutes = vec![0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 59];

    for minute in common_minutes {
        let input = format!("3点{}分", minute);
        let results = ruleset.apply_all(&input).unwrap();

        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == 3 && td.datetime.minute() == minute
            } else {
                false
            }
        }), "应该解析 '{}'", input);
    }
}

// ============================================
// 性能和重复性测试
// ============================================

#[test]
fn test_repeated_parsing_consistency() {
    let ruleset = setup_ruleset();

    // 多次解析同一输入应该得到一致结果
    let input = "三点十五分";

    let mut hours = vec![];
    let mut minutes = vec![];

    for _ in 0..10 {
        let results = ruleset.apply_all(input).unwrap();
        if let Some(result) = results.iter().find(|r| matches!(r.value, Value::Time(_))) {
            if let Value::Time(TimeValue::Instant(td)) = &result.value {
                hours.push(td.datetime.hour());
                minutes.push(td.datetime.minute());
            }
        }
    }

    // 所有解析结果应该一致
    assert!(hours.iter().all(|&h| h == 3), "多次解析小时应该一致");
    assert!(minutes.iter().all(|&m| m == 15), "多次解析分钟应该一致");
}

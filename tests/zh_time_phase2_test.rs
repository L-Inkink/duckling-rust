// Phase 2 Tests: Chinese Time - Hour/Minute Patterns (35 rules)
// 按照TDD原则：先写测试，再实现代码
//
// 测试覆盖:
// - 时段模式 (6条规则): 凌晨、早上、中午、下午、晚上、半夜
// - 小时表达 (12条规则): X点、X点Y分、X点零Y分、X点差Y分、X点一刻、X点半
// - 5分钟单位 (4条规则): 踏/搭、個字
// - 特殊分钟 (13条规则): 刻钟、半点、差模式

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain, Form};
use chrono::{Timelike};

fn setup_ruleset() -> rustling_core::RuleSet<Value> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    zh_time::rules(&b, None);
    b.build()
}

// ============================================
// 时段模式 (Part of Day) - 6条规则
// ============================================

#[test]
fn test_dawn() {
    let ruleset = setup_ruleset();

    // 凌晨 (língchén) - dawn, 0-4am
    let results = ruleset.apply_all("凌晨").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '凌晨' (dawn)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        // 应该在0-4点之间
        let hour = td.datetime.hour();
        assert!(hour >= 0 && hour < 4, "凌晨应该是0-4点，实际: {}", hour);
        assert_eq!(td.form, Form::PartOfDay);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_morning() {
    let ruleset = setup_ruleset();

    // 早上/早晨/朝早 - morning, 4-12
    let variants = vec!["早上", "早晨", "朝早"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let hour = td.datetime.hour();
                hour >= 4 && hour < 12 && td.form == Form::PartOfDay
            } else {
                false
            }
        }), "应该解析 '{}' 为早上 (4-12点)", variant);
    }
}

#[test]
fn test_noon() {
    let ruleset = setup_ruleset();

    // 中午 - noon, 12
    let results = ruleset.apply_all("中午").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '中午' (noon)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 12);
        assert_eq!(td.form, Form::PartOfDay);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_afternoon() {
    let ruleset = setup_ruleset();

    // 下午/晏晝 - afternoon, 12-18
    let variants = vec!["下午", "晏晝"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let hour = td.datetime.hour();
                hour >= 12 && hour < 18 && td.form == Form::PartOfDay
            } else {
                false
            }
        }), "应该解析 '{}' 为下午 (12-18点)", variant);
    }
}

#[test]
fn test_evening() {
    let ruleset = setup_ruleset();

    // 晚上/晚间/夜晚 - evening, 18-24
    let variants = vec!["晚上", "晚间", "夜晚"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                let hour = td.datetime.hour();
                hour >= 18 && hour < 24 && td.form == Form::PartOfDay
            } else {
                false
            }
        }), "应该解析 '{}' 为晚上 (18-24点)", variant);
    }
}

#[test]
fn test_midnight() {
    let ruleset = setup_ruleset();

    // 半夜/午夜/子夜 - midnight, 0
    let variants = vec!["半夜", "午夜", "子夜"];

    for variant in variants {
        let results = ruleset.apply_all(variant).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == 0 && td.form == Form::PartOfDay
            } else {
                false
            }
        }), "应该解析 '{}' 为午夜 (0点)", variant);
    }
}

// ============================================
// 小时表达 - 12条规则
// ============================================

#[test]
fn test_hour_oclock_simplified() {
    let ruleset = setup_ruleset();

    // "三点" - 3 o'clock (simplified)
    let results = ruleset.apply_all("三点").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点' (3 o'clock)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 0);
        assert_eq!(td.grain, Grain::Hour);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_hour_oclock_traditional() {
    let ruleset = setup_ruleset();

    // "五點" - 5 o'clock (traditional)
    let results = ruleset.apply_all("五點").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 5 && td.datetime.minute() == 0
        } else {
            false
        }
    }), "应该解析 '五點' 为5点");
}

#[test]
fn test_hour_minute_basic() {
    let ruleset = setup_ruleset();

    // "三点十五分" - 3:15
    let results = ruleset.apply_all("三点十五分").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点十五分' (3:15)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 15);
        assert_eq!(td.grain, Grain::Minute);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_hour_minute_traditional() {
    let ruleset = setup_ruleset();

    // "七點三十分" - 7:30 (traditional)
    let results = ruleset.apply_all("七點三十分").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 7 && td.datetime.minute() == 30
        } else {
            false
        }
    }), "应该解析 '七點三十分' 为7:30");
}

#[test]
fn test_hour_minute_with_zero() {
    let ruleset = setup_ruleset();

    // "九点零五分" - 9:05 (with 零 for single digit)
    let results = ruleset.apply_all("九点零五分").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '九点零五分' (9:05)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 9);
        assert_eq!(td.datetime.minute(), 5);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_hour_minus_minutes() {
    let ruleset = setup_ruleset();

    // "三点差五分" - 5 minutes before 3 = 2:55
    let results = ruleset.apply_all("三点差五分").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点差五分' (2:55)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 2);
        assert_eq!(td.datetime.minute(), 55);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_hour_quarter() {
    let ruleset = setup_ruleset();

    // "三点一刻" - quarter past 3 = 3:15
    let results = ruleset.apply_all("三点一刻").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点一刻' (3:15)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 15);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_hour_half() {
    let ruleset = setup_ruleset();

    // "三点半" - half past 3 = 3:30
    let results = ruleset.apply_all("三点半").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点半' (3:30)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 30);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_numeric_hour() {
    let ruleset = setup_ruleset();

    // "3点" - 3 o'clock (numeric)
    let results = ruleset.apply_all("3点").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 3 && td.datetime.minute() == 0
        } else {
            false
        }
    }), "应该解析 '3点' 为3点");
}

#[test]
fn test_numeric_hour_minute() {
    let ruleset = setup_ruleset();

    // "15点30分" - 15:30 (numeric)
    let results = ruleset.apply_all("15点30分").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 15 && td.datetime.minute() == 30
        } else {
            false
        }
    }), "应该解析 '15点30分' 为15:30");
}

// ============================================
// 5分钟单位标记 (粤语) - 4条规则
// ============================================

#[test]
fn test_cantonese_tap_units() {
    let ruleset = setup_ruleset();

    // "三点踏五" - 3:25 (3 + 5×5 minutes)
    let results = ruleset.apply_all("三点踏五").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点踏五' (3:25)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 25); // 5 * 5 = 25
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_cantonese_dap_units() {
    let ruleset = setup_ruleset();

    // "五点搭七" - 5:35 (5 + 7×5 minutes)
    let results = ruleset.apply_all("五点搭七").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 5 && td.datetime.minute() == 35
        } else {
            false
        }
    }), "应该解析 '五点搭七' 为5:35");
}

#[test]
fn test_cantonese_zi_units() {
    let ruleset = setup_ruleset();

    // "三点五個字" - 3:25 (5 × 5 minutes)
    let results = ruleset.apply_all("三点五個字").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '三点五個字' (3:25)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 3);
        assert_eq!(td.datetime.minute(), 25);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_cantonese_zi_range() {
    let ruleset = setup_ruleset();

    // Test range 1-11 (valid 5-minute units)
    // "十点一個字" = 10:05, "十点十一個字" = 10:55
    let test_cases = vec![
        ("十点一個字", 10, 5),
        ("十点三個字", 10, 15),
        ("十点六個字", 10, 30),
        ("十点十一個字", 10, 55),
    ];

    for (input, expected_hour, expected_min) in test_cases {
        let results = ruleset.apply_all(input).unwrap();
        assert!(results.iter().any(|r| {
            if let Value::Time(TimeValue::Instant(td)) = &r.value {
                td.datetime.hour() == expected_hour && td.datetime.minute() == expected_min
            } else {
                false
            }
        }), "应该解析 '{}' 为{}:{:02}", input, expected_hour, expected_min);
    }
}

// ============================================
// 特殊分钟表达 - 13条规则
// ============================================

#[test]
fn test_standalone_quarter() {
    let ruleset = setup_ruleset();

    // "一刻" - 15 minutes (standalone)
    let results = ruleset.apply_all("一刻").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '一刻' (15 minutes)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.minute(), 15);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_three_quarters() {
    let ruleset = setup_ruleset();

    // "三刻" - 45 minutes
    let results = ruleset.apply_all("三刻").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.minute() == 45
        } else {
            false
        }
    }), "应该解析 '三刻' 为45分钟");
}

#[test]
fn test_half_hour_standalone() {
    let ruleset = setup_ruleset();

    // "半小时" - 30 minutes
    let results = ruleset.apply_all("半小时").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '半小时' (30 minutes)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.minute(), 30);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_noon_special() {
    let ruleset = setup_ruleset();

    // "正午" - exactly noon (12:00)
    let results = ruleset.apply_all("正午").unwrap();
    let time_result = results.iter().find(|r| matches!(r.value, Value::Time(_)))
        .expect("应该解析 '正午' (12:00)");

    if let Value::Time(TimeValue::Instant(td)) = &time_result.value {
        assert_eq!(td.datetime.hour(), 12);
        assert_eq!(td.datetime.minute(), 0);
    } else {
        panic!("期望 Instant TimeValue");
    }
}

#[test]
fn test_midnight_special() {
    let ruleset = setup_ruleset();

    // "半夜三更" - middle of night (0:00)
    let results = ruleset.apply_all("半夜三更").unwrap();
    assert!(results.iter().any(|r| {
        if let Value::Time(TimeValue::Instant(td)) = &r.value {
            td.datetime.hour() == 0
        } else {
            false
        }
    }), "应该解析 '半夜三更' 为午夜");
}

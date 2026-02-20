//! 中文节假日集成测试
//!
//! 测试节假日规则的实际解析功能

use rustling::values::Value;
use rustling::languages::zh::time as zh_time;
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
    zh_time::rules(&b, ctx);
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
// 固定日期节假日测试
// ============================================================

#[test]
fn test_new_year() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "元旦");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "元旦节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()));
}

#[test]
fn test_national_day() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 9, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "国庆");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "国庆节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap()));
}

#[test]
fn test_labor_day() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 4, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "劳动节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "五一");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap()));
}

#[test]
fn test_childrens_day() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "儿童节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "六一");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()));
}

#[test]
fn test_womens_day() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "妇女节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 3, 8, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "三八");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 3, 8, 0, 0, 0).unwrap()));
}

// ============================================================
// 农历节假日测试
// ============================================================

#[test]
fn test_spring_festival_2024() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "春节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "农历新年");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "新春");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));
}

#[test]
fn test_dragon_boat_2024() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "端午节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 6, 10, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "端午");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 6, 10, 0, 0, 0).unwrap()));
}

#[test]
fn test_mid_autumn_2024() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 9, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "中秋节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 9, 17, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "中秋");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 9, 17, 0, 0, 0).unwrap()));
}

// ============================================================
// 节气节假日测试
// ============================================================

#[test]
fn test_qingming_2024() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 4, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "清明节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 4, 4, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "清明");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 4, 4, 0, 0, 0).unwrap()));
}

// ============================================================
// 国际节假日测试
// ============================================================

#[test]
fn test_valentines_day() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "情人节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 14, 0, 0, 0).unwrap()));
}

#[test]
fn test_christmas() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 12, 1, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "圣诞节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 12, 25, 0, 0, 0).unwrap()));

    let result = parse_and_get_datetime(&ruleset, "圣诞");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 12, 25, 0, 0, 0).unwrap()));
}

// ============================================================
// 跨年测试
// ============================================================

#[test]
fn test_holidays_across_years() {
    // 2024年的春节
    let ruleset_2024 = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()));
    let result = parse_and_get_datetime(&ruleset_2024, "春节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));

    // 2025年的春节（不同日期）
    let ruleset_2025 = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()));
    let result = parse_and_get_datetime(&ruleset_2025, "春节");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2025, 1, 29, 0, 0, 0).unwrap()));
}

// ============================================================
// 繁体字测试
// ============================================================

#[test]
fn test_traditional_chinese() {
    let ruleset = setup_ruleset_with_context(Some(Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap()));

    // 元旦
    let result = parse_and_get_datetime(&ruleset, "元旦節");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()));

    // 春节
    let result = parse_and_get_datetime(&ruleset, "春節");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));

    // 农历新年
    let result = parse_and_get_datetime(&ruleset, "農曆新年");
    assert_eq!(result, Some(Utc.with_ymd_and_hms(2024, 2, 10, 0, 0, 0).unwrap()));
}

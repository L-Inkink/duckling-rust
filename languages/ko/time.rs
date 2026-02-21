// Korean Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Korean Time rules
///
/// # Parameters
/// - `b`: RuleSetBuilder for registering rules
/// - `context`: Optional TimeContext for reference time
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));

    // ========================================
    // Instants (now, today, etc.)
    // ========================================
    _generate_instant_rules(b, ctx.clone());

    // ========================================
    // Days of Week
    // ========================================
    _generate_dow_rules(b, ctx.clone());

    // ========================================
    // Months
    // ========================================
    _generate_month_rules(b, ctx.clone());

    // ========================================
    // Simple Time References
    // ========================================
    _generate_simple_rules(b, ctx);
}

fn _generate_instant_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // ========================================
    // Instant patterns from Korean Duckling
    // ========================================

    // "지금" - now
    let ctx_igeum = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:igeum",
        b.reg(r"지금|지금").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_igeum.reference_utc(), Grain::Second)))
    );

    // "오늘" - today
    let ctx_oneul = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:oneul",
        b.reg(r"오늘|올").unwrap(),
        move |_| {
            let ref_time = ctx_oneul.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "내일" - tomorrow
    let ctx_naeil = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:naeil",
        b.reg(r"내일|내알").unwrap(),
        move |_| {
            let ref_time = ctx_naeil.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "어제" / "어저께" - yesterday
    let ctx_eoje = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:eoje",
        b.reg(r"어제|어저께|어제").unwrap(),
        move |_| {
            let ref_time = ctx_eoje.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "모레" - day after tomorrow
    let ctx_more = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:more",
        b.reg(r"모레").unwrap(),
        move |_| {
            let ref_time = ctx_more.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "그저께" / "어물" - day before yesterday
    let ctx_geujeo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:geujeo",
        b.reg(r"그저께|어물").unwrap(),
        move |_| {
            let ref_time = ctx_geujeo.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Korean Duckling:
    // - 월요일: 월요일|월\\.?...
    // - 화요일: 화요일|화\\.?...
    // - 수요일: 수요일|수\\.?...
    // - 목요일: 목요일|목\\.?...
    // - 금요일: 금요일|금\\.?...
    // - 토요일: 토요일|토\\.?...
    // - 일요일: 일요일|일\\.?...

    // 월요일 (Monday)
    let ctx_wol = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:wol",
        b.reg(r"월요일|월요|월").unwrap(),
        move |_| {
            let local_ref = ctx_wol.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 화요일 (Tuesday)
    let ctx_hwa = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:hwa",
        b.reg(r"화요일|화요|화").unwrap(),
        move |_| {
            let local_ref = ctx_hwa.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 수요일 (Wednesday)
    let ctx_su = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:su",
        b.reg(r"수요일|수요|수").unwrap(),
        move |_| {
            let local_ref = ctx_su.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 목요일 (Thursday)
    let ctx_mok = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:mok",
        b.reg(r"목요일|목요|목").unwrap(),
        move |_| {
            let local_ref = ctx_mok.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 금요일 (Friday)
    let ctx_geum = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:geum",
        b.reg(r"금요일|금요|금").unwrap(),
        move |_| {
            let local_ref = ctx_geum.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 토요일 (Saturday)
    let ctx_to = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:to",
        b.reg(r"토요일|토요|토").unwrap(),
        move |_| {
            let local_ref = ctx_to.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 일요일 (Sunday)
    let ctx_il = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:dow:il",
        b.reg(r"일요일|일요|일").unwrap(),
        move |_| {
            let local_ref = ctx_il.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::Sun;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );
}

fn _generate_month_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Month patterns from Korean Duckling:
    // - 1월: 1월|1월...
    // - 2월: 2월|2월...
    // - 3월: 3월|3월...
    // - 4월: 4월|4월...
    // - 5월: 5월|5월...
    // - 6월: 6월|6월...
    // - 7월: 7월|7월...
    // - 8월: 8월|8월...
    // - 9월: 9월|9월...
    // - 10월: 10월|10월...
    // - 11월: 11월|11월...
    // - 12월: 12월|12월...

    // 1월 (January)
    let ctx_1 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:1",
        b.reg(r"1월|일월").unwrap(),
        move |_| {
            let ref_time = ctx_1.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 2월 (February)
    let ctx_2 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:2",
        b.reg(r"2월|이월").unwrap(),
        move |_| {
            let ref_time = ctx_2.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 3월 (March)
    let ctx_3 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:3",
        b.reg(r"3월|삼월").unwrap(),
        move |_| {
            let ref_time = ctx_3.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 4월 (April)
    let ctx_4 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:4",
        b.reg(r"4월|사월").unwrap(),
        move |_| {
            let ref_time = ctx_4.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 5월 (May)
    let ctx_5 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:5",
        b.reg(r"5월|오월").unwrap(),
        move |_| {
            let ref_time = ctx_5.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 6월 (June)
    let ctx_6 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:6",
        b.reg(r"6월|유월").unwrap(),
        move |_| {
            let ref_time = ctx_6.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 7월 (July)
    let ctx_7 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:7",
        b.reg(r"7월|칠월").unwrap(),
        move |_| {
            let ref_time = ctx_7.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 8월 (August)
    let ctx_8 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:8",
        b.reg(r"8월|팔월").unwrap(),
        move |_| {
            let ref_time = ctx_8.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 9월 (September)
    let ctx_9 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:9",
        b.reg(r"9월|구월").unwrap(),
        move |_| {
            let ref_time = ctx_9.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 10월 (October)
    let ctx_10 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:10",
        b.reg(r"10월|시월").unwrap(),
        move |_| {
            let ref_time = ctx_10.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 11월 (November)
    let ctx_11 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:11",
        b.reg(r"11월|십일월").unwrap(),
        move |_| {
            let ref_time = ctx_11.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 12월 (December)
    let ctx_12 = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ko:time:month:12",
        b.reg(r"12월|십이월").unwrap(),
        move |_| {
            let ref_time = ctx_12.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "아침" (morning), "낮" (afternoon), "저녁" (evening), etc.
}

// ========================================
// Helper Functions
// ========================================

/// Helper: intersect two time values
fn intersect(time1: TimeData, time2: TimeData) -> Value {
    // TODO: Implement intersection logic
    Value::Time(TimeValue::instant(time1.datetime, time1.grain))
}

/// Helper: shift time by duration
fn shift(time: TimeData, grain: Grain, offset: i64) -> Value {
    // TODO: Implement shift logic
    let new_time = time.datetime + Duration::days(offset);
    Value::Time(TimeValue::instant(new_time, grain))
}

/// Parse day of week from Korean name
fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "월요일" | "월" => Some(Weekday::Mon),
        "화요일" | "화" => Some(Weekday::Tue),
        "수요일" | "수" => Some(Weekday::Wed),
        "목요일" | "목" => Some(Weekday::Thu),
        "금요일" | "금" => Some(Weekday::Fri),
        "토요일" | "토" => Some(Weekday::Sat),
        "일요일" | "일" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse month from Korean name
fn parse_month(name: &str) -> Option<u32> {
    match name {
        "1월" | "일월" => Some(1),
        "2월" | "이월" => Some(2),
        "3월" | "삼월" => Some(3),
        "4월" | "사월" => Some(4),
        "5월" | "오월" => Some(5),
        "6월" | "유월" => Some(6),
        "7월" | "칠월" => Some(7),
        "8월" | "팔월" => Some(8),
        "9월" | "구월" => Some(9),
        "10월" | "시월" => Some(10),
        "11월" | "십일월" => Some(11),
        "12월" | "십이월" => Some(12),
        _ => None,
    }
}

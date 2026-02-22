// Japanese Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Japanese Time rules
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
    // Instant patterns from Japanese Duckling
    // ========================================

    // "今" - now
    let ctx_ima = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:ima",
        b.reg(r"今").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_ima.reference_utc(), Grain::Second)))
    );

    // "今日" / "きょう" - today
    let ctx_kyou = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:kyou",
        b.reg(r"今日|きょう").unwrap(),
        move |_| {
            let ref_time = ctx_kyou.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "明日" / "あした" - tomorrow
    let ctx_ashita = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:ashita",
        b.reg(r"明日|あした").unwrap(),
        move |_| {
            let ref_time = ctx_ashita.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "昨日" / "きのう" - yesterday
    let ctx_kinou = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:kinou",
        b.reg(r"昨日|きのう").unwrap(),
        move |_| {
            let ref_time = ctx_kinou.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "明後日" / "あさって" - day after tomorrow
    let ctx_asatte = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:asatte",
        b.reg(r"明後日|あさって").unwrap(),
        move |_| {
            let ref_time = ctx_asatte.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "一昨日" / "おととい" - day before yesterday
    let ctx_ototoi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:ototoi",
        b.reg(r"一昨日|おととい").unwrap(),
        move |_| {
            let ref_time = ctx_ototoi.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Japanese Duckling:
    // - 月曜日: 月曜日|げつようび|月\\.?...
    // - 火曜日: 火曜日|かようび|火\\.?...
    // - 水曜日: 水曜日|すいようび|水\\.?...
    // - 木曜日: 木曜日|もくようび|木\\.?...
    // - 金曜日: 金曜日|きんようび|金\\.?...
    // - 土曜日: 土曜日|どようび|土\\.?...
    // - 日曜日: 日曜日|にちようび|日\\.?...

    // 月曜日 (Monday)
    let ctx_getsuyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:getsuyoubi",
        b.reg(r"月曜日|げつようび|月\\.?|月曜日").unwrap(),
        move |_| {
            let local_ref = ctx_getsuyoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 火曜日 (Tuesday)
    let ctx_kayoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:kayoubi",
        b.reg(r"火曜日|かようび|火\\.?|火曜日").unwrap(),
        move |_| {
            let local_ref = ctx_kayoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 水曜日 (Wednesday)
    let ctx_suiyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:suiyoubi",
        b.reg(r"水曜日|すいようび|水\\.?|水曜日").unwrap(),
        move |_| {
            let local_ref = ctx_suiyoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 木曜日 (Thursday)
    let ctx_mokuyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:mokuyoubi",
        b.reg(r"木曜日|もくようび|木\\.?|木曜日").unwrap(),
        move |_| {
            let local_ref = ctx_mokuyoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 金曜日 (Friday)
    let ctx_kinyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:kinyoubi",
        b.reg(r"金曜日|きんようび|金\\.?|金曜日").unwrap(),
        move |_| {
            let local_ref = ctx_kinyoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 土曜日 (Saturday)
    let ctx_doyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:doyoubi",
        b.reg(r"土曜日|どようび|土\\.?|土曜日").unwrap(),
        move |_| {
            let local_ref = ctx_doyoubi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // 日曜日 (Sunday)
    let ctx_nichiyoubi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:dow:nichiyoubi",
        b.reg(r"日曜日|にちようび|日\\.?|日曜日").unwrap(),
        move |_| {
            let local_ref = ctx_nichiyoubi.reference_local();
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
    // Month patterns from Japanese Duckling:
    // - 一月: 一月|1月|01月...
    // - 二月: 二月|2月|02月...
    // - 三月: 三月|3月|03月...
    // - 四月: 四月|4月|04月...
    // - 五月: 五月|5月|05月...
    // - 六月: 六月|6月|06月...
    // - 七月: 七月|7月|07月...
    // - 八月: 八月|8月|08月...
    // - 九月: 九月|9月|09月...
    // - 十月: 十月|10月...
    // - 十一月: 十一月|11月...
    // - 十二月: 十二月|12月...

    // 一月 (January - month 1)
    let ctx_ichigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:ichigatsu",
        b.reg(r"一月|1月|01月").unwrap(),
        move |_| {
            let ref_time = ctx_ichigatsu.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 二月 (February - month 2)
    let ctx_nigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:nigatsu",
        b.reg(r"二月|2月|02月").unwrap(),
        move |_| {
            let ref_time = ctx_nigatsu.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 三月 (March - month 3)
    let ctx_sangatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:sangatsu",
        b.reg(r"三月|3月|03月").unwrap(),
        move |_| {
            let ref_time = ctx_sangatsu.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 四月 (April - month 4)
    let ctx_shigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:shigatsu",
        b.reg(r"四月|4月|04月").unwrap(),
        move |_| {
            let ref_time = ctx_shigatsu.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 五月 (May - month 5)
    let ctx_gogatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:gogatsu",
        b.reg(r"五月|5月|05月").unwrap(),
        move |_| {
            let ref_time = ctx_gogatsu.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 六月 (June - month 6)
    let ctx_rokugatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:rokugatsu",
        b.reg(r"六月|6月|06月").unwrap(),
        move |_| {
            let ref_time = ctx_rokugatsu.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 七月 (July - month 7)
    let ctx_shichigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:shichigatsu",
        b.reg(r"七月|7月|07月").unwrap(),
        move |_| {
            let ref_time = ctx_shichigatsu.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 八月 (August - month 8)
    let ctx_hachigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:hachigatsu",
        b.reg(r"八月|8月|08月").unwrap(),
        move |_| {
            let ref_time = ctx_hachigatsu.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 九月 (September - month 9)
    let ctx_kugatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:kugatsu",
        b.reg(r"九月|9月|09月").unwrap(),
        move |_| {
            let ref_time = ctx_kugatsu.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 十月 (October - month 10)
    let ctx_jungatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:jugatsu",
        b.reg(r"十月|10月").unwrap(),
        move |_| {
            let ref_time = ctx_jungatsu.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 十一月 (November - month 11)
    let ctx_juichigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:juichigatsu",
        b.reg(r"十一月|11月").unwrap(),
        move |_| {
            let ref_time = ctx_juichigatsu.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // 十二月 (December - month 12)
    let ctx_junigatsu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ja:time:month:junigatsu",
        b.reg(r"十二月|12月").unwrap(),
        move |_| {
            let ref_time = ctx_junigatsu.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(_b: &RuleSetBuilder<Value>, _ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "朝", "午後", "夜", etc.
}

// ========================================
// Helper Functions
// ========================================

#[allow(dead_code)]
/// Helper: intersect two time values
fn intersect(time1: TimeData, _time2: TimeData) -> Value {
    // TODO: Implement intersection logic
    Value::Time(TimeValue::instant(time1.datetime, time1.grain))
}

#[allow(dead_code)]
/// Helper: shift time by duration
fn shift(time: TimeData, grain: Grain, offset: i64) -> Value {
    // TODO: Implement shift logic
    let new_time = time.datetime + Duration::days(offset);
    Value::Time(TimeValue::instant(new_time, grain))
}

/// Parse day of week from Japanese name
#[allow(dead_code)]fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "月曜日" => Some(Weekday::Mon),
        "火曜日" => Some(Weekday::Tue),
        "水曜日" => Some(Weekday::Wed),
        "木曜日" => Some(Weekday::Thu),
        "金曜日" => Some(Weekday::Fri),
        "土曜日" => Some(Weekday::Sat),
        "日曜日" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse month from Japanese name
#[allow(dead_code)]fn parse_month(name: &str) -> Option<u32> {
    match name {
        "一月" => Some(1),
        "二月" => Some(2),
        "三月" => Some(3),
        "四月" => Some(4),
        "五月" => Some(5),
        "六月" => Some(6),
        "七月" => Some(7),
        "八月" => Some(8),
        "九月" => Some(9),
        "十月" => Some(10),
        "十一月" => Some(11),
        "十二月" => Some(12),
        _ => None,
    }
}

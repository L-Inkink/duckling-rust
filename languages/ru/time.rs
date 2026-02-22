// Russian Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Russian Time rules
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
    // Instant patterns from Russian Duckling
    // ========================================

    // "сейчас" - now
    let ctx_seychas = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:seychas",
        b.reg(r"сейчас|в\s*данный\s*момент").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_seychas.reference_utc(), Grain::Second)))
    );

    // "сегодня" - today
    let ctx_segodnya = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:segodnya",
        b.reg(r"сегодня").unwrap(),
        move |_| {
            let ref_time = ctx_segodnya.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "завтра" - tomorrow
    let ctx_zavtra = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:zavtra",
        b.reg(r"завтра|завтрашний\s*день").unwrap(),
        move |_| {
            let ref_time = ctx_zavtra.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "вчера" - yesterday
    let ctx_vchera = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:vchera",
        b.reg(r"вчера|вчерашний\s*день").unwrap(),
        move |_| {
            let ref_time = ctx_vchera.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "послезавтра" - day after tomorrow
    let ctx_poslezavtra = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:poslezavtra",
        b.reg(r"послезавтра").unwrap(),
        move |_| {
            let ref_time = ctx_poslezavtra.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "позавчера" - day before yesterday
    let ctx_pozavchera = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:pozavchera",
        b.reg(r"позавчера").unwrap(),
        move |_| {
            let ref_time = ctx_pozavchera.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Russian Duckling:
    // - Понедельник: понедельник|пон?\\.?...
    // - Вторник: вторник|вт\\.?...
    // - Среда: среда|ср\\.?...
    // - Четверг: четверг|чт\\.?...
    // - Пятница: пятница|пт\\.?...
    // - Суббота: суббота|сб\\.?...
    // - Воскресенье: воскресенье|вс\\.?...

    // Понедельник (Monday)
    let ctx_ponedelnik = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:ponedelnik",
        b.reg(r"понедельник|пон?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_ponedelnik.reference_local();
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

    // Вторник (Tuesday)
    let ctx_vtornik = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:vtornik",
        b.reg(r"вторник|вт?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_vtornik.reference_local();
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

    // Среда (Wednesday)
    let ctx_sreda = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:sreda",
        b.reg(r"среда|ср?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sreda.reference_local();
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

    // Четверг (Thursday)
    let ctx_chetverg = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:chetverg",
        b.reg(r"четверг|чт?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_chetverg.reference_local();
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

    // Пятница (Friday)
    let ctx_pyatnica = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:pyatnica",
        b.reg(r"пятница|пт?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_pyatnica.reference_local();
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

    // Суббота (Saturday)
    let ctx_subbota = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:subbota",
        b.reg(r"суббота|сб?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_subbota.reference_local();
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

    // Воскресенье (Sunday)
    let ctx_voskresene = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:dow:voskresene",
        b.reg(r"воскресенье|вс?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_voskresene.reference_local();
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
    // Month patterns from Russian Duckling:
    // - Январь: январь|янв\\.?...
    // - Февраль: февраль|фев\\.?...
    // - Март: март|мар\\.?...
    // - Апрель: апрель|апр\\.?...
    // - Май: май|май\\.?...
    // - Июнь: июнь|июн\\.?...
    // - Июль: июль|июл\\.?...
    // - Август: август|авг\\.?...
    // - Сентябрь: сентябрь|сен\\.?...
    // - Октябрь: октябрь|окт\\.?...
    // - Ноябрь: ноябрь|ноя\\.?...
    // - Декабрь: декабрь|дек\\.?...

    // Январь (month 1)
    let ctx_yangvar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:yangvar",
        b.reg(r"январь|янв?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_yangvar.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Февраль (month 2)
    let ctx_fevral = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:fevral",
        b.reg(r"февраль|фев?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_fevral.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Март (month 3)
    let ctx_mart = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:mart",
        b.reg(r"март|мар?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_mart.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Апрель (month 4)
    let ctx_aprel = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:aprel",
        b.reg(r"апрель|апр?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_aprel.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Май (month 5)
    let ctx_may = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:may",
        b.reg(r"май|май?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_may.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Июнь (month 6)
    let ctx_iyun = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:iyun",
        b.reg(r"июнь|июн?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_iyun.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Июль (month 7)
    let ctx_iyul = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:iyul",
        b.reg(r"июль|июл?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_iyul.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Август (month 8)
    let ctx_avgust = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:avgust",
        b.reg(r"август|авг?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_avgust.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Сентябрь (month 9)
    let ctx_sentyabr = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:sentyabr",
        b.reg(r"сентябрь|сен?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_sentyabr.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Октябрь (month 10)
    let ctx_oktyabr = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:oktyabr",
        b.reg(r"октябрь|окт?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_oktyabr.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ноябрь (month 11)
    let ctx_noyabr = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:noyabr",
        b.reg(r"ноябрь|ноя?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_noyabr.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Декабрь (month 12)
    let ctx_dekabr = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ru:time:month:dekabr",
        b.reg(r"декабрь|дек?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_dekabr.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(_b: &RuleSetBuilder<Value>, _ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "утро", "день", "вечер", "ночь" (morning, day, evening, night)
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

/// Parse day of week from Russian name
#[allow(dead_code)]fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "понедельник" => Some(Weekday::Mon),
        "вторник" => Some(Weekday::Tue),
        "среда" => Some(Weekday::Wed),
        "четверг" => Some(Weekday::Thu),
        "пятница" => Some(Weekday::Fri),
        "суббота" => Some(Weekday::Sat),
        "воскресенье" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse month from Russian name
#[allow(dead_code)]fn parse_month(name: &str) -> Option<u32> {
    match name {
        "январь" => Some(1),
        "февраль" => Some(2),
        "март" => Some(3),
        "апрель" => Some(4),
        "май" => Some(5),
        "июнь" => Some(6),
        "июль" => Some(7),
        "август" => Some(8),
        "сентябрь" => Some(9),
        "октябрь" => Some(10),
        "ноябрь" => Some(11),
        "декабрь" => Some(12),
        _ => None,
    }
}

// Bulgarian Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Bulgarian Time rules
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
    // Instant patterns from Bulgarian Duckling
    // ========================================

    // "сега" - now
    let ctx_sega = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:sega",
        b.reg(r"сега").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_sega.reference_utc(), Grain::Second)))
    );

    // "днес" - today
    let ctx_dnes = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dnes",
        b.reg(r"днес|днеска").unwrap(),
        move |_| {
            let ref_time = ctx_dnes.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "утре" - tomorrow
    let ctx_utre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:utre",
        b.reg(r"утре").unwrap(),
        move |_| {
            let ref_time = ctx_utre.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "вчера" - yesterday
    let ctx_vchera = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:vchera",
        b.reg(r"вчера").unwrap(),
        move |_| {
            let ref_time = ctx_vchera.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "вдругиден" - day after tomorrow
    let ctx_vdrugiden = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:vdrugiden",
        b.reg(r"вдругиден").unwrap(),
        move |_| {
            let ref_time = ctx_vdrugiden.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "завчера" - day before yesterday
    let ctx_zavchera = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:zavchera",
        b.reg(r"завчера").unwrap(),
        move |_| {
            let ref_time = ctx_zavchera.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Bulgarian days of week:
    // - Понеделник: понеделник|пон?\\.?...
    // - Вторник: вторник|вт?\\.?...
    // - Сряда: сряда|сря?\\.?...
    // - Четвъртък: четвъртък|чет?\\.?...
    // - Петък: петък|пет?\\.?...
    // - Събота: събота|съб?\\.?...
    // - Неделя: неделя|нед?\\.?...

    // Понеделник (Monday)
    let ctx_ponedelnik = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:ponedelnik",
        b.reg(r"понеделник|пон").unwrap(),
        move |_| {
            let local_ref = ctx_ponedelnik.reference_local();
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
        "bg:time:dow:vtornik",
        b.reg(r"вторник|вт").unwrap(),
        move |_| {
            let local_ref = ctx_vtornik.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Сряда (Wednesday)
    let ctx_sryada = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:sryada",
        b.reg(r"сряда|сря").unwrap(),
        move |_| {
            let local_ref = ctx_sryada.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Четвъртък (Thursday)
    let ctx_chetvartak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:chetvartak",
        b.reg(r"четвъртък|чет").unwrap(),
        move |_| {
            let local_ref = ctx_chetvartak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Петък (Friday)
    let ctx_petak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:petak",
        b.reg(r"петък|пет").unwrap(),
        move |_| {
            let local_ref = ctx_petak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Събота (Saturday)
    let ctx_subota = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:subota",
        b.reg(r"събота|съб").unwrap(),
        move |_| {
            let local_ref = ctx_subota.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Неделя (Sunday)
    let ctx_nedelq = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:dow:nedelq",
        b.reg(r"неделя|нед").unwrap(),
        move |_| {
            let local_ref = ctx_nedelq.reference_local();
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
    // Bulgarian months:
    // - Януари: януари|ян\\.?...
    // - Февруари: февруари|фев\\.?...
    // - Март: март|мар\\.?...
    // - Април: април|апр\\.?...
    // - Май: май|май?\\.?...
    // - Юни: юни|юни?\\.?...
    // - Юли: юли|юли?\\.?...
    // - Август: август|авг\\.?...
    // - Септември: септември|сеп\\.?...
    // - Октомври: октомври|окт\\.?...
    // - Ноември: ноември|ноем\\.?...
    // - Декември: декември|дек\\.?...

    // Януари (January - month 1)
    let ctx_yanuari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:yanuari",
        b.reg(r"януари|ян").unwrap(),
        move |_| {
            let ref_time = ctx_yanuari.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Февруари (February - month 2)
    let ctx_fevruari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:fevruari",
        b.reg(r"февруари|фев").unwrap(),
        move |_| {
            let ref_time = ctx_fevruari.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Март (March - month 3)
    let ctx_mart = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:mart",
        b.reg(r"март|мар").unwrap(),
        move |_| {
            let ref_time = ctx_mart.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Април (April - month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:april",
        b.reg(r"април|апр").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Май (May - month 5)
    let ctx_mai = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:mai",
        b.reg(r"май").unwrap(),
        move |_| {
            let ref_time = ctx_mai.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Юни (June - month 6)
    let ctx_yuni = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:yuni",
        b.reg(r"юни").unwrap(),
        move |_| {
            let ref_time = ctx_yuni.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Юли (July - month 7)
    let ctx_yuli = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:yuli",
        b.reg(r"юли").unwrap(),
        move |_| {
            let ref_time = ctx_yuli.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Август (August - month 8)
    let ctx_avgust = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:avgust",
        b.reg(r"август|авг").unwrap(),
        move |_| {
            let ref_time = ctx_avgust.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Септември (September - month 9)
    let ctx_septemvri = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:septemvri",
        b.reg(r"септември|сеп").unwrap(),
        move |_| {
            let ref_time = ctx_septemvri.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Октомври (October - month 10)
    let ctx_oktomvri = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:oktomvri",
        b.reg(r"октомври|окт").unwrap(),
        move |_| {
            let ref_time = ctx_oktomvri.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ноември (November - month 11)
    let ctx_noemvri = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:noemvri",
        b.reg(r"ноември|ноем").unwrap(),
        move |_| {
            let ref_time = ctx_noemvri.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Декември (December - month 12)
    let ctx_dekemvri = Arc::clone(&ctx);
    b.rule_1_terminal(
        "bg:time:month:dekemvri",
        b.reg(r"декември|дек").unwrap(),
        move |_| {
            let ref_time = ctx_dekemvri.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(_b: &RuleSetBuilder<Value>, _ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "morning", "afternoon", "evening", etc.
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

#[allow(dead_code)]
/// Parse day of week from English name
fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "Monday" => Some(Weekday::Mon),
        "Tuesday" => Some(Weekday::Tue),
        "Wednesday" => Some(Weekday::Wed),
        "Thursday" => Some(Weekday::Thu),
        "Friday" => Some(Weekday::Fri),
        "Saturday" => Some(Weekday::Sat),
        "Sunday" => Some(Weekday::Sun),
        _ => None,
    }
}

#[allow(dead_code)]
/// Parse month from English name
fn parse_month(name: &str) -> Option<u32> {
    match name {
        "January" => Some(1),
        "February" => Some(2),
        "March" => Some(3),
        "April" => Some(4),
        "May" => Some(5),
        "June" => Some(6),
        "July" => Some(7),
        "August" => Some(8),
        "September" => Some(9),
        "October" => Some(10),
        "November" => Some(11),
        "December" => Some(12),
        _ => None,
    }
}

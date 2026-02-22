// Swedish Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Swedish Time rules
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
    // Instant patterns from Swedish Duckling
    // ========================================

    // "nu" - now
    let ctx_nu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:nu",
        b.reg(r"nu").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_nu.reference_utc(), Grain::Second)))
    );

    // "idag" - today
    let ctx_idag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:idag",
        b.reg(r"idag|dag").unwrap(),
        move |_| {
            let ref_time = ctx_idag.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "imorgon" - tomorrow
    let ctx_imorgon = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:imorgon",
        b.reg(r"i\s*morgon|imorgon").unwrap(),
        move |_| {
            let ref_time = ctx_imorgon.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "igår" - yesterday
    let ctx_igar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:igar",
        b.reg(r"i\s*går|igår").unwrap(),
        move |_| {
            let ref_time = ctx_igar.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "i övermorgon" - day after tomorrow
    let ctx_overmorgon = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:overmorgon",
        b.reg(r"i\s*övermorgon").unwrap(),
        move |_| {
            let ref_time = ctx_overmorgon.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "i förrgår" - day before yesterday
    let ctx_forrgar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:forrgar",
        b.reg(r"i\s*förrgår").unwrap(),
        move |_| {
            let ref_time = ctx_forrgar.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Swedish days of week:
    // - Måndag: måndag|man?\\.?...
    // - Tisdag: tisdag|tis?\\.?...
    // - Onsdag: onsdag|ons?\\.?...
    // - Torsdag: torsdag|tor?\\.?...
    // - Fredag: fredag|fre?\\.?...
    // - Lördag: lördag|lör?\\.?...
    // - Söndag: söndag|sön?\\.?...

    // Måndag (Monday)
    let ctx_mandag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:mandag",
        b.reg(r"måndag|man?").unwrap(),
        move |_| {
            let local_ref = ctx_mandag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Tisdag (Tuesday)
    let ctx_tisdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:tisdag",
        b.reg(r"tisdag|tis?").unwrap(),
        move |_| {
            let local_ref = ctx_tisdag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Onsdag (Wednesday)
    let ctx_onsdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:onsdag",
        b.reg(r"onsdag|ons?").unwrap(),
        move |_| {
            let local_ref = ctx_onsdag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Torsdag (Thursday)
    let ctx_torsdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:torsdag",
        b.reg(r"torsdag|tor?").unwrap(),
        move |_| {
            let local_ref = ctx_torsdag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Fredag (Friday)
    let ctx_fredag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:fredag",
        b.reg(r"fredag|fre?").unwrap(),
        move |_| {
            let local_ref = ctx_fredag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Lördag (Saturday)
    let ctx_lordag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:lordag",
        b.reg(r"lördag|lör?").unwrap(),
        move |_| {
            let local_ref = ctx_lordag.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Söndag (Sunday)
    let ctx_sondag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:dow:sondag",
        b.reg(r"söndag|sön?").unwrap(),
        move |_| {
            let local_ref = ctx_sondag.reference_local();
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
    // Swedish months:
    // - Januari: januari|jan\\.?...
    // - Februari: februari|feb\\.?...
    // - Mars: mars|mar\\.?...
    // - April: april|apr\\.?...
    // - Maj: maj|maj?\\.?...
    // - Juni: juni|jun\\.?...
    // - Juli: juli|jul\\.?...
    // - Augusti: augusti|aug\\.?...
    // - September: september|sept?\\.?...
    // - Oktober: oktober|okt\\.?...
    // - November: november|nov\\.?...
    // - December: december|dec\\.?...

    // Januari (January - month 1)
    let ctx_januari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:januari",
        b.reg(r"januari|jan").unwrap(),
        move |_| {
            let ref_time = ctx_januari.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Februari (February - month 2)
    let ctx_februari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:februari",
        b.reg(r"februari|feb").unwrap(),
        move |_| {
            let ref_time = ctx_februari.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mars (March - month 3)
    let ctx_mars = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:mars",
        b.reg(r"mars|mar").unwrap(),
        move |_| {
            let ref_time = ctx_mars.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // April (April - month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:april",
        b.reg(r"april|apr").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maj (May - month 5)
    let ctx_maj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:maj",
        b.reg(r"maj").unwrap(),
        move |_| {
            let ref_time = ctx_maj.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juni (June - month 6)
    let ctx_juni = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:juni",
        b.reg(r"juni|jun").unwrap(),
        move |_| {
            let ref_time = ctx_juni.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juli (July - month 7)
    let ctx_juli = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:juli",
        b.reg(r"juli|jul").unwrap(),
        move |_| {
            let ref_time = ctx_juli.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Augusti (August - month 8)
    let ctx_augusti = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:augusti",
        b.reg(r"augusti|aug").unwrap(),
        move |_| {
            let ref_time = ctx_augusti.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // September (September - month 9)
    let ctx_september = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:september",
        b.reg(r"september|sept?").unwrap(),
        move |_| {
            let ref_time = ctx_september.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Oktober (October - month 10)
    let ctx_oktober = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:oktober",
        b.reg(r"oktober|okt").unwrap(),
        move |_| {
            let ref_time = ctx_oktober.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // November (November - month 11)
    let ctx_november = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:november",
        b.reg(r"november|nov").unwrap(),
        move |_| {
            let ref_time = ctx_november.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // December (December - month 12)
    let ctx_december = Arc::clone(&ctx);
    b.rule_1_terminal(
        "sv:time:month:december",
        b.reg(r"december|dec").unwrap(),
        move |_| {
            let ref_time = ctx_december.reference_local();
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

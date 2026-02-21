// Danish Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Danish Time rules
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
    // Instant patterns from Danish Duckling
    // ========================================

    // "nu" - now
    let ctx_nu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:nu",
        b.reg(r"nu").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_nu.reference_utc(), Grain::Second)))
    );

    // "i dag" - today
    let ctx_i_dag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:i_dag",
        b.reg(r"i\s*dag|idag").unwrap(),
        move |_| {
            let ref_time = ctx_i_dag.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "i morgen" - tomorrow
    let ctx_i_morgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:i_morgen",
        b.reg(r"i\s*morgen|imorgen").unwrap(),
        move |_| {
            let ref_time = ctx_i_morgen.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "i går" - yesterday
    let ctx_i_gar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:i_gar",
        b.reg(r"i\s*går|igår").unwrap(),
        move |_| {
            let ref_time = ctx_i_gar.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "i overmorgen" - day after tomorrow
    let ctx_overmorgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:overmorgen",
        b.reg(r"i\s*overmorgen").unwrap(),
        move |_| {
            let ref_time = ctx_overmorgen.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "i forgårs" - day before yesterday
    let ctx_forgars = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:forgars",
        b.reg(r"i\s*forgårs").unwrap(),
        move |_| {
            let ref_time = ctx_forgars.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Danish days of week:
    // - Mandag: mandag|man?\\.?...
    // - Tirsdag: tirsdag|tis?\\.?...
    // - Onsdag: onsdag|ons?\\.?...
    // - Torsdag: torsdag|tor?\\.?...
    // - Fredag: fredag|fre?\\.?...
    // - Lørdag: lørdag|lør?\\.?...
    // - Søndag: søndag|søn?\\.?...

    // Mandag (Monday)
    let ctx_mandag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:dow:mandag",
        b.reg(r"mandag|man?").unwrap(),
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

    // Tirsdag (Tuesday)
    let ctx_tirsdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:dow:tirsdag",
        b.reg(r"tirsdag|tis?").unwrap(),
        move |_| {
            let local_ref = ctx_tirsdag.reference_local();
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
        "da:time:dow:onsdag",
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
        "da:time:dow:torsdag",
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
        "da:time:dow:fredag",
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

    // Lørdag (Saturday)
    let ctx_lordag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:dow:lordag",
        b.reg(r"lørdag|lør?").unwrap(),
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

    // Søndag (Sunday)
    let ctx_sondag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:dow:sondag",
        b.reg(r"søndag|søn?").unwrap(),
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
    // Danish months:
    // - Januar: januar|jan\\.?...
    // - Februar: februar|feb\\.?...
    // - Marts: marts|mar\\.?...
    // - April: april|apr\\.?...
    // - Maj: maj|maj?\\.?...
    // - Juni: juni|jun\\.?...
    // - Juli: juli|jul\\.?...
    // - August: august|aug\\.?...
    // - September: september|sept?\\.?...
    // - Oktober: oktober|okt\\.?...
    // - November: november|nov\\.?...
    // - December: december|dec\\.?...

    // Januar (January - month 1)
    let ctx_januar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:januar",
        b.reg(r"januar|jan").unwrap(),
        move |_| {
            let ref_time = ctx_januar.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Februar (February - month 2)
    let ctx_februar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:februar",
        b.reg(r"februar|feb").unwrap(),
        move |_| {
            let ref_time = ctx_februar.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Marts (March - month 3)
    let ctx_marts = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:marts",
        b.reg(r"marts|mar").unwrap(),
        move |_| {
            let ref_time = ctx_marts.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // April (April - month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:april",
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
        "da:time:month:maj",
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
        "da:time:month:juni",
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
        "da:time:month:juli",
        b.reg(r"juli|jul").unwrap(),
        move |_| {
            let ref_time = ctx_juli.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // August (August - month 8)
    let ctx_august = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:august",
        b.reg(r"august|aug").unwrap(),
        move |_| {
            let ref_time = ctx_august.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // September (September - month 9)
    let ctx_september = Arc::clone(&ctx);
    b.rule_1_terminal(
        "da:time:month:september",
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
        "da:time:month:oktober",
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
        "da:time:month:november",
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
        "da:time:month:december",
        b.reg(r"december|dec").unwrap(),
        move |_| {
            let ref_time = ctx_december.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "morning", "afternoon", "evening", etc.
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

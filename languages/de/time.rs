// German Time rules
// Auto-generated from Duckling
// Generated: 2026-02-20T23:57:54.116952

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build German Time rules
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
    // Instant patterns from German Duckling
    // ========================================

    // "jetzt" - now
    let ctx_jetzt = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:jetzt",
        b.reg(r"(genau)? ?jetzt|diesen moment|in diesem moment|gerade eben").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_jetzt.reference_utc(), Grain::Second)))
    );

    // "heute" - today
    let ctx_heute = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:heute",
        b.reg(r"heute").unwrap(),
        move |_| {
            let ref_time = ctx_heute.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "morgen" - tomorrow
    let ctx_morgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:morgen",
        b.reg(r"morgen").unwrap(),
        move |_| {
            let ref_time = ctx_morgen.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "gestern" - yesterday
    let ctx_gestern = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:gestern",
        b.reg(r"gestern").unwrap(),
        move |_| {
            let ref_time = ctx_gestern.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "übermorgen" - day after tomorrow
    let ctx_ubermorgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:ubermorgen",
        b.reg(r"übermorgen").unwrap(),
        move |_| {
            let ref_time = ctx_ubermorgen.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "vorgestern" - day before yesterday
    let ctx_vorgestern = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:vorgestern",
        b.reg(r"vorgestern").unwrap(),
        move |_| {
            let ref_time = ctx_vorgestern.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Duckling:
    // - Montag: montags?|mo\\.?...
    // - Dienstag: die?nstags?|di\\.?...
    // - Mittwoch: mittwochs?|mi\\.?...
    // - Donnerstag: donn?erstags?|do\\.?...
    // - Freitag: freitags?|fr\\.?...
    // - Samstag: samstags?|sonnabends?|sa\\.?...
    // - Sonntag: sonntags?|so\\....

    // Montag
    let ctx_montag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:montag",
        b.reg(r"montags?|mo\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_montag.reference_local();
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

    // Dienstag
    let ctx_dienstag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:dienstag",
        b.reg(r"die?nstags?|di\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_dienstag.reference_local();
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

    // Mittwoch
    let ctx_mittwoch = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:mittwoch",
        b.reg(r"mittwochs?|mi\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_mittwoch.reference_local();
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

    // Donnerstag
    let ctx_donnerstag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:donnerstag",
        b.reg(r"donn?erstags?|do\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_donnerstag.reference_local();
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

    // Freitag
    let ctx_freitag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:freitag",
        b.reg(r"freitags?|fr\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_freitag.reference_local();
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

    // Samstag
    let ctx_samstag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:samstag",
        b.reg(r"samstags?|sonnabends?|sa\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_samstag.reference_local();
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

    // Sonntag
    let ctx_sonntag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:dow:sonntag",
        b.reg(r"sonntags?|so\\.").unwrap(),
        move |_| {
            let local_ref = ctx_sonntag.reference_local();
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
}

fn _generate_month_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Month patterns from Duckling:
    // - Januar: januar|jan\\.?...
    // - Februar: februar|feb\\.?...
    // - Marz: m(ä)rz|m(ä)r\\.?...
    // - April: april|apr\\.?...
    // - Mai: mai\\.?...
    // - Juni: juni|jun\\.?...
    // - Juli: juli|jul\\.?...
    // - August: august|aug\\.?...
    // - September: september|sept?\\.?...
    // - Oktober: oktober|okt\\.?...
    // - November: november|nov\\.?...
    // - Dezember: dezember|dez\\.?...

    // Januar (month 1)
    let ctx_januar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:januar",
        b.reg(r"januar|jan\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_januar.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Februar (month 1)
    let ctx_februar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:februar",
        b.reg(r"februar|feb\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_februar.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Marz (month 1)
    let ctx_marz = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:marz",
        b.reg(r"m(ä)rz|m(ä)r\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_marz.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // April (month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:april",
        b.reg(r"april|apr\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mai (month 1)
    let ctx_mai = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:mai",
        b.reg(r"mai\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_mai.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juni (month 1)
    let ctx_juni = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:juni",
        b.reg(r"juni|jun\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juni.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juli (month 1)
    let ctx_juli = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:juli",
        b.reg(r"juli|jul\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juli.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // August (month 8)
    let ctx_august = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:august",
        b.reg(r"august|aug\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_august.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // September (month 9)
    let ctx_september = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:september",
        b.reg(r"september|sept?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_september.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Oktober (month 1)
    let ctx_oktober = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:oktober",
        b.reg(r"oktober|okt\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_oktober.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // November (month 11)
    let ctx_november = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:november",
        b.reg(r"november|nov\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_november.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Dezember (month 1)
    let ctx_dezember = Arc::clone(&ctx);
    b.rule_1_terminal(
        "de:time:month:dezember",
        b.reg(r"dezember|dez\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_dezember.reference_local();
            let target = ref_time.with_month(1).unwrap();
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

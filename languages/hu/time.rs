// Hungarian Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Hungarian Time rules
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
    // Instant patterns from Hungarian Duckling
    // ========================================

    // "most" - now
    let ctx_most = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:most",
        b.reg(r"most").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_most.reference_utc(), Grain::Second)))
    );

    // "ma" - today
    let ctx_ma = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:ma",
        b.reg(r"ma").unwrap(),
        move |_| {
            let ref_time = ctx_ma.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "holnap" - tomorrow
    let ctx_holnap = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:holnap",
        b.reg(r"holnap").unwrap(),
        move |_| {
            let ref_time = ctx_holnap.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "tegnap" - yesterday
    let ctx_tegnap = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:tegnap",
        b.reg(r"tegnap").unwrap(),
        move |_| {
            let ref_time = ctx_tegnap.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "holnapután" - day after tomorrow
    let ctx_holnaputan = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:holnaputan",
        b.reg(r"holnapután").unwrap(),
        move |_| {
            let ref_time = ctx_holnaputan.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "tegnapelőtt" - day before yesterday
    let ctx_tegnapelott = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:tegnapelott",
        b.reg(r"tegnapelőtt").unwrap(),
        move |_| {
            let ref_time = ctx_tegnapelott.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Hungarian days of week:
    // - Hétfő: hétfő|hét?\\.?...
    // - Kedd: kedd|ked?\\.?...
    // - Szerda: szerda|sze?\\.?...
    // - Csütörtök: csütörtök|csü?\\.?...
    // - Péntek: péntek|pén?\\.?...
    // - Szombat: szombat|szom?\\.?...
    // - Vasárnap: vasárnap|vas?\\.?...

    // Hétfő (Monday)
    let ctx_hetfo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:hetfo",
        b.reg(r"hétfő|hét").unwrap(),
        move |_| {
            let local_ref = ctx_hetfo.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Kedd (Tuesday)
    let ctx_kedd = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:kedd",
        b.reg(r"kedd|ked").unwrap(),
        move |_| {
            let local_ref = ctx_kedd.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Szerda (Wednesday)
    let ctx_szerda = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:szerda",
        b.reg(r"szerda|sze").unwrap(),
        move |_| {
            let local_ref = ctx_szerda.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Csütörtök (Thursday)
    let ctx_csutortok = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:csutortok",
        b.reg(r"csütörtök|csü").unwrap(),
        move |_| {
            let local_ref = ctx_csutortok.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Péntek (Friday)
    let ctx_pentek = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:pentek",
        b.reg(r"péntek|pén").unwrap(),
        move |_| {
            let local_ref = ctx_pentek.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Szombat (Saturday)
    let ctx_szombat = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:szombat",
        b.reg(r"szombat|szom").unwrap(),
        move |_| {
            let local_ref = ctx_szombat.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Vasárnap (Sunday)
    let ctx_vasarnap = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:dow:vasarnap",
        b.reg(r"vasárnap|vas").unwrap(),
        move |_| {
            let local_ref = ctx_vasarnap.reference_local();
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
    // Hungarian months:
    // - Január: január|jan\\.?...
    // - Február: február|feb\\.?...
    // - Március: március|már\\.?...
    // - Április: április|ápr\\.?...
    // - Május: május|máj\\.?...
    // - Június: június|jún\\.?...
    // - Július: július|júl\\.?...
    // - Augusztus: augusztus|aug\\.?...
    // - Szeptember: szeptember|szep\\.?...
    // - Október: október|okt\\.?...
    // - November: november|nov\\.?...
    // - December: december|dec\\.?...

    // Január (January - month 1)
    let ctx_januar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:januar",
        b.reg(r"január|jan").unwrap(),
        move |_| {
            let ref_time = ctx_januar.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Február (February - month 2)
    let ctx_februar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:februar",
        b.reg(r"február|feb").unwrap(),
        move |_| {
            let ref_time = ctx_februar.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Március (March - month 3)
    let ctx_marcius = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:marcius",
        b.reg(r"március|már").unwrap(),
        move |_| {
            let ref_time = ctx_marcius.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Április (April - month 4)
    let ctx_aprilis = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:aprilis",
        b.reg(r"április|ápr").unwrap(),
        move |_| {
            let ref_time = ctx_aprilis.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Május (May - month 5)
    let ctx_majus = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:majus",
        b.reg(r"május|máj").unwrap(),
        move |_| {
            let ref_time = ctx_majus.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Június (June - month 6)
    let ctx_junius = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:junius",
        b.reg(r"június|jún").unwrap(),
        move |_| {
            let ref_time = ctx_junius.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Július (July - month 7)
    let ctx_julius = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:julius",
        b.reg(r"július|júl").unwrap(),
        move |_| {
            let ref_time = ctx_julius.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Augusztus (August - month 8)
    let ctx_augusztus = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:augusztus",
        b.reg(r"augusztus|aug").unwrap(),
        move |_| {
            let ref_time = ctx_augusztus.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Szeptember (September - month 9)
    let ctx_szeptember = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:szeptember",
        b.reg(r"szeptember|szep").unwrap(),
        move |_| {
            let ref_time = ctx_szeptember.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Október (October - month 10)
    let ctx_oktober = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hu:time:month:oktober",
        b.reg(r"október|okt").unwrap(),
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
        "hu:time:month:november",
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
        "hu:time:month:december",
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

// Catalan Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Catalan Time rules
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
    // Instant patterns from Catalan Duckling
    // ========================================

    // "ara" - now
    let ctx_ara = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:ara",
        b.reg(r"ara|ara\s*mateix").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_ara.reference_utc(), Grain::Second)))
    );

    // "avui" - today
    let ctx_avui = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:avui",
        b.reg(r"avui|hui").unwrap(),
        move |_| {
            let ref_time = ctx_avui.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "demà" - tomorrow
    let ctx_dema = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dema",
        b.reg(r"demà|demà\s*passat").unwrap(),
        move |_| {
            let ref_time = ctx_dema.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "ahir" - yesterday
    let ctx_ahir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:ahir",
        b.reg(r"ahir").unwrap(),
        move |_| {
            let ref_time = ctx_ahir.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "demà passat" - day after tomorrow
    let ctx_dema_passat = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dema_passat",
        b.reg(r"demà\s*passat").unwrap(),
        move |_| {
            let ref_time = ctx_dema_passat.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "abans d'ahir" - day before yesterday
    let ctx_abans_ahir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:abans_ahir",
        b.reg(r"abans\s*d'ahir").unwrap(),
        move |_| {
            let ref_time = ctx_abans_ahir.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Catalan days of week:
    // - Dilluns: dilluns|dil?\\.?...
    // - Dimarts: dimarts|dima?\\.?...
    // - Dimecres: dimecres|dimec?\\.?...
    // - Dijous: dijous|dij?\\.?...
    // - Divendres: divendres|div?\\.?...
    // - Dissabte: dissabte diss?\\.?...
    // - Diumenge: diumenge|dium?\\.?...

    // Dilluns (Monday)
    let ctx_dilluns = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:dilluns",
        b.reg(r"dilluns|dil").unwrap(),
        move |_| {
            let local_ref = ctx_dilluns.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dimarts (Tuesday)
    let ctx_dimarts = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:dimarts",
        b.reg(r"dimarts|dima").unwrap(),
        move |_| {
            let local_ref = ctx_dimarts.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dimecres (Wednesday)
    let ctx_dimecres = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:dimecres",
        b.reg(r"dimecres|dimec").unwrap(),
        move |_| {
            let local_ref = ctx_dimecres.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dijous (Thursday)
    let ctx_dijous = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:dijous",
        b.reg(r"dijous|dij").unwrap(),
        move |_| {
            let local_ref = ctx_dijous.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Divendres (Friday)
    let ctx_divendres = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:divendres",
        b.reg(r"divendres|div").unwrap(),
        move |_| {
            let local_ref = ctx_divendres.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dissabte (Saturday)
    let ctx_dissabte = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:dissabte",
        b.reg(r"dissabte|diss").unwrap(),
        move |_| {
            let local_ref = ctx_dissabte.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Diumenge (Sunday)
    let ctx_diumenge = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:dow:diumenge",
        b.reg(r"diumenge|dium").unwrap(),
        move |_| {
            let local_ref = ctx_diumenge.reference_local();
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
    // Catalan months:
    // - Gener: gener|gen\\.?...
    // - Febrer: febrer|feb\\.?...
    // - Març: març|mar\\.?...
    // - Abril: abril|abr\\.?...
    // - Maig: maig|maig?\\.?...
    // - Juny: juny|jun\\.?...
    // - Juliol: juliol|jul\\.?...
    // - Agost: agost|ago\\.?...
    // - Setembre: setembre|set\\.?...
    // - Octubre: octubre|oct\\.?...
    // - Novembre: novembre|nov\\.?...
    // - Desembre: desembre|des\\.?...

    // Gener (January - month 1)
    let ctx_gener = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:gener",
        b.reg(r"gener|gen").unwrap(),
        move |_| {
            let ref_time = ctx_gener.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Febrer (February - month 2)
    let ctx_febrer = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:febrer",
        b.reg(r"febrer|feb").unwrap(),
        move |_| {
            let ref_time = ctx_febrer.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Març (March - month 3)
    let ctx_marc = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:marc",
        b.reg(r"març|mar").unwrap(),
        move |_| {
            let ref_time = ctx_marc.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Abril (April - month 4)
    let ctx_abril = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:abril",
        b.reg(r"abril|abr").unwrap(),
        move |_| {
            let ref_time = ctx_abril.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maig (May - month 5)
    let ctx_maig = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:maig",
        b.reg(r"maig").unwrap(),
        move |_| {
            let ref_time = ctx_maig.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juny (June - month 6)
    let ctx_juny = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:juny",
        b.reg(r"juny|jun").unwrap(),
        move |_| {
            let ref_time = ctx_juny.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juliol (July - month 7)
    let ctx_juliol = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:juliol",
        b.reg(r"juliol|jul").unwrap(),
        move |_| {
            let ref_time = ctx_juliol.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Agost (August - month 8)
    let ctx_agost = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:agost",
        b.reg(r"agost|ago").unwrap(),
        move |_| {
            let ref_time = ctx_agost.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Setembre (September - month 9)
    let ctx_setembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:setembre",
        b.reg(r"setembre|set").unwrap(),
        move |_| {
            let ref_time = ctx_setembre.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Octubre (October - month 10)
    let ctx_octubre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:octubre",
        b.reg(r"octubre|oct").unwrap(),
        move |_| {
            let ref_time = ctx_octubre.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Novembre (November - month 11)
    let ctx_novembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:novembre",
        b.reg(r"novembre|nov").unwrap(),
        move |_| {
            let ref_time = ctx_novembre.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Desembre (December - month 12)
    let ctx_desembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ca:time:month:desembre",
        b.reg(r"desembre|des").unwrap(),
        move |_| {
            let ref_time = ctx_desembre.reference_local();
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

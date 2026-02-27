// French Time rules
// Auto-generated from Duckling
// Generated: 2026-02-20T23:55:00.511514

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build French Time rules
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
    // Instant patterns from French Duckling
    // ========================================

    // "maintenant" - now
    let ctx_maintenant = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:maintenant",
        b.reg(r"maintenant|à l'instant|en ce moment").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_maintenant.reference_utc(), Grain::Second)))
    );

    // "aujourd'hui" - today
    let ctx_aujourd = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:aujourd_hui",
        b.reg(r"aujourd( |-)?hui|ce jour").unwrap(),
        move |_| {
            let ref_time = ctx_aujourd.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "demain" - tomorrow
    let ctx_demain = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:demain",
        b.reg(r"(le )?demain").unwrap(),
        move |_| {
            let ref_time = ctx_demain.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "hier" - yesterday
    let ctx_hier = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:hier",
        b.reg(r"hier|la veille").unwrap(),
        move |_| {
            let ref_time = ctx_hier.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "après-demain" - day after tomorrow
    let ctx_apres_demain = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:apres_demain",
        b.reg(r"apr(è|e)s[- ]?demain").unwrap(),
        move |_| {
            let ref_time = ctx_apres_demain.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "avant-hier" - day before yesterday
    let ctx_avant_hier = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:avant_hier",
        b.reg(r"avant[- ]?hier").unwrap(),
        move |_| {
            let ref_time = ctx_avant_hier.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Duckling:
    // - Lundi: lun\\.?(di)?...
    // - Mardi: mar\\.?(di)?...
    // - Mercredi: mer\\.?(credi)?...
    // - Jeudi: jeu\\.?(di)?...
    // - Vendredi: ven\\.?(dredi)?...
    // - Samedi: sam\\.?(edi)?...
    // - Dimanche: dim\\.?(anche)?...

    // Lundi
    let ctx_lundi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:lundi",
        b.reg(r"lun\\.?(di)?").unwrap(),
        move |_| {
            let local_ref = ctx_lundi.reference_local();
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

    // Mardi
    let ctx_mardi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:mardi",
        b.reg(r"mar\\.?(di)?").unwrap(),
        move |_| {
            let local_ref = ctx_mardi.reference_local();
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

    // Mercredi
    let ctx_mercredi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:mercredi",
        b.reg(r"mer\\.?(credi)?").unwrap(),
        move |_| {
            let local_ref = ctx_mercredi.reference_local();
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

    // Jeudi
    let ctx_jeudi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:jeudi",
        b.reg(r"jeu\\.?(di)?").unwrap(),
        move |_| {
            let local_ref = ctx_jeudi.reference_local();
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

    // Vendredi
    let ctx_vendredi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:vendredi",
        b.reg(r"ven\\.?(dredi)?").unwrap(),
        move |_| {
            let local_ref = ctx_vendredi.reference_local();
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

    // Samedi
    let ctx_samedi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:samedi",
        b.reg(r"sam\\.?(edi)?").unwrap(),
        move |_| {
            let local_ref = ctx_samedi.reference_local();
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

    // Dimanche
    let ctx_dimanche = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:dow:dimanche",
        b.reg(r"dim\\.?(anche)?").unwrap(),
        move |_| {
            let local_ref = ctx_dimanche.reference_local();
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
    // - Janvier: janvier|janv\\.?...
    // - Fevrier: f(é|e)vrier|f(é|e)v\\.?...
    // - Mars: mars|mar\\.?...
    // - Avril: avril|avr\\.?...
    // - Mai: mai...
    // - Juin: juin|jun\\.?...
    // - Juillet: juillet|juil?\\.?...
    // - Aout: ao(û|u)t|aou\\.?...
    // - Septembre: septembre|sept?\\.?...
    // - Octobre: octobre|oct\\.?...
    // - Novembre: novembre|nov\\.?...
    // - Decembre: d(é|e)cembre|d(é|e)c\\.?...

    // Janvier (month 1)
    let ctx_janvier = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:janvier",
        b.reg(r"janvier|janv\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_janvier.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Fevrier (month 1)
    let ctx_fevrier = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:fevrier",
        b.reg(r"f(é|e)vrier|f(é|e)v\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_fevrier.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mars (month 1)
    let ctx_mars = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:mars",
        b.reg(r"mars|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_mars.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Avril (month 1)
    let ctx_avril = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:avril",
        b.reg(r"avril|avr\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_avril.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mai (month 1)
    let ctx_mai = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:mai",
        b.reg(r"mai").unwrap(),
        move |_| {
            let ref_time = ctx_mai.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juin (month 1)
    let ctx_juin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:juin",
        b.reg(r"juin|jun\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juin.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juillet (month 1)
    let ctx_juillet = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:juillet",
        b.reg(r"juillet|juil?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juillet.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Aout (month 1)
    let ctx_aout = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:aout",
        b.reg(r"ao(û|u)t|aou\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_aout.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Septembre (month 1)
    let ctx_septembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:septembre",
        b.reg(r"septembre|sept?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_septembre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Octobre (month 1)
    let ctx_octobre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:octobre",
        b.reg(r"octobre|oct\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_octobre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Novembre (month 1)
    let ctx_novembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:novembre",
        b.reg(r"novembre|nov\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_novembre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Decembre (month 1)
    let ctx_decembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "fr:time:month:decembre",
        b.reg(r"d(é|e)cembre|d(é|e)c\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_decembre.reference_local();
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

// Irish (GA) Time rules
// Auto-generated following Spanish pattern

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Irish Time rules
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
    // Instant patterns from Irish Duckling
    // ========================================

    // "anois" - now
    let ctx_anois = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:anois",
        b.reg(r"anois|faoi\s*lathair").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_anois.reference_utc(), Grain::Second)))
    );

    // "inniu" - today
    let ctx_inniu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:inniu",
        b.reg(r"inniu|anocht").unwrap(),
        move |_| {
            let ref_time = ctx_inniu.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "amárach" - tomorrow
    let ctx_amarach = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:amarach",
        b.reg(r"am(á|a)rach|n(á|a)ire").unwrap(),
        move |_| {
            let ref_time = ctx_amarach.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "inné" - yesterday
    let ctx_inne = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:inne",
        b.reg(r"inn(e|é)|areir").unwrap(),
        move |_| {
            let ref_time = ctx_inne.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "ar náire" - day after tomorrow
    let ctx_ar_naire = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:ar_naire",
        b.reg(r"ar\s*n(á|a)ire").unwrap(),
        move |_| {
            let ref_time = ctx_ar_naire.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "ar éigean" - day before yesterday
    let ctx_ar_eigean = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:ar_eigean",
        b.reg(r"ar\s*(é|e)igean").unwrap(),
        move |_| {
            let ref_time = ctx_ar_eigean.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Irish:
    // - Dé Luain (Monday)
    // - Dé Máirt (Tuesday)
    // - Dé Céadaoin (Wednesday)
    // - Déardaoin (Thursday)
    // - Dé hAoine (Friday)
    // - Dé Sathairn (Saturday)
    // - Dé Domhnaigh (Sunday)

    // Dé Luain (Monday)
    let ctx_luain = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:luain",
        b.reg(r"d(é|e)\s*luain|d(é|e)?\s*l(ú|u)ain?").unwrap(),
        move |_| {
            let local_ref = ctx_luain.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dé Máirt (Tuesday)
    let ctx_mairt = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:mairt",
        b.reg(r"d(é|e)\s*m(á|a)irt|d(é|e)?\s*m(á|a)ir?").unwrap(),
        move |_| {
            let local_ref = ctx_mairt.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dé Céadaoin (Wednesday)
    let ctx_ceadaoin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:ceadaoin",
        b.reg(r"d(é|e)\s*c(é|e)adaoin|d(é|e)?\s*c(é|e)ad").unwrap(),
        move |_| {
            let local_ref = ctx_ceadaoin.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Déardaoin (Thursday)
    let ctx_ardaoin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:ardaoin",
        b.reg(r"d(é|e)ardaoin|d(é|e)?\s*arda").unwrap(),
        move |_| {
            let local_ref = ctx_ardaoin.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dé hAoine (Friday)
    let ctx_ahaoine = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:ahaoine",
        b.reg(r"d(é|e)\s*h?aoin(e|é)|d(é|e)?\s*haoi").unwrap(),
        move |_| {
            let local_ref = ctx_ahaoine.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dé Sathairn (Saturday)
    let ctx_sathairn = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:sathairn",
        b.reg(r"d(é|e)\s*sathairn|d(é|e)?\s*satha").unwrap(),
        move |_| {
            let local_ref = ctx_sathairn.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Dé Domhnaigh (Sunday)
    let ctx_domhnaigh = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:dow:domhnaigh",
        b.reg(r"d(é|e)\s*domhnaigh|d(é|e)?\s*domhn").unwrap(),
        move |_| {
            let local_ref = ctx_domhnaigh.reference_local();
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
    // Month patterns from Irish:
    // - Eanáir (January)
    // - Feabhra (February)
    // - Márta (March)
    // - Aibreán (April)
    // - Bealaine (May)
    // - Meitheamh (June)
    // - Iúil (July)
    // - Lúnasa (August)
    // - Meán Fómhair (September)
    // - Deireadh Fómhair (October)
    // - Samhain (November)
    // - Nollaig (December)

    // Eanáir (January)
    let ctx_eanair = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:eanair",
        b.reg(r"ean(á|a)ir|ean(á|a)?").unwrap(),
        move |_| {
            let ref_time = ctx_eanair.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Feabhra (February)
    let ctx_feabhra = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:feabhra",
        b.reg(r"feabhra|feabh?").unwrap(),
        move |_| {
            let ref_time = ctx_feabhra.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Márta (March)
    let ctx_marta = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:marta",
        b.reg(r"m(á|a)rta|m(á|a)rt?").unwrap(),
        move |_| {
            let ref_time = ctx_marta.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Aibreán (April)
    let ctx_aibrian = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:aibrian",
        b.reg(r"aibre(á|a)n|aibr?").unwrap(),
        move |_| {
            let ref_time = ctx_aibrian.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Bealaine (May)
    let ctx_bealaine = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:bealaine",
        b.reg(r"bealaine|bealain|beal").unwrap(),
        move |_| {
            let ref_time = ctx_bealaine.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Meitheamh (June)
    let ctx_meitheamh = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:meitheamh",
        b.reg(r"meitheamh|meith").unwrap(),
        move |_| {
            let ref_time = ctx_meitheamh.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Iúil (July)
    let ctx_iuil = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:iuil",
        b.reg(r"i(ú|u)il|i(ú|u)?").unwrap(),
        move |_| {
            let ref_time = ctx_iuil.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Lúnasa (August)
    let ctx_lunasa = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:lunasa",
        b.reg(r"l(ú|u)nasa|l(ú|u)nas|l(ú|u)n?").unwrap(),
        move |_| {
            let ref_time = ctx_lunasa.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Meán Fómhair (September)
    let ctx_mean_fomhair = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:mean_fomhair",
        b.reg(r"m(é|e)(á|a)n\s*f(ó|o)mhair|m(é|e)(á|a)n\s*f(ó|o)mh?").unwrap(),
        move |_| {
            let ref_time = ctx_mean_fomhair.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Deireadh Fómhair (October)
    let ctx_deireadh_fomhair = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:deireadh_fomhair",
        b.reg(r"deireadh\s*f(ó|o)mhair|deireadh\s*f(ó|o)mh?").unwrap(),
        move |_| {
            let ref_time = ctx_deireadh_fomhair.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Samhain (November)
    let ctx_samhain = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:samhain",
        b.reg(r"samhain|samh?").unwrap(),
        move |_| {
            let ref_time = ctx_samhain.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Nollaig (December)
    let ctx_nollaig = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ga:time:month:nollaig",
        b.reg(r"nollaig|noll?").unwrap(),
        move |_| {
            let ref_time = ctx_nollaig.reference_local();
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

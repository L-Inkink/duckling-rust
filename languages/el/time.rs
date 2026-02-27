// Greek Time rules
// Based on Spanish pattern

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Greek Time rules
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
    // Instant patterns from Greek
    // ========================================

    // "τώρα" - now
    let ctx_now = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:now",
        b.reg(r"τώρα").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_now.reference_utc(), Grain::Second)))
    );

    // "σήμερα" - today
    let ctx_today = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:today",
        b.reg(r"σήμερα").unwrap(),
        move |_| {
            let ref_time = ctx_today.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "αύριο" - tomorrow
    let ctx_tomorrow = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:tomorrow",
        b.reg(r"αύριο").unwrap(),
        move |_| {
            let ref_time = ctx_tomorrow.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "χθες" - yesterday
    let ctx_yesterday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:yesterday",
        b.reg(r"χθες").unwrap(),
        move |_| {
            let ref_time = ctx_yesterday.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "μεθαύριο" - day after tomorrow
    let ctx_day_after = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:day_after_tomorrow",
        b.reg(r"μεθαύριο").unwrap(),
        move |_| {
            let ref_time = ctx_day_after.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "προχθές" - day before yesterday
    let ctx_day_before = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:day_before_yesterday",
        b.reg(r"προχθές").unwrap(),
        move |_| {
            let ref_time = ctx_day_before.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Greek days of week:
    // - Δευτέρα (Mon)
    // - Τρίτη (Tue)
    // - Τετάρτη (Wed)
    // - Πέμπτη (Thu)
    // - Παρασκευή (Fri)
    // - Σάββατο (Sat)
    // - Κυριακή (Sun)

    // Δευτέρα (Monday)
    let ctx_monday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:monday",
        b.reg(r"Δευτέρα|Δευ").unwrap(),
        move |_| {
            let local_ref = ctx_monday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Τρίτη (Tuesday)
    let ctx_tuesday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:tuesday",
        b.reg(r"Τρίτη|Τρι").unwrap(),
        move |_| {
            let local_ref = ctx_tuesday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Τετάρτη (Wednesday)
    let ctx_wednesday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:wednesday",
        b.reg(r"Τετάρτη|Τετ").unwrap(),
        move |_| {
            let local_ref = ctx_wednesday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Πέμπτη (Thursday)
    let ctx_thursday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:thursday",
        b.reg(r"Πέμπτη|Πεμ").unwrap(),
        move |_| {
            let local_ref = ctx_thursday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Παρασκευή (Friday)
    let ctx_friday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:friday",
        b.reg(r"Παρασκευή|Παρ").unwrap(),
        move |_| {
            let local_ref = ctx_friday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Σάββατο (Saturday)
    let ctx_saturday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:saturday",
        b.reg(r"Σάββατο|Σαβ").unwrap(),
        move |_| {
            let local_ref = ctx_saturday.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Κυριακή (Sunday)
    let ctx_sunday = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:dow:sunday",
        b.reg(r"Κυριακή|Κυρ").unwrap(),
        move |_| {
            let local_ref = ctx_sunday.reference_local();
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
    // Greek months:
    // - Ιανουάριος (January)
    // - Φεβρουάριος (February)
    // - Μάρτιος (March)
    // - Απρίλιος (April)
    // - Μάιος (May)
    // - Ιούνιος (June)
    // - Ιούλιος (July)
    // - Αύγουστος (August)
    // - Σεπτέμβριος (September)
    // - Οκτώβριος (October)
    // - Νοέμβριος (November)
    // - Δεκέμβριος (December)

    // Ιανουάριος (January - month 1)
    let ctx_january = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:january",
        b.reg(r"Ιανουάριος|Ιαν").unwrap(),
        move |_| {
            let ref_time = ctx_january.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Φεβρουάριος (February - month 2)
    let ctx_february = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:february",
        b.reg(r"Φεβρουάριος|Φεβ").unwrap(),
        move |_| {
            let ref_time = ctx_february.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Μάρτιος (March - month 3)
    let ctx_march = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:march",
        b.reg(r"Μάρτιος|Μαρ").unwrap(),
        move |_| {
            let ref_time = ctx_march.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Απρίλιος (April - month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:april",
        b.reg(r"Απρίλιος|Απρ").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Μάιος (May - month 5)
    let ctx_may = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:may",
        b.reg(r"Μάιος|Μαϊ").unwrap(),
        move |_| {
            let ref_time = ctx_may.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ιούνιος (June - month 6)
    let ctx_june = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:june",
        b.reg(r"Ιούνιος").unwrap(),
        move |_| {
            let ref_time = ctx_june.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ιούλιος (July - month 7)
    let ctx_july = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:july",
        b.reg(r"Ιούλιος").unwrap(),
        move |_| {
            let ref_time = ctx_july.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Αύγουστος (August - month 8)
    let ctx_august = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:august",
        b.reg(r"Αύγουστος|Αυγ").unwrap(),
        move |_| {
            let ref_time = ctx_august.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Σεπτέμβριος (September - month 9)
    let ctx_september = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:september",
        b.reg(r"Σεπτέμβριος|Σεπτ").unwrap(),
        move |_| {
            let ref_time = ctx_september.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Οκτώβριος (October - month 10)
    let ctx_october = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:october",
        b.reg(r"Οκτώβριος|Οκτ").unwrap(),
        move |_| {
            let ref_time = ctx_october.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Νοέμβριος (November - month 11)
    let ctx_november = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:november",
        b.reg(r"Νοέμβριος|Νοεμ").unwrap(),
        move |_| {
            let ref_time = ctx_november.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Δεκέμβριος (December - month 12)
    let ctx_december = Arc::clone(&ctx);
    b.rule_1_terminal(
        "el:time:month:december",
        b.reg(r"Δεκέμβριος|Δεκ").unwrap(),
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
    // Examples: "πρωί", "μεσημέρι", "απόγευμα", "βράδυ", etc.
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

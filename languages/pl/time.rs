// Polish Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Polish Time rules
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
    // Instant patterns from Polish Duckling
    // ========================================

    // "teraz" - now
    let ctx_teraz = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:teraz",
        b.reg(r"teraz|\s+teraz").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_teraz.reference_utc(), Grain::Second)))
    );

    // "dzisiaj" / "dziś" - today
    let ctx_dzisiaj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dzisiaj",
        b.reg(r"dzisiaj|dzi(s|ś)").unwrap(),
        move |_| {
            let ref_time = ctx_dzisiaj.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "jutro" - tomorrow
    let ctx_jutro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:jutro",
        b.reg(r"jutro").unwrap(),
        move |_| {
            let ref_time = ctx_jutro.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "wczoraj" - yesterday
    let ctx_wczoraj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:wczoraj",
        b.reg(r"wczoraj").unwrap(),
        move |_| {
            let ref_time = ctx_wczoraj.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "pojutrze" / "po jutra" - day after tomorrow
    let ctx_pojutrze = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:pojutrze",
        b.reg(r"pojutrze|po\s*jutra").unwrap(),
        move |_| {
            let ref_time = ctx_pojutrze.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "przedwczoraj" - day before yesterday
    let ctx_przedwczoraj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:przedwczoraj",
        b.reg(r"przedwczoraj").unwrap(),
        move |_| {
            let ref_time = ctx_przedwczoraj.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Polish:
    // - Poniedziałek: poniedziałek|pon\\.?...
    // - Wtorek: wtorek|wt\\.?...
    // - Środa: środa|śr\\.?...
    // - Czwartek: czwartek|czw\\.?...
    // - Piątek: piątek|pt\\.?...
    // - Sobota: sobota|sob\\.?...
    // - Niedziela: niedziela|nd\\.?...

    // Poniedziałek (Monday)
    let ctx_poniedzialek = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:poniedzialek",
        b.reg(r"poniedziałek|pon\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_poniedzialek.reference_local();
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

    // Wtorek (Tuesday)
    let ctx_wtorek = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:wtorek",
        b.reg(r"wtorek|wt\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_wtorek.reference_local();
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

    // Środa (Wednesday)
    let ctx_sroda = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:sroda",
        b.reg(r"środa|śr\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sroda.reference_local();
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

    // Czwartek (Thursday)
    let ctx_czwartek = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:czwartek",
        b.reg(r"czwartek|czw\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_czwartek.reference_local();
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

    // Piątek (Friday)
    let ctx_piatek = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:piatek",
        b.reg(r"piątek|pt\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_piatek.reference_local();
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

    // Sobota (Saturday)
    let ctx_sobota = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:sobota",
        b.reg(r"sobota|sob\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sobota.reference_local();
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

    // Niedziela (Sunday)
    let ctx_niedziela = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:dow:niedziela",
        b.reg(r"niedziela|nd\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_niedziela.reference_local();
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
    // Month patterns from Polish:
    // - Styczeń: styczeń|sty\\.?...
    // - Luty: luty|lut\\.?...
    // - Marzec: marzec|mar\\.?...
    // - Kwiecień: kwiecień|kwi\\.?...
    // - Maj: maj|maj\\.?...
    // - Czerwiec: czerwiec|cze\\.?...
    // - Lipiec: lipiec|lip\\.?...
    // - Sierpień: sierpień|sie\\.?...
    // - Wrzesień: wrzesień|wrz\\.?...
    // - Październik: październik|paź\\.?...
    // - Listopad: november|lis\\.?...
    // - Grudzień: grudzień|gru\\.?...

    // Styczeń (January - month 1)
    let ctx_styczen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:styczen",
        b.reg(r"styczeń|sty\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_styczen.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Luty (February - month 2)
    let ctx_luty = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:luty",
        b.reg(r"luty|lut\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_luty.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Marzec (March - month 3)
    let ctx_marzec = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:marzec",
        b.reg(r"marzec|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_marzec.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Kwiecień (April - month 4)
    let ctx_kwiecien = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:kwiecien",
        b.reg(r"kwiecień|kwi\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_kwiecien.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maj (May - month 5)
    let ctx_maj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:maj",
        b.reg(r"maj|maj\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_maj.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Czerwiec (June - month 6)
    let ctx_czerwiec = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:czerwiec",
        b.reg(r"czerwiec|cze\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_czerwiec.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Lipiec (July - month 7)
    let ctx_lipiec = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:lipiec",
        b.reg(r"lipiec|lip\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_lipiec.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Sierpień (August - month 8)
    let ctx_sierpien = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:sierpien",
        b.reg(r"sierpień|sie\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_sierpien.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Wrzesień (September - month 9)
    let ctx_wrzesien = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:wrzesien",
        b.reg(r"wrzesień|wrz\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_wrzesien.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Październik (October - month 10)
    let ctx_pazdziernik = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:pazdziernik",
        b.reg(r"październik|paź\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_pazdziernik.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Listopad (November - month 11)
    let ctx_listopad = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:listopad",
        b.reg(r"listopad|lis\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_listopad.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Grudzień (December - month 12)
    let ctx_grudzien = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pl:time:month:grudzien",
        b.reg(r"grudzień|gru\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_grudzien.reference_local();
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

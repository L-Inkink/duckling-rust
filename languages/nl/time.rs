// Dutch Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Dutch Time rules
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
    // Instant patterns from Dutch Duckling
    // ========================================

    // "nu" - now
    let ctx_nu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:nu",
        b.reg(r"nu|op\s*dit\s*moment|op\s*dit\s*ogenblik").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_nu.reference_utc(), Grain::Second)))
    );

    // "vandaag" - today
    let ctx_vandaag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:vandaag",
        b.reg(r"vandaag|van\s*daag").unwrap(),
        move |_| {
            let ref_time = ctx_vandaag.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "morgen" - tomorrow
    let ctx_morgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:morgen",
        b.reg(r"morgen|de\s*dag\s*erna").unwrap(),
        move |_| {
            let ref_time = ctx_morgen.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "gisteren" - yesterday
    let ctx_gisteren = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:gisteren",
        b.reg(r"gisteren|de\s*dag\s*ervoor|de\s*vorige\s*dag").unwrap(),
        move |_| {
            let ref_time = ctx_gisteren.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "overmorgen" - day after tomorrow
    let ctx_overmorgen = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:overmorgen",
        b.reg(r"overmorgen").unwrap(),
        move |_| {
            let ref_time = ctx_overmorgen.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "eergisteren" - day before yesterday
    let ctx_eergisteren = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:eergisteren",
        b.reg(r"eergisteren").unwrap(),
        move |_| {
            let ref_time = ctx_eergisteren.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Dutch Duckling:
    // - Maandag: maandag|ma\.?...
    // - Dinsdag: dinsdag|di\.?...
    // - Woensdag: woensdag|wo\.?...
    // - Donderdag: donderdag|do\.?...
    // - Vrijdag: vrijdag|vr\.?...
    // - Zaterdag: zaterdag|za\.?...
    // - Zondag: zondag|zo\.?...

    // Maandag
    let ctx_maandag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:maandag",
        b.reg(r"maandag|ma\.?").unwrap(),
        move |_| {
            let local_ref = ctx_maandag.reference_local();
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

    // Dinsdag
    let ctx_dinsdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:dinsdag",
        b.reg(r"dinsdag|di\.?").unwrap(),
        move |_| {
            let local_ref = ctx_dinsdag.reference_local();
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

    // Woensdag
    let ctx_woensdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:woensdag",
        b.reg(r"woensdag|wo\.?").unwrap(),
        move |_| {
            let local_ref = ctx_woensdag.reference_local();
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

    // Donderdag
    let ctx_donderdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:donderdag",
        b.reg(r"donderdag|do\.?").unwrap(),
        move |_| {
            let local_ref = ctx_donderdag.reference_local();
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

    // Vrijdag
    let ctx_vrijdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:vrijdag",
        b.reg(r"vrijdag|vr\.?").unwrap(),
        move |_| {
            let local_ref = ctx_vrijdag.reference_local();
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

    // Zaterdag
    let ctx_zaterdag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:zaterdag",
        b.reg(r"zaterdag|za\.?").unwrap(),
        move |_| {
            let local_ref = ctx_zaterdag.reference_local();
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

    // Zondag
    let ctx_zondag = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:dow:zondag",
        b.reg(r"zondag|zo\.?").unwrap(),
        move |_| {
            let local_ref = ctx_zondag.reference_local();
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
    // Month patterns from Dutch Duckling:
    // - Januari: januari|jan\.?...
    // - Februari: februari|feb\.?...
    // - Maart: maart|mar\.?...
    // - April: april|apr\.?...
    // - Mei: mei|mei?\\.?...
    // - Juni: juni|jun\.?...
    // - Juli: juli|jul\.?...
    // - Augustus: augustus|aug\.?...
    // - September: september|sept?\\.?|sep\\.?...
    // - Oktober: oktober|okt\.?...
    // - November: november|nov\.?...
    // - December: december|dec\.?...

    // Januari (month 1)
    let ctx_januari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:januari",
        b.reg(r"januari|jan\.?").unwrap(),
        move |_| {
            let ref_time = ctx_januari.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Februari (month 2)
    let ctx_februari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:februari",
        b.reg(r"februari|feb\.?").unwrap(),
        move |_| {
            let ref_time = ctx_februari.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maart (month 3)
    let ctx_maart = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:maart",
        b.reg(r"maart|mar\.?").unwrap(),
        move |_| {
            let ref_time = ctx_maart.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // April (month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:april",
        b.reg(r"april|apr\.?").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mei (month 5)
    let ctx_mei = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:mei",
        b.reg(r"mei").unwrap(),
        move |_| {
            let ref_time = ctx_mei.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juni (month 6)
    let ctx_juni = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:juni",
        b.reg(r"juni|jun\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juni.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Juli (month 7)
    let ctx_juli = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:juli",
        b.reg(r"juli|jul\.?").unwrap(),
        move |_| {
            let ref_time = ctx_juli.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Augustus (month 8)
    let ctx_augustus = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:augustus",
        b.reg(r"augustus|aug\.?").unwrap(),
        move |_| {
            let ref_time = ctx_augustus.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // September (month 9)
    let ctx_september = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:september",
        b.reg(r"september|sept?\.?|sep\.?").unwrap(),
        move |_| {
            let ref_time = ctx_september.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Oktober (month 10)
    let ctx_oktober = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:oktober",
        b.reg(r"oktober|okt\.?").unwrap(),
        move |_| {
            let ref_time = ctx_oktober.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // November (month 11)
    let ctx_november = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:november",
        b.reg(r"november|nov\.?").unwrap(),
        move |_| {
            let ref_time = ctx_november.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // December (month 12)
    let ctx_december = Arc::clone(&ctx);
    b.rule_1_terminal(
        "nl:time:month:december",
        b.reg(r"december|dec\.?").unwrap(),
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
    // Examples: "ochtend", "middag", "avond", etc.
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

/// Parse day of week from Dutch name
fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "maandag" => Some(Weekday::Mon),
        "dinsdag" => Some(Weekday::Tue),
        "woensdag" => Some(Weekday::Wed),
        "donderdag" => Some(Weekday::Thu),
        "vrijdag" => Some(Weekday::Fri),
        "zaterdag" => Some(Weekday::Sat),
        "zondag" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse month from Dutch name
fn parse_month(name: &str) -> Option<u32> {
    match name {
        "januari" => Some(1),
        "februari" => Some(2),
        "maart" => Some(3),
        "april" => Some(4),
        "mei" => Some(5),
        "juni" => Some(6),
        "juli" => Some(7),
        "augustus" => Some(8),
        "september" => Some(9),
        "oktober" => Some(10),
        "november" => Some(11),
        "december" => Some(12),
        _ => None,
    }
}

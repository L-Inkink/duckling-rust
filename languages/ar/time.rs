// Arabic Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Arabic Time rules
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
    // Instant patterns from Arabic Duckling
    // ========================================

    // "الآن" / "حالا" - now
    let ctx_alaan = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:alaan",
        b.reg(r"الآن|حالا|حالياً").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_alaan.reference_utc(), Grain::Second)))
    );

    // "اليوم" - today
    let ctx_alyawm = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:alyawm",
        b.reg(r"اليوم|يومنا").unwrap(),
        move |_| {
            let ref_time = ctx_alyawm.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "غدا" / "بكرة" - tomorrow
    let ctx_ghadan = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:ghadan",
        b.reg(r"غداً?|بكرة").unwrap(),
        move |_| {
            let ref_time = ctx_ghadan.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "أمس" - yesterday
    let ctx_ams = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:ams",
        b.reg(r"أمس").unwrap(),
        move |_| {
            let ref_time = ctx_ams.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "بعد غد" - day after tomorrow
    let ctx_baad_ghad = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:baad_ghad",
        b.reg(r"بعد\s*غد").unwrap(),
        move |_| {
            let ref_time = ctx_baad_ghad.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "أول أمس" - day before yesterday
    let ctx_awwal_ams = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:awwal_ams",
        b.reg(r"أول\s*أمس").unwrap(),
        move |_| {
            let ref_time = ctx_awwal_ams.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Arabic:
    // - الاثنين / إثنين: Monday
    // - الثلاثاء: Tuesday
    // - الأربعاء: Wednesday
    // - الخميس: Thursday
    // - الجمعة: Friday
    // - السبت: Saturday
    // - الأحد / أحد: Sunday

    // الاثنين (Monday)
    let ctx_ithnayn = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:ithnayn",
        b.reg(r"الاثنين|إثنين").unwrap(),
        move |_| {
            let local_ref = ctx_ithnayn.reference_local();
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

    // الثلاثاء (Tuesday)
    let ctx_thulatha = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:thulatha",
        b.reg(r"الثلاثاء").unwrap(),
        move |_| {
            let local_ref = ctx_thulatha.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // الأربعاء (Wednesday)
    let ctx_arbah = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:arbah",
        b.reg(r"الأربعاء").unwrap(),
        move |_| {
            let local_ref = ctx_arbah.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // الخميس (Thursday)
    let ctx_khamis = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:khamis",
        b.reg(r"الخميس").unwrap(),
        move |_| {
            let local_ref = ctx_khamis.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // الجمعة (Friday)
    let ctx_jumuah = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:jumuah",
        b.reg(r"الجمعة").unwrap(),
        move |_| {
            let local_ref = ctx_jumuah.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // السبت (Saturday)
    let ctx_sabt = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:sabt",
        b.reg(r"السبت").unwrap(),
        move |_| {
            let local_ref = ctx_sabt.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // الأحد / أحد (Sunday)
    let ctx_ahad = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:dow:ahad",
        b.reg(r"الأحد|أحد").unwrap(),
        move |_| {
            let local_ref = ctx_ahad.reference_local();
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
    // Month patterns from Arabic:
    // - يناير: January
    // - فبراير: February
    // - مارس: March
    // - أبريل: April
    // - مايو: May
    // - يونيو: June
    // - يوليو: July
    // - أغسطس: August
    // - سبتمبر: September
    // - أكتوبر: October
    // - نوفمبر: November
    // - ديسمبر: December

    // يناير (January - month 1)
    let ctx_yanair = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:yanair",
        b.reg(r"يناير").unwrap(),
        move |_| {
            let ref_time = ctx_yanair.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // فبراير (February - month 2)
    let ctx_fibrayir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:fibrayir",
        b.reg(r"فبراير").unwrap(),
        move |_| {
            let ref_time = ctx_fibrayir.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // مارس (March - month 3)
    let ctx_maris = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:maris",
        b.reg(r"مارس").unwrap(),
        move |_| {
            let ref_time = ctx_maris.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // أبريل (April - month 4)
    let ctx_april = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:april",
        b.reg(r"أبريل|ابريل").unwrap(),
        move |_| {
            let ref_time = ctx_april.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // مايو (May - month 5)
    let ctx_mayyu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:mayyu",
        b.reg(r"مايو").unwrap(),
        move |_| {
            let ref_time = ctx_mayyu.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // يونيو (June - month 6)
    let ctx_yuniyu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:yuniyu",
        b.reg(r"يونيو").unwrap(),
        move |_| {
            let ref_time = ctx_yuniyu.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // يوليو (July - month 7)
    let ctx_yuliyu = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:yuliyu",
        b.reg(r"يوليو").unwrap(),
        move |_| {
            let ref_time = ctx_yuliyu.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // أغسطس (August - month 8)
    let ctx_agustus = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:agustus",
        b.reg(r"أغسطس").unwrap(),
        move |_| {
            let ref_time = ctx_agustus.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // سبتمبر (September - month 9)
    let ctx_septimbir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:septimbir",
        b.reg(r"سبتمبر").unwrap(),
        move |_| {
            let ref_time = ctx_septimbir.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // أكتوبر (October - month 10)
    let ctx_oktubir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:oktubir",
        b.reg(r"أكتوبر").unwrap(),
        move |_| {
            let ref_time = ctx_oktubir.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // نوفمبر (November - month 11)
    let ctx_nuvimbir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:nuvimbir",
        b.reg(r"نوفمبر").unwrap(),
        move |_| {
            let ref_time = ctx_nuvimbir.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ديسمبر (December - month 12)
    let ctx_dicimbir = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ar:time:month:dicimbir",
        b.reg(r"ديسمبر").unwrap(),
        move |_| {
            let ref_time = ctx_dicimbir.reference_local();
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

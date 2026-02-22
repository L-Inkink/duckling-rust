// Turkish (TR) Time rules
// Auto-generated following Spanish pattern

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Turkish Time rules
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
    // Instant patterns from Turkish Duckling
    // ========================================

    // "şimdi" - now
    let ctx_simdi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:simdi",
        b.reg(r"şimdi|hemen").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_simdi.reference_utc(), Grain::Second)))
    );

    // "bugün" - today
    let ctx_bugun = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:bugun",
        b.reg(r"bugün").unwrap(),
        move |_| {
            let ref_time = ctx_bugun.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "yarın" - tomorrow
    let ctx_yarin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:yarin",
        b.reg(r"yarın").unwrap(),
        move |_| {
            let ref_time = ctx_yarin.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "dün" - yesterday
    let ctx_dun = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dun",
        b.reg(r"dün").unwrap(),
        move |_| {
            let ref_time = ctx_dun.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "öbür gün" - day after tomorrow
    let ctx_obur_gun = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:obur_gun",
        b.reg(r"öbür\s*gün|yarından\s*sonra").unwrap(),
        move |_| {
            let ref_time = ctx_obur_gun.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "evvelsi gün" - day before yesterday
    let ctx_evvelsi_gun = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:evvelsi_gun",
        b.reg(r"evvelsi\s*gün").unwrap(),
        move |_| {
            let ref_time = ctx_evvelsi_gun.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Turkish:
    // - pazartesi (Monday)
    // - salı (Tuesday)
    // - çarşamba (Wednesday)
    // - perşembe (Thursday)
    // - cuma (Friday)
    // - cumartesi (Saturday)
    // - pazar (Sunday)

    // pazartesi (Monday)
    let ctx_pazartesi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:pazartesi",
        b.reg(r"pazartesi|paz\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_pazartesi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // salı (Tuesday)
    let ctx_sali = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:sali",
        b.reg(r"salı|sal\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sali.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // çarşamba (Wednesday)
    let ctx_carsamba = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:carsamba",
        b.reg(r"çarşamba|çar\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_carsamba.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // perşembe (Thursday)
    let ctx_persembe = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:persembe",
        b.reg(r"perşembe|per\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_persembe.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // cuma (Friday)
    let ctx_cuma = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:cuma",
        b.reg(r"cum(a|ı)").unwrap(),
        move |_| {
            let local_ref = ctx_cuma.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // cumartesi (Saturday)
    let ctx_cumartesi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:cumartesi",
        b.reg(r"cumartesi|cum\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_cumartesi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // pazar (Sunday)
    let ctx_pazar = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:dow:pazar",
        b.reg(r"pazar|paz\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_pazar.reference_local();
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
    // Month patterns from Turkish:
    // - ocak (January)
    // - şubat (February)
    // - mart (March)
    // - nisan (April)
    // - mayıs (May)
    // - haziran (June)
    // - temmuz (July)
    // - ağustos (August)
    // - eylül (September)
    // - ekim (October)
    // - kasım (November)
    // - aralık (December)

    // ocak (January)
    let ctx_ocak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:ocak",
        b.reg(r"ocak|oca\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_ocak.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // şubat (February)
    let ctx_subat = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:subat",
        b.reg(r"şubat|subat|sub\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_subat.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // mart (March)
    let ctx_mart = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:mart",
        b.reg(r"mart|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_mart.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // nisan (April)
    let ctx_nisan = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:nisan",
        b.reg(r"nisan|nis\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_nisan.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // mayıs (May)
    let ctx_mayis = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:mayis",
        b.reg(r"mayıs|mayi?").unwrap(),
        move |_| {
            let ref_time = ctx_mayis.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // haziran (June)
    let ctx_haziran = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:haziran",
        b.reg(r"haziran|haz\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_haziran.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // temmuz (July)
    let ctx_temmuz = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:temmuz",
        b.reg(r"temmuz|temm?").unwrap(),
        move |_| {
            let ref_time = ctx_temmuz.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ağustos (August)
    let ctx_agustos = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:agustos",
        b.reg(r"ağustos|agustos|ağus\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_agustos.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // eylül (September)
    let ctx_eylul = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:eylul",
        b.reg(r"eylül|eylul|eyl\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_eylul.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ekim (October)
    let ctx_ekim = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:ekim",
        b.reg(r"ekim|eki\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_ekim.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // kasım (November)
    let ctx_kasim = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:kasim",
        b.reg(r"kasım|kasin|kas\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_kasim.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // aralık (December)
    let ctx_aralik = Arc::clone(&ctx);
    b.rule_1_terminal(
        "tr:time:month:aralik",
        b.reg(r"aralık|aralik|ara\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_aralik.reference_local();
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

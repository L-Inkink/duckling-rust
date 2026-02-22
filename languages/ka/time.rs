// Georgian (KA) Time rules
// Auto-generated following Spanish pattern

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Georgian Time rules
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
    // Instant patterns from Georgian Duckling
    // ========================================

    // "ახლა" - now
    let ctx_akhla = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:akhla",
        b.reg(r"ახლა|ამჟამად").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_akhla.reference_utc(), Grain::Second)))
    );

    // "დღეს" - today
    let ctx_dges = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dges",
        b.reg(r"დღეს").unwrap(),
        move |_| {
            let ref_time = ctx_dges.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "ხვალ" - tomorrow
    let ctx_kval = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:kval",
        b.reg(r"ხვალ|ორშაბათი").unwrap(),
        move |_| {
            let ref_time = ctx_kval.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "გუშინ" - yesterday
    let ctx_gushin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:gushin",
        b.reg(r"გუშინ").unwrap(),
        move |_| {
            let ref_time = ctx_gushin.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "ზეგ" - day after tomorrow
    let ctx_zeg = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:zeg",
        b.reg(r"ზეგ").unwrap(),
        move |_| {
            let ref_time = ctx_zeg.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "გუშინწინ" - day before yesterday
    let ctx_gushintsin = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:gushintsin",
        b.reg(r"გუშინწინ").unwrap(),
        move |_| {
            let ref_time = ctx_gushintsin.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Georgian:
    // - ორშაბათი (Monday)
    // - სამშაბათი (Tuesday)
    // - ოთხშაბათი (Wednesday)
    // - ხუთშაბათი (Thursday)
    // - პარასკევი (Friday)
    // - შაბათი (Saturday)
    // - კვირა (Sunday)

    // ორშაბათი (Monday)
    let ctx_orshabati = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:orshabati",
        b.reg(r"ორშაბათი").unwrap(),
        move |_| {
            let local_ref = ctx_orshabati.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // სამშაბათი (Tuesday)
    let ctx_samshabati = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:samshabati",
        b.reg(r"სამშაბათი").unwrap(),
        move |_| {
            let local_ref = ctx_samshabati.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // ოთხშაბათი (Wednesday)
    let ctx_otkhshabati = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:otkhshabati",
        b.reg(r"ოთხშაბათი").unwrap(),
        move |_| {
            let local_ref = ctx_otkhshabati.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // ხუთშაბათი (Thursday)
    let ctx_khutshabati = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:khutshabati",
        b.reg(r"ხუთშაბათი").unwrap(),
        move |_| {
            let local_ref = ctx_khutshabati.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // პარასკევი (Friday)
    let ctx_paraskevi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:paraskevi",
        b.reg(r"პარასკევი").unwrap(),
        move |_| {
            let local_ref = ctx_paraskevi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // შაბათი (Saturday)
    let ctx_shabati = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:shabati",
        b.reg(r"შაბათი").unwrap(),
        move |_| {
            let local_ref = ctx_shabati.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // კვირა (Sunday)
    let ctx_kvira = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:dow:kvira",
        b.reg(r"კვირა").unwrap(),
        move |_| {
            let local_ref = ctx_kvira.reference_local();
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
    // Month patterns from Georgian:
    // - იანვარი (January)
    // - თებერვალი (February)
    // - მარტი (March)
    // - აპრილი (April)
    // - მაისი (May)
    // - ივნისი (June)
    // - ივლისი (July)
    // - აგვისტო (August)
    // - სექტემბერი (September)
    // - ოქტომბერი (October)
    // - ნოემბერი (November)
    // - დეკემბერი (December)

    // იანვარი (January)
    let ctx_ianvari = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:ianvari",
        b.reg(r"იანვარი").unwrap(),
        move |_| {
            let ref_time = ctx_ianvari.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // თებერვალი (February)
    let ctx_tebervali = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:tebervali",
        b.reg(r"თებერვალი").unwrap(),
        move |_| {
            let ref_time = ctx_tebervali.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // მარტი (March)
    let ctx_marti = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:marti",
        b.reg(r"მარტი").unwrap(),
        move |_| {
            let ref_time = ctx_marti.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // აპრილი (April)
    let ctx_aprili = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:aprili",
        b.reg(r"აპრილი").unwrap(),
        move |_| {
            let ref_time = ctx_aprili.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // მაისი (May)
    let ctx_maisi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:maisi",
        b.reg(r"მაისი").unwrap(),
        move |_| {
            let ref_time = ctx_maisi.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ივნისი (June)
    let ctx_ivnisi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:ivnisi",
        b.reg(r"ივნისი").unwrap(),
        move |_| {
            let ref_time = ctx_ivnisi.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ივლისი (July)
    let ctx_ivlisi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:ivlisi",
        b.reg(r"ივლისი").unwrap(),
        move |_| {
            let ref_time = ctx_ivlisi.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // აგვისტო (August)
    let ctx_agvtisto = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:agvtisto",
        b.reg(r"აგვისტო").unwrap(),
        move |_| {
            let ref_time = ctx_agvtisto.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // სექტემბერი (September)
    let ctx_sektemberi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:sektemberi",
        b.reg(r"სექტემბერი").unwrap(),
        move |_| {
            let ref_time = ctx_sektemberi.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ოქტომბერი (October)
    let ctx_oktomberi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:oktomberi",
        b.reg(r"ოქტომბერი").unwrap(),
        move |_| {
            let ref_time = ctx_oktomberi.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // ნოემბერი (November)
    let ctx_nomberi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:nomberi",
        b.reg(r"ნოემბერი").unwrap(),
        move |_| {
            let ref_time = ctx_nomberi.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // დეკემბერი (December)
    let ctx_dekemberi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "ka:time:month:dekemberi",
        b.reg(r"დეკემბერი").unwrap(),
        move |_| {
            let ref_time = ctx_dekemberi.reference_local();
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

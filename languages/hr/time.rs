// Croatian Time rules
// Auto-generated from Duckling

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Croatian Time rules
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
    // Instant patterns from Croatian Duckling
    // ========================================

    // "sad" - now
    let ctx_sad = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:sad",
        b.reg(r"sad|sada").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_sad.reference_utc(), Grain::Second)))
    );

    // "danas" - today
    let ctx_danas = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:danas",
        b.reg(r"danas").unwrap(),
        move |_| {
            let ref_time = ctx_danas.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "sutra" - tomorrow
    let ctx_sutra = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:sutra",
        b.reg(r"sutra").unwrap(),
        move |_| {
            let ref_time = ctx_sutra.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "jučer" - yesterday
    let ctx_jucer = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:jucer",
        b.reg(r"jučer|juče").unwrap(),
        move |_| {
            let ref_time = ctx_jucer.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "prekosutra" - day after tomorrow
    let ctx_prekosutra = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:prekosutra",
        b.reg(r"prekosutra").unwrap(),
        move |_| {
            let ref_time = ctx_prekosutra.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "prekjučer" - day before yesterday
    let ctx_prekjucer = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:prekjucer",
        b.reg(r"prekjučer").unwrap(),
        move |_| {
            let ref_time = ctx_prekjucer.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Croatian days of week:
    // - Ponedjeljak: ponedjeljak|pon?\\.?...
    // - Utorak: utorak|uto?\\.?...
    // - Srijeda: srijeda|sri?\\.?...
    // - Četvrtak: četvrtak|čet?\\.?...
    // - Petak: petak|pet?\\.?...
    // - Subota: subota|sub?\\.?...
    // - Nedjelja: nedjelja|ned?\\.?...

    // Ponedjeljak (Monday)
    let ctx_ponedjeljak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:ponedjeljak",
        b.reg(r"ponedjeljak|pon").unwrap(),
        move |_| {
            let local_ref = ctx_ponedjeljak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Utorak (Tuesday)
    let ctx_utorak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:utorak",
        b.reg(r"utorak|uto").unwrap(),
        move |_| {
            let local_ref = ctx_utorak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Srijeda (Wednesday)
    let ctx_srijeda = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:srijeda",
        b.reg(r"srijeda|sri").unwrap(),
        move |_| {
            let local_ref = ctx_srijeda.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Četvrtak (Thursday)
    let ctx_cetvrtak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:cetvrtak",
        b.reg(r"četvrtak|čet").unwrap(),
        move |_| {
            let local_ref = ctx_cetvrtak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Petak (Friday)
    let ctx_petak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:petak",
        b.reg(r"petak|pet").unwrap(),
        move |_| {
            let local_ref = ctx_petak.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Subota (Saturday)
    let ctx_subota = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:subota",
        b.reg(r"subota|sub").unwrap(),
        move |_| {
            let local_ref = ctx_subota.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Nedjelja (Sunday)
    let ctx_nedjelja = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:dow:nedjelja",
        b.reg(r"nedjelja|ned").unwrap(),
        move |_| {
            let local_ref = ctx_nedjelja.reference_local();
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
    // Croatian months:
    // - Siječanj: siječanj|sij\\.?...
    // - Veljača: veljača|velj\\.?...
    // - Ožujak: ožujak|ožu\\.?...
    // - Travanj: travanj|trav\\.?...
    // - Svibanj: svibanj|svib\\.?...
    // - Lipanj: lipanj|lip\\.?...
    // - Srpanj: srpanj|srp\\.?...
    // - Kolovoz: kolovoz|kol\\.?...
    // - Rujan: rujan|ruj\\.?...
    // - Listopad: listopad|lis\\.?...
    // - Studeni: studeni|stu\\.?...
    // - Prosinac: prosinac|pro\\.?...

    // Siječanj (January - month 1)
    let ctx_sijecanj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:sijecanj",
        b.reg(r"siječanj|sij").unwrap(),
        move |_| {
            let ref_time = ctx_sijecanj.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Veljača (February - month 2)
    let ctx_veljaca = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:veljaca",
        b.reg(r"veljača|velj").unwrap(),
        move |_| {
            let ref_time = ctx_veljaca.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ožujak (March - month 3)
    let ctx_ozujak = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:ozujak",
        b.reg(r"ožujak|ožu").unwrap(),
        move |_| {
            let ref_time = ctx_ozujak.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Travanj (April - month 4)
    let ctx_travanj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:travanj",
        b.reg(r"travanj|trav").unwrap(),
        move |_| {
            let ref_time = ctx_travanj.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Svibanj (May - month 5)
    let ctx_svibanj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:svibanj",
        b.reg(r"svibanj|svib").unwrap(),
        move |_| {
            let ref_time = ctx_svibanj.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Lipanj (June - month 6)
    let ctx_lipanj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:lipanj",
        b.reg(r"lipanj|lip").unwrap(),
        move |_| {
            let ref_time = ctx_lipanj.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Srpanj (July - month 7)
    let ctx_srpanj = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:srpanj",
        b.reg(r"srpanj|srp").unwrap(),
        move |_| {
            let ref_time = ctx_srpanj.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Kolovoz (August - month 8)
    let ctx_kolovoz = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:kolovoz",
        b.reg(r"kolovoz|kol").unwrap(),
        move |_| {
            let ref_time = ctx_kolovoz.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Rujan (September - month 9)
    let ctx_rujan = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:rujan",
        b.reg(r"rujan|ruj").unwrap(),
        move |_| {
            let ref_time = ctx_rujan.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Listopad (October - month 10)
    let ctx_listopad = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:listopad",
        b.reg(r"listopad|lis").unwrap(),
        move |_| {
            let ref_time = ctx_listopad.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Studeni (November - month 11)
    let ctx_studeni = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:studeni",
        b.reg(r"studeni|stu").unwrap(),
        move |_| {
            let ref_time = ctx_studeni.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Prosinac (December - month 12)
    let ctx_prosinac = Arc::clone(&ctx);
    b.rule_1_terminal(
        "hr:time:month:prosinac",
        b.reg(r"prosinac|pro").unwrap(),
        move |_| {
            let ref_time = ctx_prosinac.reference_local();
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

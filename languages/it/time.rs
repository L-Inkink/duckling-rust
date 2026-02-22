// Italian Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeData, TimeValue};
use rustling_core::RuleSetBuilder;
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use std::sync::Arc;

/// Build Italian Time rules
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
    // Instant patterns from Italian Duckling
    // ========================================

    // "ora" / "adesso" - now
    let ctx_ora = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:ora",
        b.reg(r"ora|adesso|adesso\s*stesso|in\s*questo\s*momento|proprio\s*ora").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_ora.reference_utc(), Grain::Second)))
    );

    // "oggi" - today
    let ctx_oggi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:oggi",
        b.reg(r"oggi|questo\s*giorno|il\s*giorno").unwrap(),
        move |_| {
            let ref_time = ctx_oggi.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "domani" - tomorrow
    let ctx_domani = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:domani",
        b.reg(r"domani|il\s*giorno\s*successivo").unwrap(),
        move |_| {
            let ref_time = ctx_domani.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "ieri" - yesterday
    let ctx_ieri = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:ieri",
        b.reg(r"ieri|il\s*giorno\s*precedente").unwrap(),
        move |_| {
            let ref_time = ctx_ieri.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "dopodomani" - day after tomorrow
    let ctx_dopodomani = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dopodomani",
        b.reg(r"(il\s*giorno\s*)?dopo\s*domani").unwrap(),
        move |_| {
            let ref_time = ctx_dopodomani.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "ieri l'altro" - day before yesterday
    let ctx_ieri_laltro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:ieri_laltro",
        b.reg(r"ieri\s*l'?(altro|ieri)").unwrap(),
        move |_| {
            let ref_time = ctx_ieri_laltro.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Duckling:
    // - Lunedì: luned[ì|i]|lun\\.?...
    // - Martedì: marted[ì|i]|mar\\.?...
    // - Mercoledì: mercoled[ì|i]|mer\\.?...
    // - Giovedì: gioved[ì|i]|gio\\.?...
    // - Venerdì: venerd[ì|i]|ven\\.?...
    // - Sabato: sabato|sab\\.?...
    // - Domenica: domenica|dom\\.?...

    // Lunedì
    let ctx_lunedi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:lunedi",
        b.reg(r"luned[ì|i]|lun\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_lunedi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Mon;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Martedì
    let ctx_martedi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:martedi",
        b.reg(r"marted[ì|i]|mar\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_martedi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Tue;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Mercoledì
    let ctx_mercoledi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:mercoledi",
        b.reg(r"mercoled[ì|i]|mer\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_mercoledi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Wed;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Giovedì
    let ctx_giovedi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:giovedi",
        b.reg(r"gioved[ì|i]|gio\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_giovedi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Thu;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Venerdì
    let ctx_venerdi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:venerdi",
        b.reg(r"venerd[ì|i]|ven\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_venerdi.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Fri;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Sabato
    let ctx_sabato = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:sabato",
        b.reg(r"sabato|sab\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sabato.reference_local();
            let current = local_ref.weekday();
            let target = Weekday::Sat;
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        },
    );

    // Domenica
    let ctx_domenica = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:dow:domenica",
        b.reg(r"domenica|dom\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_domenica.reference_local();
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
    // Month patterns from Duckling:
    // - Gennaio: gennaio|gen\\.?...
    // - Febbraio: febbraio|feb\\.?...
    // - Marzo: marzo|mar\\.?...
    // - Aprile: aprile|apr\\.?...
    // - Maggio: maggio|mag\\.?...
    // - Giugno: giugno|giu\\.?...
    // - Luglio: luglio|lug\\.?...
    // - Agosto: agosto|ago\\.?...
    // - Settembre: settembre|set\\.?...
    // - Ottobre: ottobre|ott\\.?...
    // - Novembre: novembre|nov\\.?...
    // - Dicembre: dicembre|dic\\.?...

    // Gennaio (month 1)
    let ctx_gennaio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:gennaio",
        b.reg(r"gennaio|gen\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_gennaio.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Febbraio (month 2)
    let ctx_febbraio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:febbraio",
        b.reg(r"febbraio|feb\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_febbraio.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Marzo (month 3)
    let ctx_marzo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:marzo",
        b.reg(r"marzo|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_marzo.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Aprile (month 4)
    let ctx_aprile = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:aprile",
        b.reg(r"aprile|apr\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_aprile.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maggio (month 5)
    let ctx_maggio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:maggio",
        b.reg(r"maggio|mag\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_maggio.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Giugno (month 6)
    let ctx_giugno = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:giugno",
        b.reg(r"giugno|giug?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_giugno.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Luglio (month 7)
    let ctx_luglio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:luglio",
        b.reg(r"luglio|lug\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_luglio.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Agosto (month 8)
    let ctx_agosto = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:agosto",
        b.reg(r"agosto|ago\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_agosto.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Settembre (month 9)
    let ctx_settembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:settembre",
        b.reg(r"settembre|sett?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_settembre.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Ottobre (month 10)
    let ctx_ottobre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:ottobre",
        b.reg(r"ottobre|ott\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_ottobre.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Novembre (month 11)
    let ctx_novembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:novembre",
        b.reg(r"novembre|nov\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_novembre.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Dicembre (month 12)
    let ctx_dicembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "it:time:month:dicembre",
        b.reg(r"dicembre|dic\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_dicembre.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(_b: &RuleSetBuilder<Value>, _ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "mattina", "pomeriggio", "sera", etc.
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

/// Parse day of week from Italian name
#[allow(dead_code)]fn parse_dow(name: &str) -> Option<Weekday> {
    match name {
        "lunedì" | "lunedi" => Some(Weekday::Mon),
        "martedì" | "martedi" => Some(Weekday::Tue),
        "mercoledì" | "mercoledi" => Some(Weekday::Wed),
        "giovedì" | "giovedi" => Some(Weekday::Thu),
        "venerdì" | "venerdi" => Some(Weekday::Fri),
        "sabato" => Some(Weekday::Sat),
        "domenica" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse month from Italian name
#[allow(dead_code)]fn parse_month(name: &str) -> Option<u32> {
    match name {
        "gennaio" => Some(1),
        "febbraio" => Some(2),
        "marzo" => Some(3),
        "aprile" => Some(4),
        "maggio" => Some(5),
        "giugno" => Some(6),
        "luglio" => Some(7),
        "agosto" => Some(8),
        "settembre" => Some(9),
        "ottobre" => Some(10),
        "novembre" => Some(11),
        "dicembre" => Some(12),
        _ => None,
    }
}

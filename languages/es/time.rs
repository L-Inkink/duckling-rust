// Spanish Time rules
// Auto-generated from Duckling
// Generated: 2026-02-20T22:55:00.817602

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Spanish Time rules
///
/// # Parameters
/// - `b`: RuleSetBuilder for registering rules
/// - `context`: Optional TimeContext for reference time
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));

    // ========================================
    // Instants (now, today, etc.)
    // ========================================
    self._generate_instant_rules(b, ctx);

    // ========================================
    // Days of Week
    // ========================================
    self._generate_dow_rules(b, ctx);

    // ========================================
    // Months
    // ========================================
    self._generate_month_rules(b, ctx);

    // ========================================
    // Simple Time References
    // ========================================
    self._generate_simple_rules(b, ctx);
}

fn _generate_instant_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // TODO: Add instant patterns (now, today, tomorrow, etc.)
    // Based on Duckling patterns

    // Example pattern:
    // let ctx_now = Arc::clone(&ctx);
    // b.rule_1_terminal(
    //     "es:time:now",
    //     b.reg(r"now|.today").unwrap(),
    //     move |_| Ok(Value::Time(TimeValue::instant(ctx_now.reference_utc(), Grain::Second)))
    // );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Duckling:
    // - Lunes: lunes|lun?\\.?...
    // - Martes: martes|mar?\\.?...
    // - Miercoles: mi(e|é)\\.?(rcoles)?|mx|mier?\\....
    // - Jueves: jueves|jue|jue\\....
    // - Viernes: viernes|vie|vie\\....
    // - Sabado: s(á|a)bado|s(á|a)b\\.?...
    // - Domingo: domingo|dom\\.?...

    // Lunes
    let ctx_lunes = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:lunes",
        b.reg(r"lunes|lun?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_lunes.reference_local();
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

    // Martes
    let ctx_martes = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:martes",
        b.reg(r"martes|mar?\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_martes.reference_local();
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

    // Miercoles
    let ctx_miercoles = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:miercoles",
        b.reg(r"mi(e|é)\\.?(rcoles)?|mx|mier?\\.").unwrap(),
        move |_| {
            let local_ref = ctx_miercoles.reference_local();
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

    // Jueves
    let ctx_jueves = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:jueves",
        b.reg(r"jueves|jue|jue\\.").unwrap(),
        move |_| {
            let local_ref = ctx_jueves.reference_local();
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

    // Viernes
    let ctx_viernes = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:viernes",
        b.reg(r"viernes|vie|vie\\.").unwrap(),
        move |_| {
            let local_ref = ctx_viernes.reference_local();
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

    // Sabado
    let ctx_sabado = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:sabado",
        b.reg(r"s(á|a)bado|s(á|a)b\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sabado.reference_local();
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

    // Domingo
    let ctx_domingo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:dow:domingo",
        b.reg(r"domingo|dom\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_domingo.reference_local();
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
    // - Enero: enero|ene\\.?...
    // - Febrero: febrero|feb\\.?...
    // - Marzo: marzo|mar\\.?...
    // - Abril: abril|abr\\.?...
    // - Mayo: mayo?\\.?...
    // - Junio: junio|jun\\.?...
    // - Julio: julio|jul\\.?...
    // - Agosto: agosto|ago\\.?...
    // - Septiembre: septiembre|sept?\\.?...
    // - Octubre: octubre|oct\\.?...
    // - Noviembre: noviembre|nov\\.?...
    // - Diciembre: diciembre|dic\\.?...

    // Enero (month 1)
    let ctx_enero = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:enero",
        b.reg(r"enero|ene\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_enero.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Febrero (month 1)
    let ctx_febrero = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:febrero",
        b.reg(r"febrero|feb\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_febrero.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Marzo (month 1)
    let ctx_marzo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:marzo",
        b.reg(r"marzo|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_marzo.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Abril (month 1)
    let ctx_abril = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:abril",
        b.reg(r"abril|abr\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_abril.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Mayo (month 1)
    let ctx_mayo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:mayo",
        b.reg(r"mayo?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_mayo.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Junio (month 1)
    let ctx_junio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:junio",
        b.reg(r"junio|jun\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_junio.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Julio (month 1)
    let ctx_julio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:julio",
        b.reg(r"julio|jul\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_julio.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Agosto (month 1)
    let ctx_agosto = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:agosto",
        b.reg(r"agosto|ago\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_agosto.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Septiembre (month 1)
    let ctx_septiembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:septiembre",
        b.reg(r"septiembre|sept?\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_septiembre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Octubre (month 1)
    let ctx_octubre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:octubre",
        b.reg(r"octubre|oct\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_octubre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Noviembre (month 1)
    let ctx_noviembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:noviembre",
        b.reg(r"noviembre|nov\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_noviembre.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Diciembre (month 1)
    let ctx_diciembre = Arc::clone(&ctx);
    b.rule_1_terminal(
        "es:time:month:diciembre",
        b.reg(r"diciembre|dic\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_diciembre.reference_local();
            let target = ref_time.with_month(1).unwrap();
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

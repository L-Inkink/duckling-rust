// Portuguese Time rules
// Auto-generated from Duckling
// Generated: 2026-02-21

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Portuguese Time rules
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
    // Instant patterns from Portuguese Duckling
    // ========================================

    // "agora" - now
    let ctx_agora = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:agora",
        b.reg(r"agora|atualmente|neste momento").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_agora.reference_utc(), Grain::Second)))
    );

    // "hoje" - today
    let ctx_hoje = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:hoje",
        b.reg(r"hoje|o\s*dia|de\s*hoje").unwrap(),
        move |_| {
            let ref_time = ctx_hoje.reference_local();
            let today = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let today_utc = Utc.from_utc_datetime(&today);
            Ok(Value::Time(TimeValue::instant(today_utc, Grain::Day)))
        }
    );

    // "amanhã" / "amanha" - tomorrow
    let ctx_amanha = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:amanha",
        b.reg(r"amanh(ã|a)|o\s*dia\s*seguinte").unwrap(),
        move |_| {
            let ref_time = ctx_amanha.reference_local();
            let tomorrow = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(1);
            let tomorrow_utc = Utc.from_utc_datetime(&tomorrow);
            Ok(Value::Time(TimeValue::instant(tomorrow_utc, Grain::Day)))
        }
    );

    // "ontem" - yesterday
    let ctx_ontem = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:ontem",
        b.reg(r"ontem|o\s*dia\s*anterior").unwrap(),
        move |_| {
            let ref_time = ctx_ontem.reference_local();
            let yesterday = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(1);
            let yesterday_utc = Utc.from_utc_datetime(&yesterday);
            Ok(Value::Time(TimeValue::instant(yesterday_utc, Grain::Day)))
        }
    );

    // "depois de amanhã" / "amanhã depois" - day after tomorrow
    let ctx_depois_amanha = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:depois_amanha",
        b.reg(r"depois\s*de\s*amanh(ã|a)|amanh(ã|a)\s*depois").unwrap(),
        move |_| {
            let ref_time = ctx_depois_amanha.reference_local();
            let day_after = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() + chrono::Duration::days(2);
            let day_after_utc = Utc.from_utc_datetime(&day_after);
            Ok(Value::Time(TimeValue::instant(day_after_utc, Grain::Day)))
        }
    );

    // "anteontem" / "antes de ontem" - day before yesterday
    let ctx_anteontem = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:anteontem",
        b.reg(r"anteontem|antes\s*de\s*ontem").unwrap(),
        move |_| {
            let ref_time = ctx_anteontem.reference_local();
            let day_before = ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap() - chrono::Duration::days(2);
            let day_before_utc = Utc.from_utc_datetime(&day_before);
            Ok(Value::Time(TimeValue::instant(day_before_utc, Grain::Day)))
        }
    );
}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // Day of week patterns from Portuguese Duckling:
    // - Segunda-feira / Segunda: segunda.?|segunda\\.?...
    // - Terça-feira / Terça: terç[ae]\\.?|terça\\.?...
    // - Quarta-feira / Quarta: quarta\\.?|quarta\\.?...
    // - Quinta-feira / Quinta: quinta\\.?|quinta\\.?...
    // - Sexta-feira / Sexta: sexta\\.?|sexta\\.?...
    // - Sábado / Sabado: s[áa]bado|s[áa]b\\.?...
    // - Domingo: domingo|dom\\.?...

    // Segunda-feira (Monday)
    let ctx_segunda = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:segunda",
        b.reg(r"segunda.?|segunda\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_segunda.reference_local();
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

    // Terça-feira (Tuesday)
    let ctx_terca = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:terca",
        b.reg(r"terç[ae]\\.?|terça\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_terca.reference_local();
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

    // Quarta-feira (Wednesday)
    let ctx_quarta = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:quarta",
        b.reg(r"quarta\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_quarta.reference_local();
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

    // Quinta-feira (Thursday)
    let ctx_quinta = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:quinta",
        b.reg(r"quinta\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_quinta.reference_local();
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

    // Sexta-feira (Friday)
    let ctx_sexta = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:sexta",
        b.reg(r"sexta\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sexta.reference_local();
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

    // Sábado (Saturday)
    let ctx_sabado = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:sabado",
        b.reg(r"s[áa]bado|s[áa]b\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_sabado.reference_local();
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

    // Domingo (Sunday)
    let ctx_domingo = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:dow:domingo",
        b.reg(r"domingo|dom\\.?").unwrap(),
        move |_| {
            let local_ref = ctx_domingo.reference_local();
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
    // Month patterns from Portuguese Duckling:
    // - Janeiro: janeiro|jan\\.?...
    // - Fevereiro: fevereiro|fev\\.?...
    // - Março: março|março|mar\\.?...
    // - Abril: abril|abr\\.?...
    // - Maio: maio|maio|maio\\.?...
    // - Junho: junho|jun\\.?...
    // - Julho: julho|jul\\.?...
    // - Agosto: agosto|ago\\.?...
    // - Setembro: setembro|set\\.?...
    // - Outubro: outubro|out\\.?...
    // - Novembro: novembro|nov\\.?...
    // - Dezembro: dezembro|dez\\.?...

    // Janeiro (month 1)
    let ctx_janeiro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:janeiro",
        b.reg(r"janeiro|jan\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_janeiro.reference_local();
            let target = ref_time.with_month(1).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Fevereiro (month 2)
    let ctx_fevereiro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:fevereiro",
        b.reg(r"fevereiro|fev\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_fevereiro.reference_local();
            let target = ref_time.with_month(2).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Março (month 3)
    let ctx_marco = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:marco",
        b.reg(r"março|marco|mar\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_marco.reference_local();
            let target = ref_time.with_month(3).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Abril (month 4)
    let ctx_abril = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:abril",
        b.reg(r"abril|abr\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_abril.reference_local();
            let target = ref_time.with_month(4).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Maio (month 5)
    let ctx_maio = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:maio",
        b.reg(r"maio|maio\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_maio.reference_local();
            let target = ref_time.with_month(5).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Junho (month 6)
    let ctx_junho = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:junho",
        b.reg(r"junho|jun\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_junho.reference_local();
            let target = ref_time.with_month(6).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Julho (month 7)
    let ctx_julho = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:julho",
        b.reg(r"julho|jul\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_julho.reference_local();
            let target = ref_time.with_month(7).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Agosto (month 8)
    let ctx_agosto = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:agosto",
        b.reg(r"agosto|ago\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_agosto.reference_local();
            let target = ref_time.with_month(8).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Setembro (month 9)
    let ctx_setembro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:setembro",
        b.reg(r"setembro|set\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_setembro.reference_local();
            let target = ref_time.with_month(9).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Outubro (month 10)
    let ctx_outubro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:outubro",
        b.reg(r"outubro|out\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_outubro.reference_local();
            let target = ref_time.with_month(10).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Novembro (month 11)
    let ctx_novembro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:novembro",
        b.reg(r"novembro|nov\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_novembro.reference_local();
            let target = ref_time.with_month(11).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );

    // Dezembro (month 12)
    let ctx_dezembro = Arc::clone(&ctx);
    b.rule_1_terminal(
        "pt:time:month:dezembro",
        b.reg(r"dezembro|dez\\.?").unwrap(),
        move |_| {
            let ref_time = ctx_dezembro.reference_local();
            let target = ref_time.with_month(12).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        },
    );
}

fn _generate_simple_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // TODO: Add simple time reference patterns
    // Examples: "manhã", "tarde", "noite", etc.
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

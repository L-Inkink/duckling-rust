use crate::values::{Value, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use rustling_core::time::Grain;
use chrono::Utc;

pub fn rules(b: &RuleSetBuilder<Value>) {
    // Rule: in <integer> minutes
    b.rule_1_terminal(
        "time: in <integer> minutes",
        b.reg(r"in\s+(\d+)\s+minutes?").unwrap(),
        |text_match| {
            let amount: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse duration: {}", e))?;
            let future = Utc::now() + chrono::Duration::minutes(amount);

            Ok(Value::Time(TimeValue::instant(future, Grain::Minute)))
        }
    );

    // Rule: in <integer> hours
    b.rule_1_terminal(
        "time: in <integer> hours",
        b.reg(r"in\s+(\d+)\s+hours?").unwrap(),
        |text_match| {
            let amount: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse duration: {}", e))?;
            let future = Utc::now() + chrono::Duration::hours(amount);

            Ok(Value::Time(TimeValue::instant(future, Grain::Minute)))
        }
    );
}

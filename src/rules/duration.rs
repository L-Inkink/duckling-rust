use crate::values::{Value, DurationValue, TimeUnit};
use rustling_core::{RuleSetBuilder, rustling_error};

pub fn rules(b: &RuleSetBuilder<Value>) {
    // Rule: <integer> minutes (terminal rule)
    b.rule_1_terminal(
        "duration: <integer> minutes",
        b.reg(r"(\d+)\s+minutes?").unwrap(),
        |text_match| {
            let amount: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse duration: {}", e))?;
            Ok(Value::Duration(DurationValue {
                amount,
                unit: TimeUnit::Minute,
            }))
        }
    );

    // Rule: <integer> hours (terminal rule)
    b.rule_1_terminal(
        "duration: <integer> hours",
        b.reg(r"(\d+)\s+hours?").unwrap(),
        |text_match| {
            let amount: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse duration: {}", e))?;
            Ok(Value::Duration(DurationValue {
                amount,
                unit: TimeUnit::Hour,
            }))
        }
    );

    // Rule: <integer> days (terminal rule)
    b.rule_1_terminal(
        "duration: <integer> days",
        b.reg(r"(\d+)\s+days?").unwrap(),
        |text_match| {
            let amount: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse duration: {}", e))?;
            Ok(Value::Duration(DurationValue {
                amount,
                unit: TimeUnit::Day,
            }))
        }
    );
}

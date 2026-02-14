use crate::values::Value;
use rustling_core::{RuleSetBuilder, rustling_error};

pub fn rules(b: &RuleSetBuilder<Value>) {
    // Rule: Match 1-18 digit integers
    b.rule_1_terminal(
        "integer (numeric)",
        b.reg(r"(\d{1,18})").unwrap(),
        |text_match| {
            let num: i64 = text_match.group(0).parse()
                .map_err(|e| rustling_error!("Failed to parse integer: {}", e))?;
            Ok(Value::Integer(num))
        }
    );
}

pub mod integer;
pub mod duration;
pub mod time;

use crate::values::Value;
use rustling_core::RuleSetBuilder;

pub fn register_all_rules(b: &RuleSetBuilder<Value>) {
    integer::rules(b);
    duration::rules(b);
    time::rules(b);
}

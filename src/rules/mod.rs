pub mod integer;

use crate::values::Value;
use rustling_core::RuleSetBuilder;

pub fn register_all_rules(b: &RuleSetBuilder<Value>) {
    integer::rules(b);
}

// Debug test for intersect rules
use rustling::values::Value;
use rustling::languages::en::time as en_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::{TimeValue, Grain, Form};
use chrono::{Datelike, Timelike};

#[test]
fn debug_monday_morning() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("Monday morning").unwrap();

    println!("Found {} results for 'Monday morning':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        println!("  root_node: {:?}", r.root_node);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  TimeValue::Instant:");
                println!("    datetime: {}", td.datetime);
                println!("    grain: {:?}", td.grain);
                println!("    form: {:?}", td.form);
                println!("    latent: {}", td.latent);
                println!("    hour: {}", td.datetime.hour());
                println!("    weekday: {:?}", td.datetime.weekday());
            }
            _ => println!("  Other value type: {:?}", r.value),
        }
    }
}

#[test]
fn debug_february_15th() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("February 15th").unwrap();

    println!("Found {} results for 'February 15th':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        println!("  root_node: {:?}", r.root_node);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  TimeValue::Instant:");
                println!("    datetime: {}", td.datetime);
                println!("    grain: {:?}", td.grain);
                println!("    form: {:?}", td.form);
                println!("    month: {}", td.datetime.month());
                println!("    day: {}", td.datetime.day());
            }
            _ => println!("  Other value type: {:?}", r.value),
        }
    }
}

// Debug multi-token composite rules
use rustling::values::Value;
use rustling::languages::en::time as en_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::TimeValue;
use chrono::Datelike;

#[test]
fn debug_15th_of_march() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("15th of March").unwrap();

    println!("Found {} results for '15th of March':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {}", td.datetime);
                println!("  month: {}", td.datetime.month());
                println!("  day: {}", td.datetime.day());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);
            }
            _ => println!("  Other value type"),
        }
    }
}

#[test]
fn debug_february_2024() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("February 2024").unwrap();

    println!("Found {} results for 'February 2024':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {}", td.datetime);
                println!("  year: {}", td.datetime.year());
                println!("  month: {}", td.datetime.month());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);
                println!("  latent: {}", td.latent);
            }
            _ => println!("  Other value type"),
        }
    }
}

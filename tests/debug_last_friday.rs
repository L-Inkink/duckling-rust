// Debug test for last Friday
use rustling::values::Value;
use rustling::languages::en::time as en_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::TimeValue;
use chrono::{Utc, Datelike};

#[test]
fn debug_last_friday() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let now = Utc::now();
    println!("Current time: {} ({})", now, now.weekday());
    println!("Current date: {}", now.date_naive());

    let results = ruleset.apply_all("last Friday").unwrap();

    println!("\nFound {} results for 'last Friday':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {} ({})", td.datetime, td.datetime.weekday());
                println!("  date: {}", td.datetime.date_naive());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);

                let days_diff = (now.date_naive() - td.datetime.date_naive()).num_days();
                println!("  days_diff from now: {}", days_diff);
            }
            _ => println!("  Other value type"),
        }
    }
}

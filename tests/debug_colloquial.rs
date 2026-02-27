// Debug colloquial test failures
use rustling::values::Value;
use rustling::languages::en::time as en_time;
use rustling_core::{RuleSetBuilder, BoundariesChecker};
use rustling_core::time::TimeValue;
use chrono::{Datelike, Timelike};

#[test]
fn debug_31st_of_december() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("31st of December");

    match &results {
        Ok(results) => {
            println!("Found {} results for '31st of December':", results.len());
            for (i, r) in results.iter().enumerate() {
                println!("\nResult {}:", i);
                match &r.value {
                    Value::Time(TimeValue::Instant(td)) => {
                        println!("  datetime: {}", td.datetime);
                        println!("  year: {}", td.datetime.year());
                        println!("  month: {}", td.datetime.month());
                        println!("  day: {}", td.datetime.day());
                        println!("  grain: {:?}", td.grain);
                        println!("  form: {:?}", td.form);
                    }
                    _ => println!("  Other value type"),
                }
            }
        }
        Err(e) => {
            println!("Error parsing '31st of December': {:?}", e);
        }
    }
}

#[test]
fn debug_morning_on_friday() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("morning on Friday").unwrap();

    println!("Found {} results for 'morning on Friday':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {}", td.datetime);
                println!("  hour: {}", td.datetime.hour());
                println!("  weekday: {:?}", td.datetime.weekday());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);
                println!("  latent: {}", td.latent);
            }
            _ => println!("  Other value type"),
        }
    }
}

#[test]
fn debug_friday_at_noon() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("Friday at noon").unwrap();

    println!("Found {} results for 'Friday at noon':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {}", td.datetime);
                println!("  hour: {}", td.datetime.hour());
                println!("  weekday: {:?}", td.datetime.weekday());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);
                println!("  latent: {}", td.latent);
            }
            _ => println!("  Other value type"),
        }
    }
}

#[test]
fn debug_3_p_m() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("3 p m").unwrap();

    println!("Found {} results for '3 p m':", results.len());
    for (i, r) in results.iter().enumerate() {
        println!("\nResult {}:", i);
        match &r.value {
            Value::Time(TimeValue::Instant(td)) => {
                println!("  datetime: {}", td.datetime);
                println!("  hour: {}", td.datetime.hour());
                println!("  grain: {:?}", td.grain);
                println!("  form: {:?}", td.form);
            }
            _ => println!("  Other value type"),
        }
    }
}

#[test]
fn debug_3_30_pm() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    en_time::rules(&b, None);
    let ruleset = b.build();

    let results = ruleset.apply_all("3:30pm");

    match &results {
        Ok(results) => {
            println!("Found {} results for '3:30pm':", results.len());
            for (i, r) in results.iter().enumerate() {
                println!("\nResult {}:", i);
                match &r.value {
                    Value::Time(TimeValue::Instant(td)) => {
                        println!("  datetime: {}", td.datetime);
                        println!("  hour: {}", td.datetime.hour());
                        println!("  minute: {}", td.datetime.minute());
                        println!("  grain: {:?}", td.grain);
                        println!("  form: {:?}", td.form);
                    }
                    _ => println!("  Other value type"),
                }
            }
        }
        Err(e) => {
            println!("Error parsing '3:30pm': {:?}", e);
        }
    }
}

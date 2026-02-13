use rustling::values::{DurationValue, TimeUnit};
use chrono::Duration;

#[test]
fn test_duration_to_chrono_minutes() {
    let dur = DurationValue {
        amount: 5,
        unit: TimeUnit::Minute,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::minutes(5));
}

#[test]
fn test_duration_to_chrono_hours() {
    let dur = DurationValue {
        amount: 2,
        unit: TimeUnit::Hour,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::hours(2));
}

#[test]
fn test_duration_to_chrono_days() {
    let dur = DurationValue {
        amount: 3,
        unit: TimeUnit::Day,
    };
    assert_eq!(dur.to_chrono_duration(), Duration::days(3));
}

use rustling::values::{DurationValue, TimeUnit, TimeValue, Value};
use chrono::{Duration, Utc, TimeZone, Timelike};

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

#[test]
fn test_time_value_creation() {
    let timestamp = Utc.with_ymd_and_hms(2026, 2, 14, 8, 0, 0).unwrap();
    let time = TimeValue { timestamp };

    assert_eq!(time.timestamp.hour(), 8);
    assert_eq!(time.timestamp.minute(), 0);
}

#[test]
fn test_value_enum_duration() {
    let dur = DurationValue {
        amount: 5,
        unit: TimeUnit::Minute,
    };
    let value = Value::Duration(dur.clone());

    if let Value::Duration(d) = value {
        assert_eq!(d.amount, 5);
    } else {
        panic!("Expected Duration variant");
    }
}

#[test]
fn test_value_enum_time() {
    let timestamp = Utc::now();
    let time = TimeValue { timestamp: timestamp };
    let value = Value::Time(time.clone());

    if let Value::Time(t) = value {
        assert_eq!(t.timestamp, timestamp);
    } else {
        panic!("Expected Time variant");
    }
}

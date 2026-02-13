use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurationValue {
    pub amount: i64,
    pub unit: TimeUnit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl DurationValue {
    pub fn to_chrono_duration(&self) -> Duration {
        match self.unit {
            TimeUnit::Second => Duration::seconds(self.amount),
            TimeUnit::Minute => Duration::minutes(self.amount),
            TimeUnit::Hour => Duration::hours(self.amount),
            TimeUnit::Day => Duration::days(self.amount),
            TimeUnit::Week => Duration::weeks(self.amount),
        }
    }
}

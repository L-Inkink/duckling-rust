use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::grain::Grain;

/// Direction for time shifts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Before/past direction
    Before,
    /// After/future direction
    After,
}

/// Form of time expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Form {
    /// Unspecified form
    #[default]
    Unspecified,
    /// Day of week (Monday, Tuesday, etc.)
    DayOfWeek,
    /// Day of month (1st, 2nd, etc.)
    DayOfMonth,
    /// Month (January, February, etc.)
    Month,
    /// Year (2024, etc.)
    Year,
    /// Time of day (3 PM, 15:00, etc.)
    TimeOfDay,
    /// Part of day (morning, afternoon, etc.)
    PartOfDay,
}

use std::hash::{Hash, Hasher};

/// Core time data structure
///
/// Represents a time instant or period with associated metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeData {
    /// The datetime value (in UTC)
    pub datetime: DateTime<Utc>,

    /// Granularity of the time value
    pub grain: Grain,

    /// Whether this time is latent (requires context to be meaningful)
    /// Example: "Monday" is latent until you know which Monday
    #[serde(default)]
    pub latent: bool,

    /// Form of the time expression
    #[serde(default)]
    pub form: Form,

    /// Optional holiday name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday: Option<String>,
}

impl TimeData {
    /// Creates a new TimeData instance
    pub fn new(datetime: DateTime<Utc>, grain: Grain) -> Self {
        Self {
            datetime,
            grain,
            latent: false,
            form: Form::Unspecified,
            holiday: None,
        }
    }

    /// Creates a latent time data
    pub fn latent(datetime: DateTime<Utc>, grain: Grain) -> Self {
        Self {
            datetime,
            grain,
            latent: true,
            form: Form::Unspecified,
            holiday: None,
        }
    }

    /// Sets the form
    pub fn with_form(mut self, form: Form) -> Self {
        self.form = form;
        self
    }

    /// Sets the holiday
    pub fn with_holiday(mut self, holiday: impl Into<String>) -> Self {
        self.holiday = Some(holiday.into());
        self
    }

    /// Makes this time data non-latent
    pub fn make_explicit(mut self) -> Self {
        self.latent = false;
        self
    }

    /// Checks if this time data is explicit (not latent)
    pub fn is_explicit(&self) -> bool {
        !self.latent
    }
}

/// Time value - either an instant or an interval
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeValue {
    /// A single time instant or period
    Instant(TimeData),

    /// An interval between two times
    Interval {
        /// Start of the interval (inclusive)
        from: TimeData,
        /// End of the interval (exclusive)
        to: TimeData,
    },
}

impl TimeValue {
    /// Creates an instant time value
    pub fn instant(datetime: DateTime<Utc>, grain: Grain) -> Self {
        Self::Instant(TimeData::new(datetime, grain))
    }

    /// Creates an interval time value
    pub fn interval(from: TimeData, to: TimeData) -> Self {
        Self::Interval { from, to }
    }

    /// Returns the grain of this time value
    pub fn grain(&self) -> Grain {
        match self {
            Self::Instant(td) => td.grain,
            Self::Interval { from, .. } => from.grain,
        }
    }

    /// Returns true if this is an instant value
    pub fn is_instant(&self) -> bool {
        matches!(self, Self::Instant(_))
    }

    /// Returns true if this is an interval value
    pub fn is_interval(&self) -> bool {
        matches!(self, Self::Interval { .. })
    }
}


// Eq and Hash implementations for TimeData and TimeValue
// These are needed for using Time values in HashMaps and as part of Value enum

impl Eq for TimeData {}

impl Hash for TimeData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash datetime as timestamp (i64)
        self.datetime.timestamp().hash(state);
        self.datetime.timestamp_subsec_nanos().hash(state);
        self.grain.hash(state);
        self.latent.hash(state);
        // Form is Copy + Hash, so we can hash it directly
        self.form.hash(state);
        self.holiday.hash(state);
    }
}

impl Eq for TimeValue {}

impl Hash for TimeValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TimeValue::Instant(td) => {
                0u8.hash(state);
                td.hash(state);
            }
            TimeValue::Interval { from, to } => {
                1u8.hash(state);
                from.hash(state);
                to.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_data_creation() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 10, 30, 0).unwrap();
        let td = TimeData::new(dt, Grain::Minute);

        assert_eq!(td.grain, Grain::Minute);
        assert!(!td.latent);
        assert_eq!(td.form, Form::Unspecified);
    }

    #[test]
    fn test_latent_time() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 0, 0, 0).unwrap();
        let td = TimeData::latent(dt, Grain::Day);

        assert!(td.latent);
        assert!(!td.is_explicit());
    }

    #[test]
    fn test_time_value_types() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 10, 0, 0).unwrap();
        let instant = TimeValue::instant(dt, Grain::Hour);

        assert!(instant.is_instant());
        assert!(!instant.is_interval());
        assert_eq!(instant.grain(), Grain::Hour);
    }

    #[test]
    fn test_interval() {
        let dt1 = Utc.with_ymd_and_hms(2024, 2, 15, 9, 0, 0).unwrap();
        let dt2 = Utc.with_ymd_and_hms(2024, 2, 15, 17, 0, 0).unwrap();

        let interval = TimeValue::interval(
            TimeData::new(dt1, Grain::Hour),
            TimeData::new(dt2, Grain::Hour),
        );

        assert!(interval.is_interval());
        assert_eq!(interval.grain(), Grain::Hour);
    }
}

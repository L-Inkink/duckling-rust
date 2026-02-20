// Time Context - Reference time and timezone for time parsing
//
// Following Duckling's design:
// - User provides a reference time (their current local time)
// - Internal computation uses this reference
// - Results are standardized to UTC

use chrono::{DateTime, Local, TimeZone, Utc};

/// Context for time parsing operations
///
/// Contains the reference time point and timezone information used
/// for resolving relative time expressions like "tomorrow", "3 days ago", etc.
#[derive(Debug, Clone)]
pub struct TimeContext {
    /// Reference time point (typically user's current local time)
    /// This is the "now" from which all relative times are calculated
    reference_utc: DateTime<Utc>,

    /// Optional timezone offset in seconds from UTC
    /// Used for local time calculations
    timezone_offset: Option<i32>,
}

impl TimeContext {
    /// Creates a new TimeContext with the given reference time in UTC
    ///
    /// # Example
    /// ```
    /// use chrono::Utc;
    /// use rustling_core::time::TimeContext;
    ///
    /// let now = Utc::now();
    /// let context = TimeContext::new(now);
    /// ```
    pub fn new(reference_utc: DateTime<Utc>) -> Self {
        TimeContext {
            reference_utc,
            timezone_offset: None,
        }
    }

    /// Creates a TimeContext from a DateTime in any timezone
    ///
    /// The datetime is converted to UTC internally for standardization
    ///
    /// # Example
    /// ```
    /// use chrono::Local;
    /// use rustling_core::time::TimeContext;
    ///
    /// let now_local = Local::now();
    /// let context = TimeContext::from_datetime(now_local);
    /// ```
    pub fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Self
    where
        Tz::Offset: Copy,
    {
        let utc = datetime.with_timezone(&Utc);

        // Try to extract timezone offset
        let timezone_offset = {
            use chrono::Offset;
            let offset = datetime.offset().fix();
            Some(offset.local_minus_utc())
        };

        TimeContext {
            reference_utc: utc,
            timezone_offset,
        }
    }

    /// Creates a default TimeContext using the current system local time
    ///
    /// This is the most common use case - using "now" as the reference point
    ///
    /// # Example
    /// ```
    /// use rustling_core::time::TimeContext;
    ///
    /// let context = TimeContext::default();
    /// ```
    pub fn default_now() -> Self {
        Self::from_datetime(Local::now())
    }

    /// Returns the reference time in UTC
    ///
    /// This is the standardized time point used for all calculations
    pub fn reference_utc(&self) -> DateTime<Utc> {
        self.reference_utc
    }

    /// Returns the reference time in the original timezone if available,
    /// otherwise returns UTC
    ///
    /// This is useful when you need to work with the user's local time
    /// for operations like "what day is it" or "what hour is it"
    pub fn reference_local(&self) -> DateTime<chrono::FixedOffset> {
        match self.timezone_offset {
            Some(offset) => {
                let tz = chrono::FixedOffset::east_opt(offset)
                    .unwrap_or_else(|| chrono::FixedOffset::east_opt(0).unwrap());
                self.reference_utc.with_timezone(&tz)
            }
            None => {
                // No timezone info, return as UTC with zero offset
                self.reference_utc.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
            }
        }
    }

    /// Returns the timezone offset in seconds from UTC
    pub fn timezone_offset(&self) -> Option<i32> {
        self.timezone_offset
    }

    /// Creates a TimeContext with a specific UTC time (mainly for testing)
    ///
    /// # Example
    /// ```
    /// use chrono::{Utc, TimeZone};
    /// use rustling_core::time::TimeContext;
    ///
    /// let context = TimeContext::for_test(
    ///     Utc.with_ymd_and_hms(2026, 2, 20, 12, 0, 0).unwrap()
    /// );
    /// ```
    pub fn for_test(reference_utc: DateTime<Utc>) -> Self {
        TimeContext {
            reference_utc,
            timezone_offset: None,
        }
    }
}

impl Default for TimeContext {
    /// Default implementation uses current system local time
    fn default() -> Self {
        Self::default_now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_new() {
        let utc_time = Utc.with_ymd_and_hms(2026, 2, 20, 12, 0, 0).unwrap();
        let context = TimeContext::new(utc_time);

        assert_eq!(context.reference_utc(), utc_time);
        assert_eq!(context.timezone_offset(), None);
    }

    #[test]
    fn test_from_datetime_utc() {
        let utc_time = Utc.with_ymd_and_hms(2026, 2, 20, 12, 0, 0).unwrap();
        let context = TimeContext::from_datetime(utc_time);

        assert_eq!(context.reference_utc(), utc_time);
    }

    #[test]
    fn test_from_datetime_with_offset() {
        // Beijing time: UTC+8
        let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        let beijing_time = offset.with_ymd_and_hms(2026, 2, 20, 20, 0, 0).unwrap();

        let context = TimeContext::from_datetime(beijing_time);

        // Should be converted to UTC
        let expected_utc = Utc.with_ymd_and_hms(2026, 2, 20, 12, 0, 0).unwrap();
        assert_eq!(context.reference_utc(), expected_utc);
        assert_eq!(context.timezone_offset(), Some(8 * 3600));
    }

    #[test]
    fn test_reference_local() {
        let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        let beijing_time = offset.with_ymd_and_hms(2026, 2, 20, 20, 0, 0).unwrap();

        let context = TimeContext::from_datetime(beijing_time);
        let local = context.reference_local();

        // Should get back Beijing time
        assert_eq!(local.hour(), 20);
        assert_eq!(local.offset().local_minus_utc(), 8 * 3600);
    }

    #[test]
    fn test_for_test() {
        let utc_time = Utc.with_ymd_and_hms(2026, 2, 20, 12, 0, 0).unwrap();
        let context = TimeContext::for_test(utc_time);

        assert_eq!(context.reference_utc(), utc_time);
    }
}

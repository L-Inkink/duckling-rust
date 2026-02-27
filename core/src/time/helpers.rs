//! Time manipulation helpers
//!
//! Core functions for time calculations, borrowed from Duckling/Time/Helpers.hs

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use super::{Direction, Grain, TimeData, TimeValue};

/// Intersect two time values to produce a more specific time
///
/// Example: "February" ∩ "2024" → "February 2024"
///
/// # Arguments
///
/// * `td1` - First time data
/// * `td2` - Second time data
///
/// # Returns
///
/// `Some(TimeData)` if the intersection is valid, `None` otherwise
pub fn intersect(td1: &TimeData, td2: &TimeData) -> Option<TimeData> {
    // Basic implementation: take the finer grain and intersect datetime values
    let grain = Grain::finer(td1.grain, td2.grain);

    // For a proper intersection, we need to ensure both times are compatible
    // This is a simplified version - Duckling has much more complex logic

    // Start with the coarser time's datetime
    let (coarse, fine) = if td1.grain.coarser_than(&td2.grain) {
        (td1, td2)
    } else {
        (td2, td1)
    };

    // Check if the fine-grained time falls within the coarse-grained time
    // For now, we'll use a simple heuristic: combine their datetime components
    let mut result = fine.datetime;

    // Copy year/month/day from coarser grain if it's coarser
    match coarse.grain {
        Grain::Year => {
            result = result.with_year(coarse.datetime.year())?;
        }
        Grain::Month => {
            result = result.with_year(coarse.datetime.year())?;
            result = result.with_month(coarse.datetime.month())?;
        }
        Grain::Day => {
            result = result.with_year(coarse.datetime.year())?;
            result = result.with_month(coarse.datetime.month())?;
            result = result.with_day(coarse.datetime.day())?;
        }
        _ => {}
    }

    Some(TimeData {
        datetime: result,
        grain,
        latent: td1.latent && td2.latent,
        form: fine.form,
        holiday: td1.holiday.clone().or_else(|| td2.holiday.clone()),
    })
}

/// Shift a time by a given offset in a specific direction
///
/// Example: "today" + 1 day → "tomorrow"
///
/// # Arguments
///
/// * `td` - Base time data
/// * `direction` - Direction to shift (Before/After)
/// * `grain` - Grain of the shift
/// * `n` - Number of units to shift
///
/// # Returns
///
/// Shifted time data
pub fn shift(td: &TimeData, direction: Direction, grain: Grain, n: i32) -> TimeData {
    let duration = match grain {
        Grain::Second => Duration::seconds(n as i64),
        Grain::Minute => Duration::minutes(n as i64),
        Grain::Hour => Duration::hours(n as i64),
        Grain::Day => Duration::days(n as i64),
        Grain::Week => Duration::weeks(n as i64),
        Grain::Month => {
            // Month arithmetic is tricky - use chrono's built-in support
            let months = n;
            let new_date = if months >= 0 {
                td.datetime.checked_add_months(chrono::Months::new(months as u32))
            } else {
                td.datetime.checked_sub_months(chrono::Months::new((-months) as u32))
            };

            let datetime = match direction {
                Direction::After => new_date.unwrap_or(td.datetime),
                Direction::Before => new_date.unwrap_or(td.datetime),
            };

            return TimeData {
                datetime,
                grain: td.grain,
                latent: false, // Shifted times are usually explicit
                form: td.form,
                holiday: td.holiday.clone(),
            };
        }
        Grain::Quarter => {
            // Quarter = 3 months
            let months = n * 3;
            let new_date = if months >= 0 {
                td.datetime.checked_add_months(chrono::Months::new(months as u32))
            } else {
                td.datetime.checked_sub_months(chrono::Months::new((-months) as u32))
            };

            let datetime = match direction {
                Direction::After => new_date.unwrap_or(td.datetime),
                Direction::Before => new_date.unwrap_or(td.datetime),
            };

            return TimeData {
                datetime,
                grain: td.grain,
                latent: false,
                form: td.form,
                holiday: td.holiday.clone(),
            };
        }
        Grain::Year => {
            let years = n;
            let new_date = td.datetime.with_year(td.datetime.year() + years);

            let datetime = new_date.unwrap_or(td.datetime);

            return TimeData {
                datetime,
                grain: td.grain,
                latent: false,
                form: td.form,
                holiday: td.holiday.clone(),
            };
        }
    };

    let datetime = match direction {
        Direction::After => td.datetime + duration,
        Direction::Before => td.datetime - duration,
    };

    TimeData {
        datetime,
        grain: td.grain,
        latent: false,
        form: td.form,
        holiday: td.holiday.clone(),
    }
}

/// Create a sequence of time intervals
///
/// Example: ["Monday", "Tuesday", "Wednesday"] → 3-day interval
///
/// # Arguments
///
/// * `tds` - Vector of time data in chronological order
///
/// # Returns
///
/// `Some(TimeValue)` representing the interval, or `None` if invalid
pub fn sequence(tds: Vec<TimeData>) -> Option<TimeValue> {
    if tds.len() < 2 {
        return None;
    }

    // Check if the sequence is evenly spaced
    let _grain = tds[0].grain;

    // For now, simple implementation: return interval from first to last
    let from = tds.first()?.clone();
    let to = tds.last()?.clone();

    Some(TimeValue::Interval { from, to })
}

/// Round a datetime to the start of its grain
///
/// Example: "2024-02-15 14:30:00" rounded to Day → "2024-02-15 00:00:00"
pub fn round_to_grain(dt: DateTime<Utc>, grain: Grain) -> DateTime<Utc> {
    match grain {
        Grain::Second => dt,
        Grain::Minute => dt
            .with_second(0)
            .unwrap_or(dt)
            .with_nanosecond(0)
            .unwrap_or(dt),
        Grain::Hour => dt
            .with_minute(0)
            .unwrap_or(dt)
            .with_second(0)
            .unwrap_or(dt)
            .with_nanosecond(0)
            .unwrap_or(dt),
        Grain::Day => dt
            .with_hour(0)
            .unwrap_or(dt)
            .with_minute(0)
            .unwrap_or(dt)
            .with_second(0)
            .unwrap_or(dt)
            .with_nanosecond(0)
            .unwrap_or(dt),
        Grain::Week => {
            // Round to Monday (start of week)
            let weekday = dt.weekday().num_days_from_monday();
            let days_back = Duration::days(weekday as i64);
            (dt - days_back)
                .with_hour(0)
                .unwrap_or(dt)
                .with_minute(0)
                .unwrap_or(dt)
                .with_second(0)
                .unwrap_or(dt)
                .with_nanosecond(0)
                .unwrap_or(dt)
        }
        Grain::Month => dt
            .with_day(1)
            .unwrap_or(dt)
            .with_hour(0)
            .unwrap_or(dt)
            .with_minute(0)
            .unwrap_or(dt)
            .with_second(0)
            .unwrap_or(dt)
            .with_nanosecond(0)
            .unwrap_or(dt),
        Grain::Quarter => {
            // Round to start of quarter (Jan 1, Apr 1, Jul 1, Oct 1)
            let month = dt.month();
            let quarter_start_month = ((month - 1) / 3) * 3 + 1;
            dt.with_month(quarter_start_month)
                .unwrap_or(dt)
                .with_day(1)
                .unwrap_or(dt)
                .with_hour(0)
                .unwrap_or(dt)
                .with_minute(0)
                .unwrap_or(dt)
                .with_second(0)
                .unwrap_or(dt)
                .with_nanosecond(0)
                .unwrap_or(dt)
        }
        Grain::Year => dt
            .with_month(1)
            .unwrap_or(dt)
            .with_day(1)
            .unwrap_or(dt)
            .with_hour(0)
            .unwrap_or(dt)
            .with_minute(0)
            .unwrap_or(dt)
            .with_second(0)
            .unwrap_or(dt)
            .with_nanosecond(0)
            .unwrap_or(dt),
    }
}

/// Get the end datetime for a time data based on its grain
///
/// Example: "2024-02-15" with Day grain → "2024-02-16 00:00:00" (exclusive end)
pub fn end_of_grain(td: &TimeData) -> DateTime<Utc> {
    let start = round_to_grain(td.datetime, td.grain);

    match td.grain {
        Grain::Second => start + Duration::seconds(1),
        Grain::Minute => start + Duration::minutes(1),
        Grain::Hour => start + Duration::hours(1),
        Grain::Day => start + Duration::days(1),
        Grain::Week => start + Duration::weeks(1),
        Grain::Month => {
            // Add one month
            start.checked_add_months(chrono::Months::new(1))
                .unwrap_or(start + Duration::days(30))
        }
        Grain::Quarter => {
            // Add 3 months
            start.checked_add_months(chrono::Months::new(3))
                .unwrap_or(start + Duration::days(90))
        }
        Grain::Year => {
            // Add one year
            start.with_year(start.year() + 1)
                .unwrap_or(start + Duration::days(365))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_intersect_year_and_month() {
        let year_2024 = TimeData::new(
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            Grain::Year,
        );

        let february = TimeData::new(
            Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap(),
            Grain::Month,
        );

        let result = intersect(&year_2024, &february).unwrap();

        assert_eq!(result.grain, Grain::Month);
        assert_eq!(result.datetime.year(), 2024);
        assert_eq!(result.datetime.month(), 2);
    }

    #[test]
    fn test_shift_day_forward() {
        let today = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 15, 12, 0, 0).unwrap(),
            Grain::Day,
        );

        let tomorrow = shift(&today, Direction::After, Grain::Day, 1);

        assert_eq!(tomorrow.datetime.year(), 2024);
        assert_eq!(tomorrow.datetime.month(), 2);
        assert_eq!(tomorrow.datetime.day(), 16);
    }

    #[test]
    fn test_shift_day_backward() {
        let today = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 15, 12, 0, 0).unwrap(),
            Grain::Day,
        );

        let yesterday = shift(&today, Direction::Before, Grain::Day, 1);

        assert_eq!(yesterday.datetime.year(), 2024);
        assert_eq!(yesterday.datetime.month(), 2);
        assert_eq!(yesterday.datetime.day(), 14);
    }

    #[test]
    fn test_shift_month() {
        let jan = TimeData::new(
            Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap(),
            Grain::Month,
        );

        let mar = shift(&jan, Direction::After, Grain::Month, 2);

        assert_eq!(mar.datetime.year(), 2024);
        assert_eq!(mar.datetime.month(), 3);
    }

    #[test]
    fn test_sequence() {
        let mon = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 12, 0, 0, 0).unwrap(),
            Grain::Day,
        ).with_form(super::super::Form::DayOfWeek);

        let tue = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 13, 0, 0, 0).unwrap(),
            Grain::Day,
        ).with_form(super::super::Form::DayOfWeek);

        let wed = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 14, 0, 0, 0).unwrap(),
            Grain::Day,
        ).with_form(super::super::Form::DayOfWeek);

        let result = sequence(vec![mon.clone(), tue.clone(), wed.clone()]).unwrap();

        if let TimeValue::Interval { from, to } = result {
            assert_eq!(from.datetime.day(), 12);
            assert_eq!(to.datetime.day(), 14);
        } else {
            panic!("Expected interval");
        }
    }

    #[test]
    fn test_round_to_grain_day() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 14, 30, 45).unwrap();
        let rounded = round_to_grain(dt, Grain::Day);

        assert_eq!(rounded.hour(), 0);
        assert_eq!(rounded.minute(), 0);
        assert_eq!(rounded.second(), 0);
        assert_eq!(rounded.day(), 15);
    }

    #[test]
    fn test_round_to_grain_week() {
        // Feb 15, 2024 is a Thursday
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 14, 30, 0).unwrap();
        let rounded = round_to_grain(dt, Grain::Week);

        // Should round back to Monday, Feb 12
        assert_eq!(rounded.day(), 12);
        assert_eq!(rounded.hour(), 0);
    }

    #[test]
    fn test_round_to_grain_month() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 15, 14, 30, 0).unwrap();
        let rounded = round_to_grain(dt, Grain::Month);

        assert_eq!(rounded.day(), 1);
        assert_eq!(rounded.hour(), 0);
    }

    #[test]
    fn test_end_of_grain_day() {
        let td = TimeData::new(
            Utc.with_ymd_and_hms(2024, 2, 15, 10, 0, 0).unwrap(),
            Grain::Day,
        );

        let end = end_of_grain(&td);

        // End should be start of next day
        assert_eq!(end.year(), 2024);
        assert_eq!(end.month(), 2);
        assert_eq!(end.day(), 16);
        assert_eq!(end.hour(), 0);
    }
}

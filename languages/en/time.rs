// English Time rules
// Manually implemented based on Duckling/Time/EN/Rules.hs
//
// Status: Phase 2.2 - Pilot implementation
// Coverage: Core rules (named-days, named-months, simple patterns)

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};

/// Build English Time rules
///
/// Implements core Time parsing rules for English language:
/// - Named days of week (Monday, Tuesday, etc.)
/// - Named months (January, February, etc.)
/// - Relative time (now, today, tomorrow, yesterday)
/// - Simple date patterns
pub fn rules(b: &RuleSetBuilder<Value>) {
    // ========================================
    // Named Days of Week (7 rules)
    // ========================================

    named_day_of_week(b, "Monday", r"(?i)mondays?|mon\.?", Weekday::Mon);
    named_day_of_week(b, "Tuesday", r"(?i)tuesdays?|tues?\.?", Weekday::Tue);
    named_day_of_week(b, "Wednesday", r"(?i)wed?nesdays?|wed\.?", Weekday::Wed);
    named_day_of_week(b, "Thursday", r"(?i)thursdays?|thu(rs?)?\.?", Weekday::Thu);
    named_day_of_week(b, "Friday", r"(?i)fridays?|fri\.?", Weekday::Fri);
    named_day_of_week(b, "Saturday", r"(?i)saturdays?|sat\.?", Weekday::Sat);
    named_day_of_week(b, "Sunday", r"(?i)sundays?|sun\.?", Weekday::Sun);

    // ========================================
    // Named Months (12 rules)
    // ========================================

    named_month(b, "January", r"(?i)january|jan\.?", 1, false);
    named_month(b, "February", r"(?i)february|feb\.?", 2, false);
    named_month(b, "March", r"(?i)march|mar\.?", 3, false);
    named_month(b, "April", r"(?i)april|apr\.?", 4, false);
    named_month(b, "May", r"(?i)may", 5, true); // "may" is ambiguous (modal verb)
    named_month(b, "June", r"(?i)june|jun\.?", 6, false);
    named_month(b, "July", r"(?i)july|jul\.?", 7, false);
    named_month(b, "August", r"(?i)august|aug\.?", 8, false);
    named_month(b, "September", r"(?i)september|sept?\.?", 9, false);
    named_month(b, "October", r"(?i)october|oct\.?", 10, false);
    named_month(b, "November", r"(?i)november|nov\.?", 11, false);
    named_month(b, "December", r"(?i)december|dec\.?", 12, false);

    // ========================================
    // Simple Time References (4 rules)
    // ========================================

    // "now"
    b.rule_1_terminal(
        "en:time:now",
        b.reg(r"(?i)now|at\s+the\s+moment|atm").unwrap(),
        |_| Ok(Value::Time(TimeValue::instant(Utc::now(), Grain::Second)))
    );

    // "today"
    b.rule_1_terminal(
        "en:time:today",
        b.reg(r"(?i)todays?").unwrap(),
        |_| {
            let now = Utc::now();
            let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "tomorrow"
    b.rule_1_terminal(
        "en:time:tomorrow",
        b.reg(r"(?i)tomorrows?|tmrw?").unwrap(),
        |_| {
            let now = Utc::now();
            let tomorrow = now + Duration::days(1);
            let start_of_day = tomorrow.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "yesterday"
    b.rule_1_terminal(
        "en:time:yesterday",
        b.reg(r"(?i)yesterdays?").unwrap(),
        |_| {
            let now = Utc::now();
            let yesterday = now - Duration::days(1);
            let start_of_day = yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // ========================================
    // Year Patterns (3 rules)
    // ========================================

    // "2024", "1999" - 4-digit year
    b.rule_1_terminal(
        "en:time:year",
        b.reg(r"(?i)(1\d{3}|20\d{2}|2100)").unwrap(),
        |text_match| {
            let year: i32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse year: {}", e))?;

            let dt = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid year: {}", year))?;

            let time_data = TimeData::new(dt, Grain::Year)
                .with_form(Form::Year);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Time of Day (4 rules)
    // ========================================

    // "3pm", "11am", "3 pm"
    b.rule_1_terminal(
        "en:time:hour_am_pm",
        b.reg(r"(?i)(\d{1,2})\s?(am|pm)").unwrap(),
        |text_match| {
            let hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse hour: {}", e))?;
            let am_pm = text_match.group(2).to_lowercase();

            // Convert to 24-hour format
            let hour_24 = if am_pm == "am" {
                if hour == 12 { 0 } else { hour }
            } else {
                if hour == 12 { 12 } else { hour + 12 }
            };

            if hour_24 >= 24 {
                return Err(rustling_error!("Invalid hour: {}", hour));
            }

            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(hour_24, 0, 0)
                .ok_or_else(|| rustling_error!("Invalid time"))?;
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::TimeOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "15:30", "3:45", "23:59"
    b.rule_1_terminal(
        "en:time:hh_mm",
        b.reg(r"(?i)(\d{1,2}):(\d{2})").unwrap(),
        |text_match| {
            let hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse hour: {}", e))?;
            let minute: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse minute: {}", e))?;

            if hour >= 24 || minute >= 60 {
                return Err(rustling_error!("Invalid time: {}:{}", hour, minute));
            }

            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(hour, minute, 0)
                .ok_or_else(|| rustling_error!("Invalid time"))?;
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Minute)
                .with_form(Form::TimeOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "15:30:45", "3:45:12"
    b.rule_1_terminal(
        "en:time:hh_mm_ss",
        b.reg(r"(?i)(\d{1,2}):(\d{2}):(\d{2})").unwrap(),
        |text_match| {
            let hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse hour: {}", e))?;
            let minute: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse minute: {}", e))?;
            let second: u32 = text_match.group(3).parse()
                .map_err(|e| rustling_error!("Failed to parse second: {}", e))?;

            if hour >= 24 || minute >= 60 || second >= 60 {
                return Err(rustling_error!("Invalid time: {}:{}:{}", hour, minute, second));
            }

            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(hour, minute, second)
                .ok_or_else(|| rustling_error!("Invalid time"))?;
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Second)
                .with_form(Form::TimeOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Part of Day (4 rules)
    // ========================================

    // "morning"
    b.rule_1_terminal(
        "en:time:morning",
        b.reg(r"(?i)mornings?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(8, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::latent(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "afternoon"
    b.rule_1_terminal(
        "en:time:afternoon",
        b.reg(r"(?i)afternoons?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(15, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::latent(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "evening"
    b.rule_1_terminal(
        "en:time:evening",
        b.reg(r"(?i)evenings?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(18, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::latent(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "night"
    b.rule_1_terminal(
        "en:time:night",
        b.reg(r"(?i)nights?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(21, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::latent(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Day of Month (2 rules)
    // ========================================

    // "the 15th", "the 3rd", "31st"
    b.rule_1_terminal(
        "en:time:day_of_month",
        b.reg(r"(?i)(the\s+)?(\d{1,2})(st|nd|rd|th)").unwrap(),
        |text_match| {
            let day: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse day: {}", e))?;

            if day < 1 || day > 31 {
                return Err(rustling_error!("Invalid day of month: {}", day));
            }

            let now = Utc::now();
            let year = now.year();
            let month = now.month();

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day)
                .with_form(Form::DayOfMonth);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Date Patterns (5 rules)
    // ========================================

    // "02/15", "2/15" (MM/DD) - without year
    b.rule_1_terminal(
        "en:time:mm_dd",
        b.reg(r"(\d{1,2})/(\d{1,2})(?:[^\d/]|$)").unwrap(),
        |text_match| {
            let month: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse month: {}", e))?;
            let day: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse day: {}", e))?;

            if month < 1 || month > 12 || day < 1 || day > 31 {
                return Err(rustling_error!("Invalid date: {}/{}", month, day));
            }

            let now = Utc::now();
            let year = now.year();

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "02/15/2024", "2/15/24"
    b.rule_1_terminal(
        "en:time:mm_dd_yyyy",
        b.reg(r"(\d{1,2})/(\d{1,2})/(\d{2,4})").unwrap(),
        |text_match| {
            let month: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse month: {}", e))?;
            let day: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse day: {}", e))?;
            let year_str = text_match.group(3);

            // Handle 2-digit or 4-digit year
            let year: i32 = if year_str.len() == 2 {
                let yy: i32 = year_str.parse()
                    .map_err(|e| rustling_error!("Failed to parse year: {}", e))?;
                // Assume 20xx for years 00-49, 19xx for 50-99
                if yy < 50 { 2000 + yy } else { 1900 + yy }
            } else {
                year_str.parse()
                    .map_err(|e| rustling_error!("Failed to parse year: {}", e))?
            };

            if month < 1 || month > 12 || day < 1 || day > 31 {
                return Err(rustling_error!("Invalid date: {}/{}/{}", month, day, year));
            }

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "2024-02-15" (ISO 8601)
    b.rule_1_terminal(
        "en:time:yyyy_mm_dd",
        b.reg(r"(\d{4})-(\d{1,2})-(\d{1,2})").unwrap(),
        |text_match| {
            let year: i32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse year: {}", e))?;
            let month: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Failed to parse month: {}", e))?;
            let day: u32 = text_match.group(3).parse()
                .map_err(|e| rustling_error!("Failed to parse day: {}", e))?;

            if month < 1 || month > 12 || day < 1 || day > 31 {
                return Err(rustling_error!("Invalid date: {}-{}-{}", year, month, day));
            }

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}-{}-{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Relative Time - Next/Last (6 rules)
    // ========================================

    // "next week", "next month", "next year"
    b.rule_1_terminal(
        "en:time:next_grain",
        b.reg(r"(?i)next\s+(week|month|year)").unwrap(),
        |text_match| {
            let grain_str = text_match.group(1).to_lowercase();
            let grain = match grain_str.as_str() {
                "week" => Grain::Week,
                "month" => Grain::Month,
                "year" => Grain::Year,
                _ => return Err(rustling_error!("Invalid grain: {}", grain_str)),
            };

            let now = Utc::now();
            let dt = match grain {
                Grain::Week => now + Duration::weeks(1),
                Grain::Month => {
                    now.checked_add_months(chrono::Months::new(1))
                        .ok_or_else(|| rustling_error!("Invalid month calculation"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(now.year() + 1, now.month(), now.day(), 0, 0, 0)
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid year calculation"))?
                }
                _ => return Err(rustling_error!("Unsupported grain")),
            };

            // Round to start of grain
            let dt_rounded = match grain {
                Grain::Week => {
                    let weekday = dt.weekday().num_days_from_monday();
                    let days_back = Duration::days(weekday as i64);
                    (dt - days_back).date_naive().and_hms_opt(0, 0, 0)
                        .map(|naive| Utc.from_utc_datetime(&naive))
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                Grain::Month => {
                    let d = dt.with_day(1).ok_or_else(|| rustling_error!("Invalid day"))?;
                    let d = d.with_hour(0).ok_or_else(|| rustling_error!("Invalid hour"))?;
                    let d = d.with_minute(0).ok_or_else(|| rustling_error!("Invalid minute"))?;
                    d.with_second(0).ok_or_else(|| rustling_error!("Invalid second"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(dt.year(), 1, 1, 0, 0, 0)
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                _ => dt,
            };

            let time_data = TimeData::new(dt_rounded, grain);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "last week", "last month", "last year"
    b.rule_1_terminal(
        "en:time:last_grain",
        b.reg(r"(?i)last\s+(week|month|year)").unwrap(),
        |text_match| {
            let grain_str = text_match.group(1).to_lowercase();
            let grain = match grain_str.as_str() {
                "week" => Grain::Week,
                "month" => Grain::Month,
                "year" => Grain::Year,
                _ => return Err(rustling_error!("Invalid grain: {}", grain_str)),
            };

            let now = Utc::now();
            let dt = match grain {
                Grain::Week => now - Duration::weeks(1),
                Grain::Month => {
                    now.checked_sub_months(chrono::Months::new(1))
                        .ok_or_else(|| rustling_error!("Invalid month calculation"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(now.year() - 1, now.month(), now.day(), 0, 0, 0)
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid year calculation"))?
                }
                _ => return Err(rustling_error!("Unsupported grain")),
            };

            // Round to start of grain
            let dt_rounded = match grain {
                Grain::Week => {
                    let weekday = dt.weekday().num_days_from_monday();
                    let days_back = Duration::days(weekday as i64);
                    (dt - days_back).date_naive().and_hms_opt(0, 0, 0)
                        .map(|naive| Utc.from_utc_datetime(&naive))
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                Grain::Month => {
                    let d = dt.with_day(1).ok_or_else(|| rustling_error!("Invalid day"))?;
                    let d = d.with_hour(0).ok_or_else(|| rustling_error!("Invalid hour"))?;
                    let d = d.with_minute(0).ok_or_else(|| rustling_error!("Invalid minute"))?;
                    d.with_second(0).ok_or_else(|| rustling_error!("Invalid second"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(dt.year(), 1, 1, 0, 0, 0)
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                _ => dt,
            };

            let time_data = TimeData::new(dt_rounded, grain);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // TODO: More rules to be implemented (Priority 2+)
    // - "this week", "this month"
    // - "2 days ago", "3 weeks ago"
    // - "in 5 minutes", "in 2 hours"
    // - Intervals (from...to...)
    // - Intersect rules (Monday morning, February 2024)
}

/// Helper: Create a named day of week rule
fn named_day_of_week(
    b: &RuleSetBuilder<Value>,
    name: &'static str,
    pattern: &str,
    weekday: Weekday,
) {
    let rule_name = format!("en:time:{}", name.to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            // Get the next occurrence of this weekday
            let now = Utc::now();
            let current_weekday = now.weekday();

            // Calculate days until target weekday
            let days_until = ((weekday.number_from_monday() as i32)
                - (current_weekday.number_from_monday() as i32)
                + 7) % 7;

            let target_date = if days_until == 0 {
                // If it's the same weekday, use today
                now
            } else {
                now + Duration::days(days_until as i64)
            };

            let start_of_day = target_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);

            let time_data = TimeData::new(dt, Grain::Day)
                .with_form(Form::DayOfWeek);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Helper: Create a named month rule
fn named_month(
    b: &RuleSetBuilder<Value>,
    name: &'static str,
    pattern: &str,
    month_num: u32,
    latent: bool,
) {
    let rule_name = format!("en:time:{}", name.to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            // Create a time for this month in the current year
            let now = Utc::now();
            let year = now.year();

            // Start of the month
            let dt = Utc.with_ymd_and_hms(year, month_num, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid month: {}", month_num))?;

            let mut time_data = TimeData::new(dt, Grain::Month)
                .with_form(Form::Month);

            if latent {
                time_data.latent = true;
            }

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustling_core::BoundariesChecker;

    #[test]
    fn test_en_time_rules_compile() {
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);
        // Smoke test - rules should register without panic
    }

    #[test]
    fn test_rule_count() {
        // We should have at least:
        // 7 days of week + 12 months + 4 simple references = 23 rules
        let b = RuleSetBuilder::new(
            BoundariesChecker::detailed(),
            BoundariesChecker::separated_alphanumeric_word(),
        );
        rules(&b);

        // TODO: Add actual rule counting when RuleSetBuilder exposes this
    }
}

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

    // ========================================
    // Relative Time - This (3 rules)
    // ========================================

    // "this week", "this month", "this year"
    b.rule_1_terminal(
        "en:time:this_grain",
        b.reg(r"(?i)this\s+(week|month|year)").unwrap(),
        |text_match| {
            let grain_str = text_match.group(1).to_lowercase();
            let grain = match grain_str.as_str() {
                "week" => Grain::Week,
                "month" => Grain::Month,
                "year" => Grain::Year,
                _ => return Err(rustling_error!("Invalid grain: {}", grain_str)),
            };

            let now = Utc::now();

            // Round to start of grain
            let dt = match grain {
                Grain::Week => {
                    let weekday = now.weekday().num_days_from_monday();
                    let days_back = Duration::days(weekday as i64);
                    (now - days_back).date_naive().and_hms_opt(0, 0, 0)
                        .map(|naive| Utc.from_utc_datetime(&naive))
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                Grain::Month => {
                    let d = now.with_day(1).ok_or_else(|| rustling_error!("Invalid day"))?;
                    let d = d.with_hour(0).ok_or_else(|| rustling_error!("Invalid hour"))?;
                    let d = d.with_minute(0).ok_or_else(|| rustling_error!("Invalid minute"))?;
                    d.with_second(0).ok_or_else(|| rustling_error!("Invalid second"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0)
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid date"))?
                }
                _ => now,
            };

            let time_data = TimeData::new(dt, grain);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Relative Time - N Cycles Ago (6 rules)
    // ========================================

    // "2 days ago", "3 weeks ago", "5 months ago"
    b.rule_1_terminal(
        "en:time:n_cycles_ago",
        b.reg(r"(?i)(\d+)\s+(seconds?|minutes?|hours?|days?|weeks?|months?|years?)\s+ago").unwrap(),
        |text_match| {
            let n: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse number: {}", e))?;
            let grain_str = text_match.group(2).to_lowercase();

            let grain = match grain_str.as_str() {
                "second" | "seconds" => Grain::Second,
                "minute" | "minutes" => Grain::Minute,
                "hour" | "hours" => Grain::Hour,
                "day" | "days" => Grain::Day,
                "week" | "weeks" => Grain::Week,
                "month" | "months" => Grain::Month,
                "year" | "years" => Grain::Year,
                _ => return Err(rustling_error!("Invalid grain: {}", grain_str)),
            };

            let now = Utc::now();
            let dt = match grain {
                Grain::Second => now - Duration::seconds(n),
                Grain::Minute => now - Duration::minutes(n),
                Grain::Hour => now - Duration::hours(n),
                Grain::Day => now - Duration::days(n),
                Grain::Week => now - Duration::weeks(n),
                Grain::Month => {
                    now.checked_sub_months(chrono::Months::new(n as u32))
                        .ok_or_else(|| rustling_error!("Invalid month calculation"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(now.year() - n as i32, now.month(), now.day(), now.hour(), now.minute(), now.second())
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid year calculation"))?
                }
                _ => return Err(rustling_error!("Unsupported grain")),
            };

            let time_data = TimeData::new(dt, grain);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Relative Time - In Duration (6 rules)
    // ========================================

    // "in 5 minutes", "in 2 hours", "in 3 days"
    b.rule_1_terminal(
        "en:time:in_duration",
        b.reg(r"(?i)in\s+(\d+)\s+(seconds?|minutes?|hours?|days?|weeks?|months?|years?)").unwrap(),
        |text_match| {
            let n: i64 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse number: {}", e))?;
            let grain_str = text_match.group(2).to_lowercase();

            let grain = match grain_str.as_str() {
                "second" | "seconds" => Grain::Second,
                "minute" | "minutes" => Grain::Minute,
                "hour" | "hours" => Grain::Hour,
                "day" | "days" => Grain::Day,
                "week" | "weeks" => Grain::Week,
                "month" | "months" => Grain::Month,
                "year" | "years" => Grain::Year,
                _ => return Err(rustling_error!("Invalid grain: {}", grain_str)),
            };

            let now = Utc::now();
            let dt = match grain {
                Grain::Second => now + Duration::seconds(n),
                Grain::Minute => now + Duration::minutes(n),
                Grain::Hour => now + Duration::hours(n),
                Grain::Day => now + Duration::days(n),
                Grain::Week => now + Duration::weeks(n),
                Grain::Month => {
                    now.checked_add_months(chrono::Months::new(n as u32))
                        .ok_or_else(|| rustling_error!("Invalid month calculation"))?
                }
                Grain::Year => {
                    Utc.with_ymd_and_hms(now.year() + n as i32, now.month(), now.day(), now.hour(), now.minute(), now.second())
                        .single()
                        .ok_or_else(|| rustling_error!("Invalid year calculation"))?
                }
                _ => return Err(rustling_error!("Unsupported grain")),
            };

            let time_data = TimeData::new(dt, grain);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Relative Time - Next/Last + Day of Week (14 rules)
    // ========================================

    // "next Monday", "next Tuesday", etc.
    next_day_of_week(b, "next Monday", r"(?i)next\s+mondays?", Weekday::Mon);
    next_day_of_week(b, "next Tuesday", r"(?i)next\s+tuesdays?", Weekday::Tue);
    next_day_of_week(b, "next Wednesday", r"(?i)next\s+wed?nesdays?", Weekday::Wed);
    next_day_of_week(b, "next Thursday", r"(?i)next\s+thursdays?", Weekday::Thu);
    next_day_of_week(b, "next Friday", r"(?i)next\s+fridays?", Weekday::Fri);
    next_day_of_week(b, "next Saturday", r"(?i)next\s+saturdays?", Weekday::Sat);
    next_day_of_week(b, "next Sunday", r"(?i)next\s+sundays?", Weekday::Sun);

    // "last Monday", "last Tuesday", etc.
    last_day_of_week(b, "last Monday", r"(?i)last\s+mondays?", Weekday::Mon);
    last_day_of_week(b, "last Tuesday", r"(?i)last\s+tuesdays?", Weekday::Tue);
    last_day_of_week(b, "last Wednesday", r"(?i)last\s+wed?nesdays?", Weekday::Wed);
    last_day_of_week(b, "last Thursday", r"(?i)last\s+thursdays?", Weekday::Thu);
    last_day_of_week(b, "last Friday", r"(?i)last\s+fridays?", Weekday::Fri);
    last_day_of_week(b, "last Saturday", r"(?i)last\s+saturdays?", Weekday::Sat);
    last_day_of_week(b, "last Sunday", r"(?i)last\s+sundays?", Weekday::Sun);

    // ========================================
    // Intervals - Basic patterns (3 rules)
    // ========================================
    // Note: Full interval support requires composite rules
    // These are simplified single-regex versions

    // "from <time> to <time>" - simplified for times only
    b.rule_1_terminal(
        "en:time:from_to_hours",
        b.reg(r"(?i)from\s+(\d{1,2})(?::(\d{2}))?\s*(am|pm)?\s+to\s+(\d{1,2})(?::(\d{2}))?\s*(am|pm)?").unwrap(),
        |text_match| {
            // Parse start time
            let start_hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Failed to parse hour: {}", e))?;
            let start_minute: u32 = text_match.group(2).parse().unwrap_or(0);
            let start_am_pm = text_match.group(3).to_lowercase();

            // Parse end time
            let end_hour: u32 = text_match.group(4).parse()
                .map_err(|e| rustling_error!("Failed to parse hour: {}", e))?;
            let end_minute: u32 = text_match.group(5).parse().unwrap_or(0);
            let end_am_pm = text_match.group(6).to_lowercase();

            // Convert to 24-hour format
            let start_hour_24 = if !start_am_pm.is_empty() {
                if start_am_pm == "am" {
                    if start_hour == 12 { 0 } else { start_hour }
                } else {
                    if start_hour == 12 { 12 } else { start_hour + 12 }
                }
            } else {
                start_hour
            };

            let end_hour_24 = if !end_am_pm.is_empty() {
                if end_am_pm == "am" {
                    if end_hour == 12 { 0 } else { end_hour }
                } else {
                    if end_hour == 12 { 12 } else { end_hour + 12 }
                }
            } else {
                end_hour
            };

            let now = Utc::now();
            let from_dt = now.date_naive().and_hms_opt(start_hour_24, start_minute, 0)
                .ok_or_else(|| rustling_error!("Invalid start time"))?;
            let to_dt = now.date_naive().and_hms_opt(end_hour_24, end_minute, 0)
                .ok_or_else(|| rustling_error!("Invalid end time"))?;

            let from = TimeData::new(Utc.from_utc_datetime(&from_dt), Grain::Minute);
            let to = TimeData::new(Utc.from_utc_datetime(&to_dt), Grain::Minute);

            Ok(Value::Time(TimeValue::Interval { from, to }))
        }
    );

    // ========================================
    // Simple Intersect - Day + Part of Day (28 rules)
    // ========================================

    // Monday
    intersect_dow_part_of_day(b, "Monday morning", r"(?i)mondays?\s+mornings?", Weekday::Mon, 8);
    intersect_dow_part_of_day(b, "Monday afternoon", r"(?i)mondays?\s+afternoons?", Weekday::Mon, 15);
    intersect_dow_part_of_day(b, "Monday evening", r"(?i)mondays?\s+evenings?", Weekday::Mon, 18);
    intersect_dow_part_of_day(b, "Monday night", r"(?i)mondays?\s+nights?", Weekday::Mon, 21);

    // Tuesday
    intersect_dow_part_of_day(b, "Tuesday morning", r"(?i)tuesdays?\s+mornings?", Weekday::Tue, 8);
    intersect_dow_part_of_day(b, "Tuesday afternoon", r"(?i)tuesdays?\s+afternoons?", Weekday::Tue, 15);
    intersect_dow_part_of_day(b, "Tuesday evening", r"(?i)tuesdays?\s+evenings?", Weekday::Tue, 18);
    intersect_dow_part_of_day(b, "Tuesday night", r"(?i)tuesdays?\s+nights?", Weekday::Tue, 21);

    // Wednesday
    intersect_dow_part_of_day(b, "Wednesday morning", r"(?i)wed?nesdays?\s+mornings?", Weekday::Wed, 8);
    intersect_dow_part_of_day(b, "Wednesday afternoon", r"(?i)wed?nesdays?\s+afternoons?", Weekday::Wed, 15);
    intersect_dow_part_of_day(b, "Wednesday evening", r"(?i)wed?nesdays?\s+evenings?", Weekday::Wed, 18);
    intersect_dow_part_of_day(b, "Wednesday night", r"(?i)wed?nesdays?\s+nights?", Weekday::Wed, 21);

    // Thursday
    intersect_dow_part_of_day(b, "Thursday morning", r"(?i)thursdays?\s+mornings?", Weekday::Thu, 8);
    intersect_dow_part_of_day(b, "Thursday afternoon", r"(?i)thursdays?\s+afternoons?", Weekday::Thu, 15);
    intersect_dow_part_of_day(b, "Thursday evening", r"(?i)thursdays?\s+evenings?", Weekday::Thu, 18);
    intersect_dow_part_of_day(b, "Thursday night", r"(?i)thursdays?\s+nights?", Weekday::Thu, 21);

    // Friday
    intersect_dow_part_of_day(b, "Friday morning", r"(?i)fridays?\s+mornings?", Weekday::Fri, 8);
    intersect_dow_part_of_day(b, "Friday afternoon", r"(?i)fridays?\s+afternoons?", Weekday::Fri, 15);
    intersect_dow_part_of_day(b, "Friday evening", r"(?i)fridays?\s+evenings?", Weekday::Fri, 18);
    intersect_dow_part_of_day(b, "Friday night", r"(?i)fridays?\s+nights?", Weekday::Fri, 21);

    // Saturday
    intersect_dow_part_of_day(b, "Saturday morning", r"(?i)saturdays?\s+mornings?", Weekday::Sat, 8);
    intersect_dow_part_of_day(b, "Saturday afternoon", r"(?i)saturdays?\s+afternoons?", Weekday::Sat, 15);
    intersect_dow_part_of_day(b, "Saturday evening", r"(?i)saturdays?\s+evenings?", Weekday::Sat, 18);
    intersect_dow_part_of_day(b, "Saturday night", r"(?i)saturdays?\s+nights?", Weekday::Sat, 21);

    // Sunday
    intersect_dow_part_of_day(b, "Sunday morning", r"(?i)sundays?\s+mornings?", Weekday::Sun, 8);
    intersect_dow_part_of_day(b, "Sunday afternoon", r"(?i)sundays?\s+afternoons?", Weekday::Sun, 15);
    intersect_dow_part_of_day(b, "Sunday evening", r"(?i)sundays?\s+evenings?", Weekday::Sun, 18);
    intersect_dow_part_of_day(b, "Sunday night", r"(?i)sundays?\s+nights?", Weekday::Sun, 21);

    // ========================================
    // Additional Useful Patterns (6 rules)
    // ========================================

    // "tonight"
    b.rule_1_terminal(
        "en:time:tonight",
        b.reg(r"(?i)tonights?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(21, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "this morning", "this afternoon", "this evening"
    b.rule_1_terminal(
        "en:time:this_morning",
        b.reg(r"(?i)this\s+mornings?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(8, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    b.rule_1_terminal(
        "en:time:this_afternoon",
        b.reg(r"(?i)this\s+afternoons?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(15, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    b.rule_1_terminal(
        "en:time:this_evening",
        b.reg(r"(?i)this\s+evenings?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(18, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "noon", "midnight"
    b.rule_1_terminal(
        "en:time:noon",
        b.reg(r"(?i)noons?|middays?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(12, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::TimeOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    b.rule_1_terminal(
        "en:time:midnight",
        b.reg(r"(?i)midnights?").unwrap(),
        |_| {
            let now = Utc::now();
            let dt = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::TimeOfDay);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // TODO: More complex rules requiring multi-token matching
    // - Month + day combinations (February 15th)
    // - Year + month combinations (2024 February)
    // - Full composite rules from Duckling
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

/// Helper: Create a next day of week rule
fn next_day_of_week(
    b: &RuleSetBuilder<Value>,
    name: &'static str,
    pattern: &str,
    weekday: Weekday,
) {
    let rule_name = format!("en:time:{}", name.replace(" ", "_").to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            let now = Utc::now();
            let current_weekday = now.weekday();

            // Calculate days until next occurrence of target weekday (at least 1 day)
            let days_until = ((weekday.number_from_monday() as i32)
                - (current_weekday.number_from_monday() as i32)
                + 7) % 7;

            let days_until = if days_until == 0 { 7 } else { days_until };

            let target_date = now + Duration::days(days_until as i64);
            let start_of_day = target_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);

            let time_data = TimeData::new(dt, Grain::Day)
                .with_form(Form::DayOfWeek);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Helper: Create a last day of week rule
fn last_day_of_week(
    b: &RuleSetBuilder<Value>,
    name: &'static str,
    pattern: &str,
    weekday: Weekday,
) {
    let rule_name = format!("en:time:{}", name.replace(" ", "_").to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            let now = Utc::now();
            let current_weekday = now.weekday();

            // Calculate days back to last occurrence of target weekday (at least 1 day)
            let days_back = ((current_weekday.number_from_monday() as i32)
                - (weekday.number_from_monday() as i32)
                + 7) % 7;

            let days_back = if days_back == 0 { 7 } else { days_back };

            let target_date = now - Duration::days(days_back as i64);
            let start_of_day = target_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);

            let time_data = TimeData::new(dt, Grain::Day)
                .with_form(Form::DayOfWeek);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Helper: Create an intersect rule for day of week + part of day
fn intersect_dow_part_of_day(
    b: &RuleSetBuilder<Value>,
    name: &'static str,
    pattern: &str,
    weekday: Weekday,
    hour: u32,
) {
    let rule_name = format!("en:time:{}", name.replace(" ", "_").to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            let now = Utc::now();
            let current_weekday = now.weekday();

            // Calculate days until target weekday
            let days_until = ((weekday.number_from_monday() as i32)
                - (current_weekday.number_from_monday() as i32)
                + 7) % 7;

            let target_date = if days_until == 0 {
                now
            } else {
                now + Duration::days(days_until as i64)
            };

            let dt = target_date.date_naive().and_hms_opt(hour, 0, 0).unwrap();
            let dt_utc = Utc.from_utc_datetime(&dt);

            let time_data = TimeData::new(dt_utc, Grain::Hour)
                .with_form(Form::PartOfDay);

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

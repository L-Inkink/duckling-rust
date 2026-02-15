// English Time rules
// Manually implemented based on Duckling/Time/EN/Rules.hs
//
// Status: Phase 2.2 - Pilot implementation
// Coverage: Core rules (named-days, named-months, simple patterns)

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};

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

    // TODO: More rules to be implemented
    // - Year patterns (2024, '24)
    // - Date patterns (MM/DD, DD/MM/YYYY)
    // - Time of day (3pm, 15:00)
    // - Relative time (next week, last month)
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

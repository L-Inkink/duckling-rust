// Chinese Time rules
// Implementing Phase 2.3: ZH Time Rules (163+ rules total)
//
// Following TDD: Tests written FIRST, implementation follows
// Pattern: Based on successful EN Time implementation (languages/en/time.rs)
//
// Phase 1 (In Progress - 25 rules):
// - Simple time references (12 rules): local_ref, today, tomorrow, yesterday, etc.
// - Days of week (7 rules): Monday-Sunday in Chinese
// - Relative time periods (6 rules): last/next week/month/year

use crate::values::Value;
use rustling_core::time::{Form, Grain, TimeContext, TimeData, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use std::sync::Arc;

/// Build Chinese Time rules
///
/// Accepts an optional TimeContext for reference time.
/// If not provided, uses current system local time.
///
/// # Parameters
/// - `b`: RuleSetBuilder for registering rules
/// - `context`: Optional TimeContext (defaults to Local::local_ref() if None)
///
/// # Phases
/// - Phase 1 (20 rules): Simple time references
/// - Phase 2 (25 rules): Hour/minute expressions
/// - Phase 3 (18 rules): Composite patterns
/// - Phase 4 (13 rules): Durations & intervals
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {
    // Use provided context or create default from current local time
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));
    // ========================================
    // Simple Time References (12 rules)
    // ========================================

    // "现在/現在/此时/此刻/当前/當前/宜家/而家/依家" - local_ref
    let ctx_now: Arc<TimeContext> = Arc::clone(&ctx);
    let ctx_local_ref = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:local_ref",
        b.reg(r"现在|現在|此时|此時|此刻|当前|當前|宜家|而家|依家").unwrap(),
        move |_| Ok(Value::Time(TimeValue::instant(ctx_now.reference_utc(), Grain::Second)))
    );

    // "今天/今日" - today
    let ctx_today: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:today",
        b.reg(r"今天|今日").unwrap(),
        move |_| {
            // Use local date to get the user's "today"
            let local_ref = ctx_today.reference_local();
            let start_of_day = local_ref.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "明天/明日/聽日" - tomorrow
    let ctx_tomorrow: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:tomorrow",
        b.reg(r"明天|明日|聽日").unwrap(),
        move |_| {
            let local_ref = ctx_tomorrow.reference_local();
            let tomorrow = local_ref + Duration::days(1);
            let start_of_day = tomorrow.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "昨天/昨日/尋日" - yesterday
    let ctx_yesterday: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:yesterday",
        b.reg(r"昨天|昨日|尋日").unwrap(),
        move |_| {
            let local_ref = ctx_yesterday.reference_local();
            let yesterday = local_ref - Duration::days(1);
            let start_of_day = yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "后天/後天/後日" - day after tomorrow
    let ctx_day_after: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:day_after_tomorrow",
        b.reg(r"后天|後天|後日").unwrap(),
        move |_| {
            let local_ref = ctx_day_after.reference_local();
            let target = local_ref + Duration::days(2);
            let start_of_day = target.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "大后天/大後天/大後日" - three days from local_ref
    let ctx_three_days: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:three_days_from_now",
        b.reg(r"大后天|大後天|大後日").unwrap(),
        move |_| {
            let local_ref = ctx_three_days.reference_local();
            let target = local_ref + Duration::days(3);
            let start_of_day = target.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "前天/前日" - day before yesterday
    let ctx_day_before: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:day_before_yesterday",
        b.reg(r"前天|前日").unwrap(),
        move |_| {
            let local_ref = ctx_day_before.reference_local();
            let target = local_ref - Duration::days(2);
            let start_of_day = target.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // ========================================
    // Named Days of Week (7 rules)
    // ========================================
    // Supports multiple formats: 星期X, 礼拜X, 周X, 禮拜X, 週X

    named_day_of_week_zh(b, Arc::clone(&ctx), "Monday", r"星期一|礼拜一|周一|禮拜一|週一", Weekday::Mon);
    named_day_of_week_zh(b, Arc::clone(&ctx), "Tuesday", r"星期二|礼拜二|周二|禮拜二|週二", Weekday::Tue);
    named_day_of_week_zh(b, Arc::clone(&ctx), "Wednesday", r"星期三|礼拜三|周三|禮拜三|週三", Weekday::Wed);
    named_day_of_week_zh(b, Arc::clone(&ctx), "Thursday", r"星期四|礼拜四|周四|禮拜四|週四", Weekday::Thu);
    named_day_of_week_zh(b, Arc::clone(&ctx), "Friday", r"星期五|礼拜五|周五|禮拜五|週五", Weekday::Fri);
    named_day_of_week_zh(b, Arc::clone(&ctx), "Saturday", r"星期六|礼拜六|周六|禮拜六|週六", Weekday::Sat);
    // Sunday: can be 星期日/星期天, 礼拜日/礼拜天, etc.
    named_day_of_week_zh(b, Arc::clone(&ctx), "Sunday", r"星期[日天]|礼拜[日天]|周[日天]|禮拜[日天]|週[日天]", Weekday::Sun);

    // ========================================
    // Relative Time Periods (6 rules)
    // ========================================

    // "上周/上週/上个星期/上個星期" - last week
    let ctx_last_week: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:last_week",
        b.reg(r"上周|上週|上个星期|上個星期").unwrap(),
        move |_| {
            let local_ref = ctx_last_week.reference_local();
            let dt = local_ref - Duration::weeks(1);

            // Round to start of week (Monday)
            let weekday = dt.weekday().num_days_from_monday();
            let days_back = Duration::days(weekday as i64);
            let dt_rounded = (dt - days_back).date_naive().and_hms_opt(0, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid date"))?;

            let time_data = TimeData::new(dt_rounded, Grain::Week);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "下周/下週/下个星期/下個星期" - next week
    let ctx_next_week: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:next_week",
        b.reg(r"下周|下週|下个星期|下個星期").unwrap(),
        move |_| {
            let local_ref = ctx_next_week.reference_local();
            let dt = local_ref + Duration::weeks(1);

            // Round to start of week (Monday)
            let weekday = dt.weekday().num_days_from_monday();
            let days_back = Duration::days(weekday as i64);
            let dt_rounded = (dt - days_back).date_naive().and_hms_opt(0, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid date"))?;

            let time_data = TimeData::new(dt_rounded, Grain::Week);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "上个月/上個月" - last month
    let ctx_last_month: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:last_month",
        b.reg(r"上个月|上個月").unwrap(),
        move |_| {
            let local_ref = ctx_last_month.reference_local();
            let dt = local_ref.checked_sub_months(chrono::Months::new(1))
                .ok_or_else(|| rustling_error!("Invalid month calculation"))?;

            // Round to start of month
            let dt_rounded = dt.with_day(1)
                .and_then(|d| d.with_hour(0))
                .and_then(|d| d.with_minute(0))
                .and_then(|d| d.with_second(0))
                .ok_or_else(|| rustling_error!("Invalid date"))?
                .with_timezone(&Utc);

            let time_data = TimeData::new(dt_rounded, Grain::Month);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "下个月/下個月" - next month
    let ctx_next_month: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:next_month",
        b.reg(r"下个月|下個月").unwrap(),
        move |_| {
            let local_ref = ctx_next_month.reference_local();
            let dt = local_ref.checked_add_months(chrono::Months::new(1))
                .ok_or_else(|| rustling_error!("Invalid month calculation"))?;

            // Round to start of month
            let dt_rounded = dt.with_day(1)
                .and_then(|d| d.with_hour(0))
                .and_then(|d| d.with_minute(0))
                .and_then(|d| d.with_second(0))
                .ok_or_else(|| rustling_error!("Invalid date"))?
                .with_timezone(&Utc);

            let time_data = TimeData::new(dt_rounded, Grain::Month);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "去年/上年" - last year
    let ctx_last_year: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:last_year",
        b.reg(r"去年|上年").unwrap(),
        move |_| {
            let local_ref = ctx_last_year.reference_local();
            let dt = Utc.with_ymd_and_hms(local_ref.year() - 1, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid year calculation"))?;

            let time_data = TimeData::new(dt, Grain::Year);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "明年/下年" - next year
    let ctx_next_year: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:next_year",
        b.reg(r"明年|下年").unwrap(),
        move |_| {
            let local_ref = ctx_next_year.reference_local();
            let dt = Utc.with_ymd_and_hms(local_ref.year() + 1, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid year calculation"))?;

            let time_data = TimeData::new(dt, Grain::Year);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // ========================================
    // Phase 2: Time of Day & Hour/Minute Patterns (35 rules)
    // ========================================

    add_part_of_day_rules(b, Arc::clone(&ctx));
    add_hour_minute_rules(b, Arc::clone(&ctx));

    // ========================================
    // Phase 3: Composite Patterns & Intersections (18 rules)
    // ========================================

    add_month_day_rules(b, Arc::clone(&ctx));
    add_year_rules(b, Arc::clone(&ctx));
    add_intersection_rules(b, Arc::clone(&ctx));

    // Phase 4: Duration and interval rules (需要 context)
    add_duration_interval_rules(b, ctx);
}

// ========================================
// Helper Functions
// ========================================

/// Helper: Create a named day of week rule for Chinese
///
/// Supports multiple Chinese formats:
/// - 星期X (xīngqī) - most common
/// - 礼拜X (lǐbài) - colloquial
/// - 周X (zhōu) - shortened form
/// - 禮拜X (traditional)
/// - 週X (traditional)
fn named_day_of_week_zh(
    b: &RuleSetBuilder<Value>,
    ctx: Arc<TimeContext>,
    name: &'static str,
    pattern: &str,
    weekday: Weekday,
) {
    let rule_name = format!("zh:time:{}", name.to_lowercase());

    b.rule_1_terminal(
        &rule_name,
        b.reg(pattern).unwrap(),
        move |_| {
            // Get the next occurrence of this weekday
            let local_ref = ctx.reference_local();
            let current_weekday = local_ref.weekday();

            // Calculate days until target weekday
            let days_until = ((weekday.number_from_monday() as i32)
                - (current_weekday.number_from_monday() as i32)
                + 7) % 7;

            let target_date = if days_until == 0 {
                // If it's the same weekday, use today
                local_ref
            } else {
                local_ref + Duration::days(days_until as i64)
            };

            let start_of_day = target_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);

            let time_data = TimeData::new(dt, Grain::Day)
                .with_form(Form::DayOfWeek);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Parse Chinese numeral to integer (0-99)
/// Supports: 零/〇(0), 一(1), 二/两/兩(2), 三(3), 四(4), 五(5), 六(6), 七(7), 八(8), 九(9), 十(10), etc.
fn parse_chinese_number(s: &str) -> Option<u32> {
    let num_map: std::collections::HashMap<char, u32> = [
        ('零', 0), ('〇', 0),
        ('一', 1), ('二', 2), ('两', 2), ('兩', 2), ('三', 3), ('四', 4),
        ('五', 5), ('六', 6), ('七', 7), ('八', 8), ('九', 9),
        ('十', 10),
    ].iter().cloned().collect();

    // Handle simple single digits
    if s.len() == 3 { // One Chinese character = 3 bytes in UTF-8
        return s.chars().next().and_then(|c| num_map.get(&c).copied());
    }

    // Handle 十X (10-19)
    if s.starts_with('十') {
        if s.len() == 3 {
            return Some(10);
        }
        let rest: String = s.chars().skip(1).collect();
        return parse_chinese_number(&rest).map(|n| 10 + n);
    }

    // Handle X十 (20, 30, 40, etc.)
    if s.contains('十') {
        let parts: Vec<&str> = s.split('十').collect();
        if parts.len() == 2 {
            let tens = parse_chinese_number(parts[0]).unwrap_or(1);
            let ones = if parts[1].is_empty() {
                0
            } else {
                parse_chinese_number(parts[1]).unwrap_or(0)
            };
            return Some(tens * 10 + ones);
        }
    }

    None
}

/// Add Part of Day rules (6 rules)
fn add_part_of_day_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // "凌晨" - dawn (0-4am)
    let ctx_dawn = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:dawn",
        b.reg(r"凌晨").unwrap(),
        move |_| {
            let local_ref = ctx_dawn.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(2, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "早上/早晨/朝早" - morning (4-12)
    let ctx_morning = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:morning",
        b.reg(r"早上|早晨|朝早").unwrap(),
        move |_| {
            let local_ref = ctx_morning.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(8, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "中午" - noon (12)
    let ctx_noon = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:noon",
        b.reg(r"中午|正午").unwrap(),
        move |_| {
            let local_ref = ctx_noon.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(12, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "下午/晏晝" - afternoon (12-18)
    let ctx_afternoon = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:afternoon",
        b.reg(r"下午|晏晝").unwrap(),
        move |_| {
            let local_ref = ctx_afternoon.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(15, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "晚上/晚间/夜晚" - evening (18-24)
    let ctx_evening = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:evening",
        b.reg(r"晚上|晚间|晚間|夜晚").unwrap(),
        move |_| {
            let local_ref = ctx_evening.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(20, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "半夜/午夜/子夜/半夜三更" - midnight (0)
    let ctx_midnight = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:midnight",
        b.reg(r"半夜三更|半夜|午夜|子夜").unwrap(),
        move |_| {
            let local_ref = ctx_midnight.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(0, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour)
                .with_form(Form::PartOfDay);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Add Hour and Minute rules (29 rules)
fn add_hour_minute_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // "X点" / "X點" - X o'clock (Chinese numerals)
    let ctx_hour_cn = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_cn",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;

            if hour > 23 {
                return Err(rustling_error!("Hour out of range: {}", hour));
            }

            let local_ref = ctx_hour_cn.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点" - X o'clock (numeric)
    let ctx_hour_num = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_num",
        b.reg(r"(\d{1,2})[点點]").unwrap(),
        move |text_match| {
            let hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid hour: {}", e))?;

            if hour > 23 {
                return Err(rustling_error!("Hour out of range: {}", hour));
            }

            let local_ref = ctx_hour_num.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, 0, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Hour);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点Y分" - hour:minute (Chinese numerals)
    let ctx_hour_minute_cn = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_minute_cn",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四)[点點](零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|三十|四十|五十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九|三十一|三十二|三十三|三十四|三十五|三十六|三十七|三十八|三十九|四十一|四十二|四十三|四十四|四十五|四十六|四十七|四十八|四十九|五十一|五十二|五十三|五十四|五十五|五十六|五十七|五十八|五十九)分").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let minute_str = text_match.group(2);

            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;
            let minute = parse_chinese_number(minute_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese minute: {}", minute_str))?;

            if hour > 23 || minute > 59 {
                return Err(rustling_error!("Time out of range: {}:{}", hour, minute));
            }

            let local_ref = ctx_hour_minute_cn.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点Y分" - hour:minute (numeric)
    let ctx_hour_minute_num = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_minute_num",
        b.reg(r"(\d{1,2})[点點](\d{1,2})分").unwrap(),
        move |text_match| {
            let hour: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid hour: {}", e))?;
            let minute: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid minute: {}", e))?;

            if hour > 23 || minute > 59 {
                return Err(rustling_error!("Time out of range"));
            }

            let local_ref = ctx_hour_minute_num.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点零Y分" - hour:0Y (with 零 for single digit minutes)
    let ctx_hour_minute_zero = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_minute_zero",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]零(一|二|三|四|五|六|七|八|九)分").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let minute_str = text_match.group(2);

            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;
            let minute = parse_chinese_number(minute_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese minute: {}", minute_str))?;

            if hour > 23 || minute > 9 {
                return Err(rustling_error!("Time out of range"));
            }

            let local_ref = ctx_hour_minute_zero.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点差Y分" - Y minutes before X
    let ctx_hour_minus_minutes = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_minus_minutes",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四)[点點]差(一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|三十|四十|五十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九)分").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let minus_str = text_match.group(2);

            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;
            let minus = parse_chinese_number(minus_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese minute: {}", minus_str))?;

            if hour > 23 || minus > 59 {
                return Err(rustling_error!("Time out of range"));
            }

            // "X点差Y分" means Y minutes before X
            // e.g., "3点差5分" = 2:55
            let actual_hour = if minus >= hour as u32 * 60 {
                (hour + 23) % 24
            } else if minus > (hour % 1) * 60 {
                hour - 1
            } else {
                hour
            };
            let actual_minute = (60 - minus) % 60;

            let local_ref = ctx_hour_minus_minutes.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(actual_hour, actual_minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点一刻" - quarter past X (X:15)
    let ctx_hour_quarter = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_quarter",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]一刻").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;

            if hour > 23 {
                return Err(rustling_error!("Hour out of range"));
            }

            let local_ref = ctx_hour_quarter.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, 15, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点半" - half past X (X:30)
    let ctx_hour_half = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_half",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]半").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;

            if hour > 23 {
                return Err(rustling_error!("Hour out of range"));
            }

            let local_ref = ctx_hour_half.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, 30, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点踏/搭Y" - Cantonese 5-minute units (X + Y*5 minutes)
    let ctx_cantonese_tap_dap = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:cantonese_tap_dap",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點][踏搭](十一|十|一|二|三|四|五|六|七|八|九)").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let unit_str = text_match.group(2);

            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;
            let unit = parse_chinese_number(unit_str)
                .ok_or_else(|| rustling_error!("Invalid 5-min unit: {}", unit_str))?;

            if hour > 23 || unit > 11 {
                return Err(rustling_error!("Time out of range"));
            }

            let minute = unit * 5;

            let local_ref = ctx_cantonese_tap_dap.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X点Y個字" - Cantonese "zi" units (Y*5 minutes)
    let ctx_cantonese_zi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:cantonese_zi",
        b.reg(r"(零|一|二|两|兩|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點](十一|十|一|二|三|四|五|六|七|八|九)個?字").unwrap(),
        move |text_match| {
            let hour_str = text_match.group(1);
            let unit_str = text_match.group(2);

            let hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese hour: {}", hour_str))?;
            let unit = parse_chinese_number(unit_str)
                .ok_or_else(|| rustling_error!("Invalid zi unit: {}", unit_str))?;

            if hour > 23 || unit > 11 {
                return Err(rustling_error!("Time out of range"));
            }

            let minute = unit * 5;

            let local_ref = ctx_cantonese_zi.reference_local();
            let today = local_ref.date_naive();
            let dt = today.and_hms_opt(hour, minute, 0)
                .map(|naive| Utc.from_utc_datetime(&naive))
                .ok_or_else(|| rustling_error!("Invalid time"))?;

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "一刻" - quarter hour (15 minutes) standalone
    let ctx_quarter_standalone = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:quarter_standalone",
        b.reg(r"一刻钟?").unwrap(),
        move |_| {
            let local_ref = ctx_quarter_standalone.reference_local();
            let dt = local_ref.with_minute(15).unwrap().with_second(0).unwrap().with_timezone(&Utc);

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "三刻" - three quarters (45 minutes)
    let ctx_three_quarters = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:three_quarters",
        b.reg(r"三刻钟?").unwrap(),
        move |_| {
            let local_ref = ctx_three_quarters.reference_local();
            let dt = local_ref.with_minute(45).unwrap().with_second(0).unwrap().with_timezone(&Utc);

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "半小时/半個小時" - half hour (30 minutes) standalone
    let ctx_half_hour_standalone = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:half_hour_standalone",
        b.reg(r"半小[时時]|半個小[时時]").unwrap(),
        move |_| {
            let local_ref = ctx_half_hour_standalone.reference_local();
            let dt = local_ref.with_minute(30).unwrap().with_second(0).unwrap().with_timezone(&Utc);

            let time_data = TimeData::new(dt, Grain::Minute);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Add Month-Day rules (8 rules)
fn add_month_day_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // "X月Y日" - numeric month and day
    let ctx_month_day_numeric = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:month_day_numeric",
        b.reg(r"(\d{1,2})月(\d{1,2})[日号號]").unwrap(),
        move |text_match| {
            let month: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;

            if month == 0 || month > 12 || day == 0 || day > 31 {
                return Err(rustling_error!("Date out of range: {}/{}", month, day));
            }

            let local_ref = ctx_month_day_numeric.reference_local();
            let year = local_ref.year();
            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "X月Y日" - Chinese numeral month and day
    let ctx_month_day_chinese = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:month_day_chinese",
        b.reg(r"(一|二|三|四|五|六|七|八|九|十|十一|十二)月(一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九|三十|三十一)[日号號]").unwrap(),
        move |text_match| {
            let month_str = text_match.group(1);
            let day_str = text_match.group(2);

            let month = parse_chinese_number(month_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese month: {}", month_str))?;
            let day = parse_chinese_number(day_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese day: {}", day_str))?;

            if month == 0 || month > 12 || day == 0 || day > 31 {
                return Err(rustling_error!("Date out of range: {}/{}", month, day));
            }

            let local_ref = ctx_month_day_chinese.reference_local();
            let year = local_ref.year();
            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "Y号" - day only (use current month)
    let ctx_day_only = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:day_only",
        b.reg(r"^(\d{1,2})[号號]$").unwrap(),
        move |text_match| {
            let day: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;

            if day == 0 || day > 31 {
                return Err(rustling_error!("Day out of range: {}", day));
            }

            let local_ref = ctx_day_only.reference_local();
            let year = local_ref.year();
            let month = local_ref.month();
            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Add Year rules (5 rules)
fn add_year_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // "YYYY年" - numeric year
    let ctx_year_numeric = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:year_numeric",
        b.reg(r"(1\d{3}|20\d{2}|2100)年").unwrap(),
        move |text_match| {
            let year: i32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid year: {}", e))?;

            let dt = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid year: {}", year))?;

            let time_data = TimeData::new(dt, Grain::Year)
                .with_form(Form::Year);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "二零二四年" - Chinese digit year (e.g., 2024 = 二零二四)
    b.rule_1_terminal(
        "zh:time:year_chinese_digits",
        b.reg(r"([一二三四五六七八九零〇]{4})年").unwrap(),
        move |text_match| {
            let year_str = text_match.group(1);
            let year = parse_chinese_year(year_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese year: {}", year_str))?;

            let dt = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid year: {}", year))?;

            let time_data = TimeData::new(dt, Grain::Year)
                .with_form(Form::Year);

            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "YYYY年X月Y日" - full date numeric
    b.rule_1_terminal(
        "zh:time:year_month_day_numeric",
        b.reg(r"(1\d{3}|20\d{2}|2100)年(\d{1,2})月(\d{1,2})[日号號]").unwrap(),
        move |text_match| {
            let year: i32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid year: {}", e))?;
            let month: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day: u32 = text_match.group(3).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;

            if month == 0 || month > 12 || day == 0 || day > 31 {
                return Err(rustling_error!("Date out of range"));
            }

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "二零二四年三月十五日" - full date Chinese
    b.rule_1_terminal(
        "zh:time:year_month_day_chinese",
        b.reg(r"([一二三四五六七八九零〇]{4})年(一|二|三|四|五|六|七|八|九|十|十一|十二)月(一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四|二十五|二十六|二十七|二十八|二十九|三十|三十一)[日号號]").unwrap(),
        move |text_match| {
            let year_str = text_match.group(1);
            let month_str = text_match.group(2);
            let day_str = text_match.group(3);

            let year = parse_chinese_year(year_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese year: {}", year_str))?;
            let month = parse_chinese_number(month_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese month: {}", month_str))?;
            let day = parse_chinese_number(day_str)
                .ok_or_else(|| rustling_error!("Invalid Chinese day: {}", day_str))?;

            if month == 0 || month > 12 || day == 0 || day > 31 {
                return Err(rustling_error!("Date out of range"));
            }

            let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month, day))?;

            let time_data = TimeData::new(dt, Grain::Day);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Add Intersection rules (5 rules)
fn add_intersection_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // "星期X早上" - DOW + Part of Day
    // For each DOW × Part of Day combination
    let dow_patterns = vec![
        ("一", Weekday::Mon),
        ("二", Weekday::Tue),
        ("三", Weekday::Wed),
        ("四", Weekday::Thu),
        ("五", Weekday::Fri),
        ("六", Weekday::Sat),
        ("[日天]", Weekday::Sun),
    ];

    let pod_patterns = vec![
        ("凌晨", 2),
        ("早上|早晨|朝早", 8),
        ("中午|正午", 12),
        ("下午|晏晝", 15),
        ("晚上|晚间|晚間|夜晚", 20),
    ];

    for (dow_pat, weekday) in &dow_patterns {
        for (pod_pat, hour) in &pod_patterns {
            let pattern = format!(r"(?:星期|礼拜|禮拜|周|週){}(?:{})", dow_pat, pod_pat);
            let rule_name = format!("zh:time:dow_{:?}_pod_{}", weekday, hour);

            let weekday_copy = *weekday;
            let hour_copy = *hour;
            let ctx_dow_pod = Arc::clone(&ctx);

            b.rule_1_terminal(
                &rule_name,
                b.reg(&pattern).unwrap(),
                move |_| {
                    let local_ref = ctx_dow_pod.reference_local();
                    let current_weekday = local_ref.weekday();

                    // Calculate days until target weekday
                    let days_until = ((weekday_copy.number_from_monday() as i32)
                        - (current_weekday.number_from_monday() as i32)
                        + 7) % 7;

                    let target_date = if days_until == 0 {
local_ref
                    } else {
                        local_ref + Duration::days(days_until as i64)
                    };

                    let dt = target_date.date_naive().and_hms_opt(hour_copy, 0, 0).unwrap();
                    let dt_utc = Utc.from_utc_datetime(&dt);

                    let time_data = TimeData::new(dt_utc, Grain::Hour);
                    Ok(Value::Time(TimeValue::Instant(time_data)))
                }
            );
        }
    }

    // "星期X三点" - DOW + Hour
    for (dow_pat, weekday) in &dow_patterns {
        let pattern = format!(r"(?:星期|礼拜|禮拜|周|週){}(零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]", dow_pat);
        let rule_name = format!("zh:time:dow_{:?}_hour", weekday);

        let weekday_copy = *weekday;
        let ctx_dow_hour = Arc::clone(&ctx);

        b.rule_1_terminal(
            &rule_name,
            b.reg(&pattern).unwrap(),
            move |text_match| {
                // Group 1 is the hour
                let hour_str = text_match.group(1);

                let hour = parse_chinese_number(hour_str)
                    .ok_or_else(|| rustling_error!("Invalid hour: {}", hour_str))?;

                let local_ref = ctx_dow_hour.reference_local();
                let current_weekday = local_ref.weekday();

                let days_until = ((weekday_copy.number_from_monday() as i32)
                    - (current_weekday.number_from_monday() as i32)
                    + 7) % 7;

                let target_date = if days_until == 0 {
local_ref
                } else {
                    local_ref + Duration::days(days_until as i64)
                };

                let dt = target_date.date_naive().and_hms_opt(hour, 0, 0).unwrap();
                let dt_utc = Utc.from_utc_datetime(&dt);

                let time_data = TimeData::new(dt_utc, Grain::Hour);
                Ok(Value::Time(TimeValue::Instant(time_data)))
            }
        );
    }

    // "3月15日下午" - Month-Day + Part of Day
    let ctx_month_day_pod = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:month_day_pod",
        b.reg(r"(\d{1,2})月(\d{1,2})[日号號](凌晨|早上|早晨|朝早|中午|正午|下午|晏晝|晚上|晚间|晚間|夜晚)").unwrap(),
        move |text_match| {
            let month: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;
            let pod_str = text_match.group(3);

            let hour = match pod_str {
                "凌晨" => 2,
                "早上" | "早晨" | "朝早" => 8,
                "中午" | "正午" => 12,
                "下午" | "晏晝" => 15,
                "晚上" | "晚间" | "晚間" | "夜晚" => 20,
                _ => return Err(rustling_error!("Unknown part of day")),
            };

            let local_ref = ctx_month_day_pod.reference_local();
            let year = local_ref.year();
            let dt = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{} {}:00", year, month, day, hour))?;

            let time_data = TimeData::new(dt, Grain::Hour);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );

    // "2024年3月15日下午三点" - Full datetime
    b.rule_1_terminal(
        "zh:time:full_datetime",
        b.reg(r"(1\d{3}|20\d{2}|2100)年(\d{1,2})月(\d{1,2})[日号號](凌晨|早上|早晨|中午|下午|晚上|晚间|夜晚)?(零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三)[点點]").unwrap(),
        move |text_match| {
            let year: i32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid year: {}", e))?;
            let month: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day: u32 = text_match.group(3).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;
            let pod_str = text_match.group(4);
            let hour_str = text_match.group(5);

            let mut hour = parse_chinese_number(hour_str)
                .ok_or_else(|| rustling_error!("Invalid hour: {}", hour_str))?;

            // Adjust hour based on part of day
            if !pod_str.is_empty() {
                match pod_str {
                    "下午" | "晚上" | "晚间" => {
                        if hour < 12 {
                            hour += 12;
                        }
                    }
                    _ => {}
                }
            }

            let dt = Utc.with_ymd_and_hms(year, month, day, hour, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid datetime"))?;

            let time_data = TimeData::new(dt, Grain::Hour);
            Ok(Value::Time(TimeValue::Instant(time_data)))
        }
    );
}

/// Add Duration and Interval rules (13 rules) - Phase 4
fn add_duration_interval_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // ========================================
    // Phase 4: 持续时间 & 区间 (13 rules)
    // ========================================

    // "X天前" - X days ago
    let ctx_days_ago: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:days_ago",
        b.reg(r"(\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|三十|四十|五十|六十|七十|八十|九十)天前").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let days = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            // Use local reference time for date calculation
            let local_ref = ctx_days_ago.reference_local();
            let target = local_ref - Duration::days(days);
            let start_of_day = target.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "X天后/X天後" - X days from local_ref
    let ctx_days_from: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:days_from_now",
        b.reg(r"(\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|三十|四十|五十|六十|七十|八十|九十)天[后後]").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let days = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            let local_ref = ctx_days_from.reference_local();
            let target = local_ref + Duration::days(days);
            let start_of_day = target.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&start_of_day);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Day)))
        }
    );

    // "X小时前/X小時前" - X hours ago
    let ctx_hours_ago: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hours_ago",
        b.reg(r"(两|兩|\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四)(?:小时|小時)前").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let hours = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if num_str == "两" || num_str == "兩" {
                2
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            let dt = ctx_hours_ago.reference_utc() - Duration::hours(hours);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Hour)))
        }
    );

    // "X小时后/X小時後" - X hours from local_ref
    let ctx_hours_from: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hours_from_now",
        b.reg(r"(两|兩|\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|二十四)(?:小时|小時)[后後]").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let hours = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if num_str == "两" || num_str == "兩" {
                2
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            let dt = ctx_hours_from.reference_utc() + Duration::hours(hours);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Hour)))
        }
    );

    // "X分钟前/X分鐘前" - X minutes ago
    let ctx_minutes_ago: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:minutes_ago",
        b.reg(r"(\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|二十|三十|四十|五十)(?:分钟|分鐘)前").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let minutes = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            let dt = ctx_minutes_ago.reference_utc() - Duration::minutes(minutes);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Minute)))
        }
    );

    // "X分钟后/X分鐘後" - X minutes from local_ref
    let ctx_minutes_from: Arc<TimeContext> = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:minutes_from_now",
        b.reg(r"(\d+|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|二十|三十|四十|五十)(?:分钟|分鐘)[后後]").unwrap(),
        move |text_match| {
            let num_str = text_match.group(1);
            let minutes = if let Ok(n) = num_str.parse::<i64>() {
                n
            } else if let Some(n) = parse_chinese_number(num_str) {
                n as i64
            } else {
                return Err(rustling_error!("Invalid number: {}", num_str));
            };

            let dt = ctx_minutes_from.reference_utc() + Duration::minutes(minutes);
            Ok(Value::Time(TimeValue::instant(dt, Grain::Minute)))
        }
    );

    // "这周末/這週末" - this weekend
    let ctx_this_weekend = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:this_weekend",
        b.reg(r"(?:这|這)(?:个|個)?(?:周末|週末)").unwrap(),
        move |_| {
            let local_ref = ctx_this_weekend.reference_local();
            let current_weekday = local_ref.weekday();

            // Calculate days until Saturday
            let days_until_saturday = match current_weekday {
                Weekday::Mon => 5,
                Weekday::Tue => 4,
                Weekday::Wed => 3,
                Weekday::Thu => 2,
                Weekday::Fri => 1,
                Weekday::Sat => 0,
                Weekday::Sun => 6, // Next Saturday
            };

            let saturday = local_ref + Duration::days(days_until_saturday);
            let sunday = saturday + Duration::days(1);

            let saturday_start = saturday.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let sunday_end = sunday.date_naive().and_hms_opt(23, 59, 59).unwrap();

            let from_dt = Utc.from_utc_datetime(&saturday_start);
            let to_dt = Utc.from_utc_datetime(&sunday_end);

            let from_data = TimeData::new(from_dt, Grain::Day);
            let to_data = TimeData::new(to_dt, Grain::Day);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "下周末/下週末" - next weekend
    let ctx_next_weekend = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:next_weekend",
        b.reg(r"下(?:周末|週末)").unwrap(),
        move |_| {
            let local_ref = ctx_next_weekend.reference_local();
            let current_weekday = local_ref.weekday();

            // Always go to next week's Saturday
            let days_until_next_saturday = match current_weekday {
                Weekday::Mon => 12,
                Weekday::Tue => 11,
                Weekday::Wed => 10,
                Weekday::Thu => 9,
                Weekday::Fri => 8,
                Weekday::Sat => 7,
                Weekday::Sun => 6,
            };

            let saturday = local_ref + Duration::days(days_until_next_saturday);
            let sunday = saturday + Duration::days(1);

            let saturday_start = saturday.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let sunday_end = sunday.date_naive().and_hms_opt(23, 59, 59).unwrap();

            let from_dt = Utc.from_utc_datetime(&saturday_start);
            let to_dt = Utc.from_utc_datetime(&sunday_end);

            let from_data = TimeData::new(from_dt, Grain::Day);
            let to_data = TimeData::new(to_dt, Grain::Day);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "上周末/上週末" - last weekend
    let ctx_last_weekend = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:last_weekend",
        b.reg(r"上(?:周末|週末)").unwrap(),
        move |_| {
            let local_ref = ctx_last_weekend.reference_local();
            let current_weekday = local_ref.weekday();

            // Calculate days back to last Saturday
            let days_back_to_saturday = match current_weekday {
                Weekday::Mon => 2,
                Weekday::Tue => 3,
                Weekday::Wed => 4,
                Weekday::Thu => 5,
                Weekday::Fri => 6,
                Weekday::Sat => 7,
                Weekday::Sun => 1,
            };

            let saturday = local_ref - Duration::days(days_back_to_saturday);
            let sunday = saturday + Duration::days(1);

            let saturday_start = saturday.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let sunday_end = sunday.date_naive().and_hms_opt(23, 59, 59).unwrap();

            let from_dt = Utc.from_utc_datetime(&saturday_start);
            let to_dt = Utc.from_utc_datetime(&sunday_end);

            let from_data = TimeData::new(from_dt, Grain::Day);
            let to_data = TimeData::new(to_dt, Grain::Day);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "从X点到Y点" / "從X點到Y點" - interval from hour to hour
    let ctx_hour_interval_from_to = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_interval_from_to",
        b.reg(r"[从從](零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|\d{1,2})[点點]到(零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|\d{1,2})[点點]").unwrap(),
        move |text_match| {
            let local_ref = ctx_hour_interval_from_to.reference_local();
            let hour1_str = text_match.group(1);
            let hour2_str = text_match.group(2);

            let hour1 = if let Ok(n) = hour1_str.parse::<u32>() {
                n
            } else if let Some(n) = parse_chinese_number(hour1_str) {
                n
            } else {
                return Err(rustling_error!("Invalid hour: {}", hour1_str));
            };

            let hour2 = if let Ok(n) = hour2_str.parse::<u32>() {
                n
            } else if let Some(n) = parse_chinese_number(hour2_str) {
                n
            } else {
                return Err(rustling_error!("Invalid hour: {}", hour2_str));
            };

            let today = local_ref.date_naive();

            let from_dt = today.and_hms_opt(hour1, 0, 0)
                .ok_or_else(|| rustling_error!("Invalid hour: {}", hour1))?;
            let to_dt = today.and_hms_opt(hour2, 0, 0)
                .ok_or_else(|| rustling_error!("Invalid hour: {}", hour2))?;

            let from = Utc.from_utc_datetime(&from_dt);
            let to = Utc.from_utc_datetime(&to_dt);

            let from_data = TimeData::new(from, Grain::Hour);
            let to_data = TimeData::new(to, Grain::Hour);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "X点至Y点" - interval using 至
    let ctx_hour_interval_zhi = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:hour_interval_zhi",
        b.reg(r"(零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|\d{1,2})[点點]至(零|一|二|三|四|五|六|七|八|九|十|十一|十二|十三|十四|十五|十六|十七|十八|十九|二十|二十一|二十二|二十三|\d{1,2})[点點]").unwrap(),
        move |text_match| {
            let local_ref = ctx_hour_interval_zhi.reference_local();
            let hour1_str = text_match.group(1);
            let hour2_str = text_match.group(2);

            let hour1 = if let Ok(n) = hour1_str.parse::<u32>() {
                n
            } else if let Some(n) = parse_chinese_number(hour1_str) {
                n
            } else {
                return Err(rustling_error!("Invalid hour: {}", hour1_str));
            };

            let hour2 = if let Ok(n) = hour2_str.parse::<u32>() {
                n
            } else if let Some(n) = parse_chinese_number(hour2_str) {
                n
            } else {
                return Err(rustling_error!("Invalid hour: {}", hour2_str));
            };

            let today = local_ref.date_naive();

            let from_dt = today.and_hms_opt(hour1, 0, 0)
                .ok_or_else(|| rustling_error!("Invalid hour: {}", hour1))?;
            let to_dt = today.and_hms_opt(hour2, 0, 0)
                .ok_or_else(|| rustling_error!("Invalid hour: {}", hour2))?;

            let from = Utc.from_utc_datetime(&from_dt);
            let to = Utc.from_utc_datetime(&to_dt);

            let from_data = TimeData::new(from, Grain::Hour);
            let to_data = TimeData::new(to, Grain::Hour);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "从X月Y日到A月B日" - date interval
    let ctx_date_interval = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:date_interval",
        b.reg(r"[从從](\d{1,2})月(\d{1,2})[日号號]到(\d{1,2})月(\d{1,2})[日号號]").unwrap(),
        move |text_match| {
            let month1: u32 = text_match.group(1).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day1: u32 = text_match.group(2).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;
            let month2: u32 = text_match.group(3).parse()
                .map_err(|e| rustling_error!("Invalid month: {}", e))?;
            let day2: u32 = text_match.group(4).parse()
                .map_err(|e| rustling_error!("Invalid day: {}", e))?;

            let local_ref = ctx_date_interval.reference_local();
            let year = local_ref.year();

            let from_dt = Utc.with_ymd_and_hms(year, month1, day1, 0, 0, 0)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month1, day1))?;
            let to_dt = Utc.with_ymd_and_hms(year, month2, day2, 23, 59, 59)
                .single()
                .ok_or_else(|| rustling_error!("Invalid date: {}/{}/{}", year, month2, day2))?;

            let from_data = TimeData::new(from_dt, Grain::Day);
            let to_data = TimeData::new(to_dt, Grain::Day);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );

    // "从星期X到星期Y" - weekday interval
    let ctx_dow_interval = Arc::clone(&ctx);
    b.rule_1_terminal(
        "zh:time:dow_interval",
        b.reg(r"[从從](?:星期|礼拜|禮拜|周|週)(一|二|三|四|五|六|日|天)到(?:星期|礼拜|禮拜|周|週)(一|二|三|四|五|六|日|天)").unwrap(),
        move |text_match| {
            let local_ref = ctx_dow_interval.reference_local();
            let dow1_str = text_match.group(1);
            let dow2_str = text_match.group(2);

            let dow1 = match dow1_str {
                "一" => Weekday::Mon,
                "二" => Weekday::Tue,
                "三" => Weekday::Wed,
                "四" => Weekday::Thu,
                "五" => Weekday::Fri,
                "六" => Weekday::Sat,
                "日" | "天" => Weekday::Sun,
                _ => return Err(rustling_error!("Invalid weekday: {}", dow1_str)),
            };

            let dow2 = match dow2_str {
                "一" => Weekday::Mon,
                "二" => Weekday::Tue,
                "三" => Weekday::Wed,
                "四" => Weekday::Thu,
                "五" => Weekday::Fri,
                "六" => Weekday::Sat,
                "日" | "天" => Weekday::Sun,
                _ => return Err(rustling_error!("Invalid weekday: {}", dow2_str)),
            };

            let current_weekday = local_ref.weekday();

            // Calculate days until the first weekday
            let days_until_dow1 = ((dow1.num_days_from_monday() as i64)
                - (current_weekday.num_days_from_monday() as i64) + 7) % 7;
            let days_until_dow2 = ((dow2.num_days_from_monday() as i64)
                - (current_weekday.num_days_from_monday() as i64) + 7) % 7;

            let from_date = local_ref + Duration::days(days_until_dow1);
            let to_date = local_ref + Duration::days(days_until_dow2);

            let from_dt_naive = from_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let to_dt_naive = to_date.date_naive().and_hms_opt(23, 59, 59).unwrap();

            let from_dt = Utc.from_utc_datetime(&from_dt_naive);
            let to_dt = Utc.from_utc_datetime(&to_dt_naive);

            let from_data = TimeData::new(from_dt, Grain::Day);
            let to_data = TimeData::new(to_dt, Grain::Day);

            Ok(Value::Time(TimeValue::Interval { from: from_data, to: to_data }))
        }
    );
}

/// Parse Chinese year digits (e.g., "二零二四" -> 2024)
fn parse_chinese_year(s: &str) -> Option<i32> {
    let digit_map: std::collections::HashMap<char, i32> = [
        ('零', 0), ('〇', 0),
        ('一', 1), ('二', 2), ('三', 3), ('四', 4),
        ('五', 5), ('六', 6), ('七', 7), ('八', 8), ('九', 9),
    ].iter().cloned().collect();

    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 4 {
        return None;
    }

    let mut year = 0;
    for (i, &ch) in chars.iter().enumerate() {
        let digit = *digit_map.get(&ch)?;
        year += digit * 10_i32.pow(3 - i as u32);
    }

    Some(year)
}

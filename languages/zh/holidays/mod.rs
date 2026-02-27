//! 中文节假日规则
//!
//! 支持15个核心节假日：
//! - 固定日期：元旦、国庆、劳动节、儿童节、妇女节等
//! - 农历节日：春节、端午、中秋
//! - 节气节日：清明
//! - 国际节日：情人节、圣诞节、万圣节

pub mod data;
pub mod fixed;
pub mod lunar;
pub mod solar;

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeValue};
use rustling_core::{RuleSetBuilder, rustling_error};
use chrono::{DateTime, Datelike, Utc};
use std::sync::Arc;

/// 节假日规则辅助函数
///
/// 为给定的节假日创建一个规则
fn mk_holiday_rule<F>(
    b: &RuleSetBuilder<Value>,
    ctx: Arc<TimeContext>,
    name: &str,
    regex_pattern: &str,
    date_fn: F,
) where
    F: Fn(i32) -> Option<DateTime<Utc>> + Send + Sync + 'static,
{
    let ctx_clone = Arc::clone(&ctx);
    let pattern = format!("zh:time:holiday:{}", name);

    b.rule_1_terminal(
        &pattern,
        b.reg(regex_pattern).unwrap(),
        move |_| {
            // 获取参考时间的年份
            let year = ctx_clone.reference_utc().year();

            // 查找节假日日期
            let date = date_fn(year)
                .ok_or_else(|| rustling_error!("Holiday date not found for year {}", year))?;

            Ok(Value::Time(TimeValue::instant(date, Grain::Day)))
        },
    );
}

/// 注册所有节假日规则
pub fn rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // ========================================
    // 固定日期节假日（中国）
    // ========================================

    mk_holiday_rule(
        b,
        ctx.clone(),
        "new_year",
        "元旦(节|節)?|((公|(阳|陽))(历|曆))?新年",
        fixed::new_year,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "national_day",
        "国庆(节|節)?|國慶(节|節)?",
        fixed::national_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "labor_day",
        "(五一|51)?(国际|國際)?(劳动|勞動)(节|節)",
        fixed::labor_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "childrens_day",
        "(国际|國際)?(六一|61)?(儿|兒)童(节|節)",
        fixed::childrens_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "womens_day",
        "(国际劳动|國際勞動|三八)?(妇|婦)女(节|節)",
        fixed::womens_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "youth_day",
        "(五四)?青年(节|節)",
        fixed::youth_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "teachers_day",
        "(中(国|國))?教师(节|節)",
        fixed::teachers_day,
    );

    // ========================================
    // 农历节假日
    // ========================================

    mk_holiday_rule(
        b,
        ctx.clone(),
        "spring_festival",
        "春(节|節)|(农历|農曆|唐人)新年|新(正|春)|正月(正(时|時)|朔日)|岁首",
        lunar::spring_festival,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "dragon_boat",
        "端午(节|節)",
        lunar::dragon_boat_festival,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "mid_autumn",
        "中秋(节|節)",
        lunar::mid_autumn_festival,
    );

    // ========================================
    // 节气节假日
    // ========================================

    mk_holiday_rule(
        b,
        ctx.clone(),
        "qingming",
        "清明(节|節)",
        solar::qingming,
    );

    // ========================================
    // 国际节假日
    // ========================================

    mk_holiday_rule(
        b,
        ctx.clone(),
        "valentines",
        "(情人|(圣瓦伦丁|聖瓦倫丁))(节|節)",
        fixed::valentines_day,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "christmas",
        "(圣诞|聖誕)(节|節)?",
        fixed::christmas,
    );

    mk_holiday_rule(
        b,
        ctx.clone(),
        "halloween",
        "万圣(节|節)前夜|萬聖(节|節)前夜",
        fixed::halloween,
    );
}

/// 预热缓存（可选，用于性能敏感场景）
#[allow(dead_code)]
pub fn warmup_cache() {
    lunar::warmup_cache();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holiday_functions() {
        // 固定日期
        assert!(fixed::new_year(2024).is_some());
        assert!(fixed::national_day(2024).is_some());

        // 农历
        assert!(lunar::spring_festival(2024).is_some());
        assert!(lunar::dragon_boat_festival(2024).is_some());

        // 节气
        assert!(solar::qingming(2024).is_some());
    }
}

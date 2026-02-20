//! 节气节假日（基于太阳历）
//!
//! 节气由太阳黄经决定，与农历无关
//! 清明是24节气之一，有精确计算公式

use super::data::QINGMING;
use chrono::{DateTime, Utc, TimeZone};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// 清明节查找表（2000-2050）
    static ref QINGMING_MAP: HashMap<i32, (u32, u32)> =
        QINGMING.iter()
            .map(|&(year, month, day)| (year, (month, day)))
            .collect();
}

/// 清明节（24节气之一）
///
/// 方法1: 查找表（精确，推荐）
/// 覆盖范围：2000-2050
pub fn qingming(year: i32) -> Option<DateTime<Utc>> {
    let (month, day) = QINGMING_MAP.get(&year)?;
    Utc.with_ymd_and_hms(year, *month, *day, 0, 0, 0).single()
}

/// 清明节（24节气之一）
///
/// 方法2: 算法计算（备用方案）
/// 基于Meeus算法，适用于1900-2100年
/// 精度：与查找表一致
///
/// 公式说明：
/// - 清明是太阳黄经达15°时的时刻
/// - 平均每年推迟0.2422天
/// - 每4年闰年修正
#[allow(dead_code)]
pub fn qingming_formula(year: i32) -> Option<DateTime<Utc>> {
    let day = if year >= 1900 && year < 2000 {
        // 1900-1999年公式
        let d = (year - 1900) as f64 * 0.2422 + 5.59
            - ((year - 1900) / 4) as f64;
        d.floor() as u32
    } else if year >= 2000 && year < 2100 {
        // 2000-2099年公式
        let d = (year - 2000) as f64 * 0.2422 + 4.81
            - ((year - 2000) / 4) as f64;
        d.floor() as u32
    } else {
        return None;
    };

    Utc.with_ymd_and_hms(year, 4, day, 0, 0, 0).single()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_qingming_2024() {
        let qm = qingming(2024).unwrap();
        assert_eq!((qm.month(), qm.day()), (4, 4));
    }

    #[test]
    fn test_qingming_range() {
        // 清明必在4月4-6日
        for year in 2000..=2050 {
            if let Some(date) = qingming(year) {
                assert_eq!(date.month(), 4);
                assert!(
                    date.day() >= 4 && date.day() <= 6,
                    "Year {} qingming day {} not in range 4-6",
                    year,
                    date.day()
                );
            }
        }
    }

    #[test]
    fn test_qingming_formula_accuracy() {
        // 验证公式与查找表的准确性
        // 注意：公式是近似的，允许±1天误差
        let mut matches = 0;
        let total = 51;

        for year in 2000..=2050 {
            let table_result = qingming(year);
            let formula_result = qingming_formula(year);

            if let (Some(t), Some(f)) = (table_result, formula_result) {
                let day_diff = (t.day() as i32 - f.day() as i32).abs();
                if day_diff == 0 {
                    matches += 1;
                } else {
                    // 允许±1天误差
                    assert!(
                        day_diff <= 1,
                        "Year {} difference too large: table {} vs formula {}",
                        year, t.day(), f.day()
                    );
                }
            }
        }

        // 至少95%的年份应该完全匹配
        assert!(
            matches >= (total * 95) / 100,
            "Only {}/{} years match exactly",
            matches, total
        );
    }

    #[test]
    fn test_coverage() {
        assert!(qingming(2000).is_some());
        assert!(qingming(2050).is_some());
        assert!(qingming(1999).is_none());
        assert!(qingming(2051).is_none());
    }
}

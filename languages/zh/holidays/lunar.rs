//! 农历节假日（基于查找表）
//!
//! 这些节假日基于农历，每年的公历日期不同
//! 使用lazy_static + HashMap实现O(1)查找

use super::data::*;
use chrono::{DateTime, Utc, TimeZone};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// 春节查找表（1950-2050）
    static ref SPRING_FESTIVAL_MAP: HashMap<i32, (u32, u32)> =
        build_map(SPRING_FESTIVAL);

    /// 端午节查找表（2000-2050）
    static ref DRAGON_BOAT_MAP: HashMap<i32, (u32, u32)> =
        build_map(DRAGON_BOAT);

    /// 中秋节查找表（2000-2050）
    static ref MID_AUTUMN_MAP: HashMap<i32, (u32, u32)> =
        build_map(MID_AUTUMN);
}

/// 构建HashMap的辅助函数
fn build_map(data: &[(i32, u32, u32)]) -> HashMap<i32, (u32, u32)> {
    data.iter()
        .map(|&(year, month, day)| (year, (month, day)))
        .collect()
}

/// 春节（农历正月初一）
///
/// 覆盖范围：1950-2050
pub fn spring_festival(year: i32) -> Option<DateTime<Utc>> {
    let (month, day) = SPRING_FESTIVAL_MAP.get(&year)?;
    Utc.with_ymd_and_hms(year, *month, *day, 0, 0, 0).single()
}

/// 端午节（农历五月初五）
///
/// 覆盖范围：2000-2050
pub fn dragon_boat_festival(year: i32) -> Option<DateTime<Utc>> {
    let (month, day) = DRAGON_BOAT_MAP.get(&year)?;
    Utc.with_ymd_and_hms(year, *month, *day, 0, 0, 0).single()
}

/// 中秋节（农历八月十五）
///
/// 覆盖范围：2000-2050
pub fn mid_autumn_festival(year: i32) -> Option<DateTime<Utc>> {
    let (month, day) = MID_AUTUMN_MAP.get(&year)?;
    Utc.with_ymd_and_hms(year, *month, *day, 0, 0, 0).single()
}

/// 预热缓存（可选，用于性能敏感场景）
pub fn warmup_cache() {
    // 触发lazy_static初始化
    let _ = &*SPRING_FESTIVAL_MAP;
    let _ = &*DRAGON_BOAT_MAP;
    let _ = &*MID_AUTUMN_MAP;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_spring_festival_2024() {
        let sf = spring_festival(2024).unwrap();
        assert_eq!((sf.month(), sf.day()), (2, 10));
    }

    #[test]
    fn test_dragon_boat_2024() {
        let db = dragon_boat_festival(2024).unwrap();
        assert_eq!((db.month(), db.day()), (6, 10));
    }

    #[test]
    fn test_mid_autumn_2024() {
        let ma = mid_autumn_festival(2024).unwrap();
        assert_eq!((ma.month(), ma.day()), (9, 17));
    }

    #[test]
    fn test_spring_festival_range() {
        // 春节必在1月21日至2月20日之间
        for year in 1950..=2050 {
            if let Some(date) = spring_festival(year) {
                let month = date.month();
                let day = date.day();
                assert!(
                    (month == 1 && day >= 21) || (month == 2 && day <= 20),
                    "Year {} spring festival {}/{} out of range",
                    year, month, day
                );
            }
        }
    }

    #[test]
    fn test_coverage() {
        // 春节：1950-2050
        assert!(spring_festival(1950).is_some());
        assert!(spring_festival(2050).is_some());
        assert!(spring_festival(1949).is_none());
        assert!(spring_festival(2051).is_none());

        // 端午/中秋：2000-2050
        assert!(dragon_boat_festival(2000).is_some());
        assert!(dragon_boat_festival(2050).is_some());
        assert!(dragon_boat_festival(1999).is_none());

        assert!(mid_autumn_festival(2000).is_some());
        assert!(mid_autumn_festival(2050).is_some());
        assert!(mid_autumn_festival(1999).is_none());
    }

    #[test]
    fn test_warmup() {
        // 确保warmup不会panic
        warmup_cache();
    }
}

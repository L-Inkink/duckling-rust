//! 固定日期节假日
//!
//! 这些节假日每年在公历的同一天

use chrono::{DateTime, Utc, TimeZone};

/// 元旦（1月1日）
pub fn new_year(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).single()
}

/// 国庆节（10月1日）
pub fn national_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 10, 1, 0, 0, 0).single()
}

/// 劳动节（5月1日）
pub fn labor_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 5, 1, 0, 0, 0).single()
}

/// 儿童节（6月1日）
pub fn childrens_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 6, 1, 0, 0, 0).single()
}

/// 妇女节（3月8日）
pub fn womens_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 3, 8, 0, 0, 0).single()
}

/// 青年节（5月4日）
pub fn youth_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 5, 4, 0, 0, 0).single()
}

/// 教师节（9月10日）
pub fn teachers_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 9, 10, 0, 0, 0).single()
}

/// 情人节（2月14日）
pub fn valentines_day(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 2, 14, 0, 0, 0).single()
}

/// 圣诞节（12月25日）
pub fn christmas(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 12, 25, 0, 0, 0).single()
}

/// 万圣节（10月31日）
pub fn halloween(year: i32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, 10, 31, 0, 0, 0).single()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_fixed_holidays_2024() {
        let year = 2024;

        let ny = new_year(year).unwrap();
        assert_eq!((ny.month(), ny.day()), (1, 1));

        let nd = national_day(year).unwrap();
        assert_eq!((nd.month(), nd.day()), (10, 1));

        let ld = labor_day(year).unwrap();
        assert_eq!((ld.month(), ld.day()), (5, 1));

        let cd = childrens_day(year).unwrap();
        assert_eq!((cd.month(), cd.day()), (6, 1));

        let wd = womens_day(year).unwrap();
        assert_eq!((wd.month(), wd.day()), (3, 8));

        let yd = youth_day(year).unwrap();
        assert_eq!((yd.month(), yd.day()), (5, 4));

        let td = teachers_day(year).unwrap();
        assert_eq!((td.month(), td.day()), (9, 10));

        let vd = valentines_day(year).unwrap();
        assert_eq!((vd.month(), vd.day()), (2, 14));

        let xmas = christmas(year).unwrap();
        assert_eq!((xmas.month(), xmas.day()), (12, 25));

        let hw = halloween(year).unwrap();
        assert_eq!((hw.month(), hw.day()), (10, 31));
    }

    #[test]
    fn test_year_consistency() {
        // 验证返回的年份正确
        for year in 2000..=2050 {
            assert_eq!(new_year(year).unwrap().year(), year);
            assert_eq!(national_day(year).unwrap().year(), year);
        }
    }
}

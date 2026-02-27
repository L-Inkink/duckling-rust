use serde::{Deserialize, Serialize};

/// Time granularity representation
///
/// Grain represents the level of precision for time values, from seconds
/// to years. This is used to determine how time values are interpreted,
/// compared, and manipulated.
///
/// # Examples
///
/// ```
/// use rustling_core::time::Grain;
///
/// let day_grain = Grain::Day;
/// let month_grain = Grain::Month;
///
/// assert!(month_grain.coarser_than(&day_grain));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Grain {
    /// Second-level precision
    Second = 0,
    /// Minute-level precision
    Minute = 1,
    /// Hour-level precision
    Hour = 2,
    /// Day-level precision
    Day = 3,
    /// Week-level precision
    Week = 4,
    /// Month-level precision
    Month = 5,
    /// Quarter-level precision (3 months)
    Quarter = 6,
    /// Year-level precision
    Year = 7,
}

impl Grain {
    /// Returns true if this grain is coarser (less precise) than the other
    ///
    /// # Examples
    ///
    /// ```
    /// use rustling_core::time::Grain;
    ///
    /// assert!(Grain::Month.coarser_than(&Grain::Day));
    /// assert!(!Grain::Hour.coarser_than(&Grain::Day));
    /// ```
    pub fn coarser_than(&self, other: &Grain) -> bool {
        (*self as u8) > (*other as u8)
    }

    /// Returns true if this grain is finer (more precise) than the other
    ///
    /// # Examples
    ///
    /// ```
    /// use rustling_core::time::Grain;
    ///
    /// assert!(Grain::Minute.finer_than(&Grain::Hour));
    /// assert!(!Grain::Week.finer_than(&Grain::Day));
    /// ```
    pub fn finer_than(&self, other: &Grain) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Returns the coarser (less precise) of the two grains
    ///
    /// # Examples
    ///
    /// ```
    /// use rustling_core::time::Grain;
    ///
    /// assert_eq!(Grain::coarser(Grain::Day, Grain::Hour), Grain::Day);
    /// ```
    pub fn coarser(g1: Grain, g2: Grain) -> Grain {
        if g1.coarser_than(&g2) { g1 } else { g2 }
    }

    /// Returns the finer (more precise) of the two grains
    ///
    /// # Examples
    ///
    /// ```
    /// use rustling_core::time::Grain;
    ///
    /// assert_eq!(Grain::finer(Grain::Day, Grain::Hour), Grain::Hour);
    /// ```
    pub fn finer(g1: Grain, g2: Grain) -> Grain {
        if g1.finer_than(&g2) { g1 } else { g2 }
    }

    /// Converts grain to a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Grain::Second => "second",
            Grain::Minute => "minute",
            Grain::Hour => "hour",
            Grain::Day => "day",
            Grain::Week => "week",
            Grain::Month => "month",
            Grain::Quarter => "quarter",
            Grain::Year => "year",
        }
    }
}

impl std::fmt::Display for Grain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grain_ordering() {
        assert!(Grain::Second < Grain::Minute);
        assert!(Grain::Minute < Grain::Hour);
        assert!(Grain::Hour < Grain::Day);
        assert!(Grain::Day < Grain::Week);
        assert!(Grain::Week < Grain::Month);
        assert!(Grain::Month < Grain::Quarter);
        assert!(Grain::Quarter < Grain::Year);
    }

    #[test]
    fn test_coarser_finer() {
        assert!(Grain::Year.coarser_than(&Grain::Day));
        assert!(Grain::Second.finer_than(&Grain::Hour));

        assert_eq!(Grain::coarser(Grain::Day, Grain::Month), Grain::Month);
        assert_eq!(Grain::finer(Grain::Day, Grain::Month), Grain::Day);
    }

    #[test]
    fn test_display() {
        assert_eq!(Grain::Day.to_string(), "day");
        assert_eq!(Grain::Month.to_string(), "month");
    }
}

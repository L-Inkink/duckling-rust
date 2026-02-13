pub mod duration;
pub mod time;

pub use duration::{DurationValue, TimeUnit};
pub use time::TimeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Duration(DurationValue),
    Time(TimeValue),
}

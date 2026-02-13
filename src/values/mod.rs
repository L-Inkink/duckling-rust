pub mod duration;
pub mod time;

pub use duration::{DurationValue, TimeUnit};
pub use time::TimeValue;
use serde::{Deserialize, Serialize};
use rustling_core::{NodePayload, StashIndexable};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Duration(DurationValue),
    Time(TimeValue),
}

impl NodePayload for Value {
    type Payload = Value;

    fn extract_payload(&self) -> Option<Self::Payload> {
        Some(self.clone())
    }
}

impl StashIndexable for Value {
    type Index = Value;

    fn index(&self) -> Self::Index {
        self.clone()
    }
}

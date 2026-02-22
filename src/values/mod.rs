pub mod duration;
pub mod time;

pub use duration::{DurationValue, TimeUnit};
pub use time::TimeValue;
use serde::{Deserialize, Serialize};
use rustling_core::{NodePayload, StashIndexable};
use std::fmt::Debug;
use std::hash::Hash;

// ValueKind: A lightweight enum for indexing
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Integer,
    Float,
    Duration,
    Time,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Duration(DurationValue),
    Time(TimeValue),
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Integer(_) => ValueKind::Integer,
            Value::Float(_) => ValueKind::Float,
            Value::Duration(_) => ValueKind::Duration,
            Value::Time(_) => ValueKind::Time,
        }
    }
}

// Custom Eq and Hash implementations for Value (needed because f64 doesn't implement Eq/Hash)
impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Integer(i) => {
                0u8.hash(state);
                i.hash(state);
            }
            Value::Float(f) => {
                1u8.hash(state);
                f.to_bits().hash(state);
            }
            Value::Duration(d) => {
                2u8.hash(state);
                d.hash(state);
            }
            Value::Time(t) => {
                3u8.hash(state);
                t.hash(state);
            }
        }
    }
}

impl NodePayload for Value {
    type Payload = Value;

    fn extract_payload(&self) -> Option<Self::Payload> {
        Some(self.clone())
    }
}

impl StashIndexable for Value {
    type Index = ValueKind;

    fn index(&self) -> Self::Index {
        self.kind()
    }
}

impl rustling_core::InnerStashIndexable for Value {
    type Index = ValueKind;

    fn index() -> Self::Index {
        // Return a placeholder - this is for type-level indexing
        // The actual kind is determined per-value via StashIndexable::index()
        ValueKind::Integer
    }
}

impl rustling_core::AttemptFrom<Value> for Value {
    fn attempt_from(v: Value) -> Option<Self> {
        Some(v)
    }
}

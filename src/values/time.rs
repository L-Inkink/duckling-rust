use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeValue {
    pub timestamp: DateTime<Utc>,
}

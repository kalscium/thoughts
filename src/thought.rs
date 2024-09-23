use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single thought
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Thought(pub String, pub Option<DateTime<Utc>>);

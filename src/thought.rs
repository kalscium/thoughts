use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single thought
#[derive(Serialize, Deserialize)]
pub struct Thought(pub String, pub Option<DateTime<Utc>>);

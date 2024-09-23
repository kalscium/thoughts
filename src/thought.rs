use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub uid: i64,
    pub thought: String,
    pub utc: Option<DateTime<Utc>>,
}

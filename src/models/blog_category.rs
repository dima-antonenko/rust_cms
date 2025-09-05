use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Blog category model
/// Used to organize blog posts into diffrent topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl BlogCategory {
    /// Creates a new blog category
    /// Categories help readers find relevent content
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            created_at: Utc::now(),
        }
    }
}


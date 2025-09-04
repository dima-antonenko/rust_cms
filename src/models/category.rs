use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Product category model
/// Used to organize products into diffrent categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl Category {
    /// Creates a new category
    /// Categories help organize products in the shop
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            created_at: Utc::now(),
        }
    }
}


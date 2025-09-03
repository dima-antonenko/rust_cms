use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a product in the e-commerce system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
}

impl Product {
    /// Creates a new product instance
    pub fn new(name: String, description: String, price: f64, category_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            price,
            category_id,
            created_at: Utc::now(),
        }
    }
}

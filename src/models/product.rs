use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a product in the e-commerce system
/// Contains all necesary information about products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category_id: String,
    pub image_url: Option<String>,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    /// Creates a new product instance
    /// This is the primary constructor for products
    pub fn new(name: String, description: String, price: f64, category_id: String, stock: i32) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            price,
            category_id,
            image_url: None,
            stock,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the product information
    /// Automaticaly sets the updated_at timestamp
    pub fn update(&mut self, name: String, description: String, price: f64, stock: i32) {
        self.name = name;
        self.description = description;
        self.price = price;
        self.stock = stock;
        self.updated_at = Utc::now();
    }

    /// Checks if product is availabe in stock
    pub fn is_available(&self) -> bool {
        self.stock > 0
    }
}

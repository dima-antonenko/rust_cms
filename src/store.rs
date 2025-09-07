use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::models::{Product, Category, Post, BlogCategory};

/// In-memory data store for the CMS
/// This provides a simple storage solution without needing a database
/// In production, this should be replaced with a proper database
#[derive(Clone)]
pub struct Store {
    pub products: Arc<RwLock<HashMap<String, Product>>>,
    pub categories: Arc<RwLock<HashMap<String, Category>>>,
    pub posts: Arc<RwLock<HashMap<String, Post>>>,
    pub blog_categories: Arc<RwLock<HashMap<String, BlogCategory>>>,
}

impl Store {
    /// Creates a new empty store instance
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
            categories: Arc::new(RwLock::new(HashMap::new())),
            posts: Arc::new(RwLock::new(HashMap::new())),
            blog_categories: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initializes the store with some sample data
    /// This is useful for demonstration and testing purposes
    pub fn init_with_sample_data(&self) {
        // Create sample categories
        let electronics = Category::new(
            "Electronics".to_string(),
            "Electronic devices and gadgets".to_string(),
        );
        let clothing = Category::new(
            "Clothing".to_string(),
            "Fashion and apparel".to_string(),
        );
        let books = Category::new(
            "Books".to_string(),
            "Books and educational materials".to_string(),
        );

        let electronics_id = electronics.id.clone();
        let clothing_id = clothing.id.clone();
        let books_id = books.id.clone();

        self.categories.write().unwrap().insert(electronics.id.clone(), electronics);
        self.categories.write().unwrap().insert(clothing.id.clone(), clothing);
        self.categories.write().unwrap().insert(books.id.clone(), books);

        // Create sample products
        let laptop = Product::new(
            "Gaming Laptop".to_string(),
            "High-performance laptop for gaming and professional work".to_string(),
            1299.99,
            electronics_id.clone(),
            15,
        );
        let headphones = Product::new(
            "Wireless Headphones".to_string(),
            "Noise-canceling wireless headphones with premium sound quality".to_string(),
            249.99,
            electronics_id.clone(),
            30,
        );
        let tshirt = Product::new(
            "Cotton T-Shirt".to_string(),
            "Comfortable cotton t-shirt available in multiple colors".to_string(),
            29.99,
            clothing_id.clone(),
            100,
        );
        let rust_book = Product::new(
            "Rust Programming Book".to_string(),
            "Complete guide to Rust programming language".to_string(),
            49.99,
            books_id.clone(),
            50,
        );

        self.products.write().unwrap().insert(laptop.id.clone(), laptop);
        self.products.write().unwrap().insert(headphones.id.clone(), headphones);
        self.products.write().unwrap().insert(tshirt.id.clone(), tshirt);
        self.products.write().unwrap().insert(rust_book.id.clone(), rust_book);

        // Create sample blog categories
        let tech = BlogCategory::new(
            "Technology".to_string(),
            "Tech news and tutorials".to_string(),
        );
        let lifestyle = BlogCategory::new(
            "Lifestyle".to_string(),
            "Lifestyle tips and articles".to_string(),
        );

        let tech_id = tech.id.clone();
        let lifestyle_id = lifestyle.id.clone();

        self.blog_categories.write().unwrap().insert(tech.id.clone(), tech);
        self.blog_categories.write().unwrap().insert(lifestyle.id.clone(), lifestyle);

        // Create sample blog posts
        let mut post1 = Post::new(
            "Getting Started with Rust".to_string(),
            "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. In this article, we'll explore the basics of Rust and why it's becoming increasingly popular among developers.".to_string(),
            "Learn the basics of Rust programming language".to_string(),
            tech_id.clone(),
            "Admin".to_string(),
        );
        post1.publish();

        let mut post2 = Post::new(
            "Building Web Applications".to_string(),
            "Web development with Rust has become much easier with modern frameworks like Axum, Actix-web, and Rocket. This post covers the fundamentals of building web applications using Rust and demonstrates best practices.".to_string(),
            "How to build modern web apps with Rust".to_string(),
            tech_id.clone(),
            "Admin".to_string(),
        );
        post2.publish();

        let mut post3 = Post::new(
            "Productivity Tips for Developers".to_string(),
            "As developers, we're always looking for ways to improve our productivity. Here are some proven strategies that can help you write better code faster and maintain a healthy work-life balance.".to_string(),
            "Improve your coding productivity".to_string(),
            lifestyle_id.clone(),
            "Admin".to_string(),
        );
        post3.publish();

        self.posts.write().unwrap().insert(post1.id.clone(), post1);
        self.posts.write().unwrap().insert(post2.id.clone(), post2);
        self.posts.write().unwrap().insert(post3.id.clone(), post3);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}


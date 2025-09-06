use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Blog post model
/// Represents individual blog posts with content and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub category_id: String,
    pub author: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    /// Creates a new blog post
    /// Posts can be published or saved as drafts
    pub fn new(title: String, content: String, excerpt: String, category_id: String, author: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            excerpt,
            category_id,
            author,
            published: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates post content and metadata
    /// Automaticaly updates the timestamp
    pub fn update(&mut self, title: String, content: String, excerpt: String) {
        self.title = title;
        self.content = content;
        self.excerpt = excerpt;
        self.updated_at = Utc::now();
    }

    /// Publishes the post making it visible to users
    pub fn publish(&mut self) {
        self.published = true;
        self.updated_at = Utc::now();
    }

    /// Unpublishes the post hiding it from users
    pub fn unpublish(&mut self) {
        self.published = false;
        self.updated_at = Utc::now();
    }
}


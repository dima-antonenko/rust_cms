mod models;
mod store;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tracing_subscriber;

use store::Store;
use handlers::*;

/// Main application entry point
/// Initializes the web server and routes
#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create and initialize the data store
    let store = Store::new();
    store.init_with_sample_data();

    // Build the application routes
    // Admin routes for managment interface
    let admin_routes = Router::new()
        .route("/admin", get(admin_dashboard))
        .route("/admin/categories", get(admin_list_categories))
        .route("/admin/categories/create", post(admin_create_category))
        .route("/admin/categories/delete/:id", post(admin_delete_category))
        .route("/admin/products", get(admin_list_products))
        .route("/admin/products/create", post(admin_create_product))
        .route("/admin/products/delete/:id", post(admin_delete_product))
        .route("/admin/blog-categories", get(admin_list_blog_categories))
        .route("/admin/blog-categories/create", post(admin_create_blog_category))
        .route("/admin/blog-categories/delete/:id", post(admin_delete_blog_category))
        .route("/admin/posts", get(admin_list_posts))
        .route("/admin/posts/create", post(admin_create_post))
        .route("/admin/posts/toggle/:id", post(admin_toggle_post))
        .route("/admin/posts/delete/:id", post(admin_delete_post));

    // Public routes for customer-facing pages
    let public_routes = Router::new()
        .route("/", get(public_home))
        .route("/shop", get(public_shop))
        .route("/blog", get(public_blog))
        .route("/blog/:id", get(public_blog_post));

    // Combine all routes and add static file serving
    let app = Router::new()
        .merge(admin_routes)
        .merge(public_routes)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(store);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    println!("Admin panel: http://{}/admin", addr);
    println!("Public site: http://{}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use crate::store::Store;
use crate::models::{Category};

/// Admin dashboard handler
/// Shows overview of the system with statistics
pub async fn admin_dashboard(State(store): State<Store>) -> impl IntoResponse {
    let products_count = store.products.read().unwrap().len();
    let categories_count = store.categories.read().unwrap().len();
    let posts_count = store.posts.read().unwrap().len();
    let blog_categories_count = store.blog_categories.read().unwrap().len();

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Admin Dashboard</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories">Product Categories</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Dashboard Overview</h2>
        <div class="stats-grid">
            <div class="stat-card">
                <h3>Product Categories</h3>
                <p class="stat-number">{}</p>
            </div>
            <div class="stat-card">
                <h3>Products</h3>
                <p class="stat-number">{}</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
        categories_count, products_count
    );

    Html(html)
}

/// Lists all product categories in admin panel
pub async fn admin_list_categories(State(store): State<Store>) -> impl IntoResponse {
    let categories = store.categories.read().unwrap();
    let mut categories_vec: Vec<_> = categories.values().collect();
    categories_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut rows = String::new();
    for cat in categories_vec {
        rows.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>"#,
            cat.name, cat.description, cat.created_at.format("%Y-%m-%d")
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Product Categories - Admin</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories" class="active">Product Categories</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Product Categories</h2>
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Description</th>
                    <th>Created</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        rows
    );

    Html(html)
}

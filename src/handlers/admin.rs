use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use crate::store::Store;
use crate::models::{Product, Category, Post, BlogCategory};

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
            <li><a href="/admin/products">Products</a></li>
            <li><a href="/admin/blog-categories">Blog Categories</a></li>
            <li><a href="/admin/posts">Blog Posts</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Dashboard Overview</h2>
        <div class="stats-grid">
            <div class="stat-card">
                <h3>Product Categories</h3>
                <p class="stat-number">{}</p>
                <a href="/admin/categories" class="btn">Manage</a>
            </div>
            <div class="stat-card">
                <h3>Products</h3>
                <p class="stat-number">{}</p>
                <a href="/admin/products" class="btn">Manage</a>
            </div>
            <div class="stat-card">
                <h3>Blog Categories</h3>
                <p class="stat-number">{}</p>
                <a href="/admin/blog-categories" class="btn">Manage</a>
            </div>
            <div class="stat-card">
                <h3>Blog Posts</h3>
                <p class="stat-number">{}</p>
                <a href="/admin/posts" class="btn">Manage</a>
            </div>
        </div>
    </div>
</body>
</html>"#,
        categories_count, products_count, blog_categories_count, posts_count
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
                <td>
                    <form method="post" action="/admin/categories/delete/{}" style="display:inline;">
                        <button type="submit" class="btn btn-danger">Delete</button>
                    </form>
                </td>
            </tr>"#,
            cat.name, cat.description, cat.created_at.format("%Y-%m-%d"), cat.id
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Product Categories - Admin</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories" class="active">Product Categories</a></li>
            <li><a href="/admin/products">Products</a></li>
            <li><a href="/admin/blog-categories">Blog Categories</a></li>
            <li><a href="/admin/posts">Blog Posts</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Product Categories</h2>
        <form method="post" action="/admin/categories/create" class="create-form">
            <input type="text" name="name" placeholder="Category Name" required>
            <input type="text" name="description" placeholder="Description" required>
            <button type="submit" class="btn">Create Category</button>
        </form>
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Description</th>
                    <th>Created</th>
                    <th>Actions</th>
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

#[derive(Deserialize)]
pub struct CreateCategoryForm {
    name: String,
    description: String,
}

/// Creates a new product category
pub async fn admin_create_category(
    State(store): State<Store>,
    Form(form): Form<CreateCategoryForm>,
) -> impl IntoResponse {
    let category = Category::new(form.name, form.description);
    store.categories.write().unwrap().insert(category.id.clone(), category);
    Redirect::to("/admin/categories")
}

/// Deletes a product category
pub async fn admin_delete_category(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    store.categories.write().unwrap().remove(&id);
    Redirect::to("/admin/categories")
}

/// Lists all products in admin panel
pub async fn admin_list_products(State(store): State<Store>) -> impl IntoResponse {
    let products = store.products.read().unwrap();
    let categories = store.categories.read().unwrap();
    let mut products_vec: Vec<_> = products.values().collect();
    products_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut rows = String::new();
    for product in products_vec {
        let category_name = categories
            .get(&product.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown");

        rows.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>${:.2}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <form method="post" action="/admin/products/delete/{}" style="display:inline;">
                        <button type="submit" class="btn btn-danger">Delete</button>
                    </form>
                </td>
            </tr>"#,
            product.name, category_name, product.price, product.stock, product.created_at.format("%Y-%m-%d"), product.id
        ));
    }

    let mut category_options = String::new();
    for cat in categories.values() {
        category_options.push_str(&format!(r#"<option value="{}">{}</option>"#, cat.id, cat.name));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Products - Admin</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories">Product Categories</a></li>
            <li><a href="/admin/products" class="active">Products</a></li>
            <li><a href="/admin/blog-categories">Blog Categories</a></li>
            <li><a href="/admin/posts">Blog Posts</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Products</h2>
        <form method="post" action="/admin/products/create" class="create-form">
            <input type="text" name="name" placeholder="Product Name" required>
            <input type="text" name="description" placeholder="Description" required>
            <input type="number" step="0.01" name="price" placeholder="Price" required>
            <input type="number" name="stock" placeholder="Stock" required>
            <select name="category_id" required>
                <option value="">Select Category</option>
                {}
            </select>
            <button type="submit" class="btn">Create Product</button>
        </form>
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Category</th>
                    <th>Price</th>
                    <th>Stock</th>
                    <th>Created</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        category_options, rows
    );

    Html(html)
}

#[derive(Deserialize)]
pub struct CreateProductForm {
    name: String,
    description: String,
    price: f64,
    category_id: String,
    stock: i32,
}

/// Creates a new product
pub async fn admin_create_product(
    State(store): State<Store>,
    Form(form): Form<CreateProductForm>,
) -> impl IntoResponse {
    let product = Product::new(form.name, form.description, form.price, form.category_id, form.stock);
    store.products.write().unwrap().insert(product.id.clone(), product);
    Redirect::to("/admin/products")
}

/// Deletes a product
pub async fn admin_delete_product(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    store.products.write().unwrap().remove(&id);
    Redirect::to("/admin/products")
}

/// Lists all blog categories in admin panel
pub async fn admin_list_blog_categories(State(store): State<Store>) -> impl IntoResponse {
    let blog_categories = store.blog_categories.read().unwrap();
    let mut categories_vec: Vec<_> = blog_categories.values().collect();
    categories_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut rows = String::new();
    for cat in categories_vec {
        rows.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <form method="post" action="/admin/blog-categories/delete/{}" style="display:inline;">
                        <button type="submit" class="btn btn-danger">Delete</button>
                    </form>
                </td>
            </tr>"#,
            cat.name, cat.description, cat.created_at.format("%Y-%m-%d"), cat.id
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Blog Categories - Admin</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories">Product Categories</a></li>
            <li><a href="/admin/products">Products</a></li>
            <li><a href="/admin/blog-categories" class="active">Blog Categories</a></li>
            <li><a href="/admin/posts">Blog Posts</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Blog Categories</h2>
        <form method="post" action="/admin/blog-categories/create" class="create-form">
            <input type="text" name="name" placeholder="Category Name" required>
            <input type="text" name="description" placeholder="Description" required>
            <button type="submit" class="btn">Create Category</button>
        </form>
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Description</th>
                    <th>Created</th>
                    <th>Actions</th>
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

/// Creates a new blog category
pub async fn admin_create_blog_category(
    State(store): State<Store>,
    Form(form): Form<CreateCategoryForm>,
) -> impl IntoResponse {
    let category = BlogCategory::new(form.name, form.description);
    store.blog_categories.write().unwrap().insert(category.id.clone(), category);
    Redirect::to("/admin/blog-categories")
}

/// Deletes a blog category
pub async fn admin_delete_blog_category(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    store.blog_categories.write().unwrap().remove(&id);
    Redirect::to("/admin/blog-categories")
}

/// Lists all blog posts in admin panel
pub async fn admin_list_posts(State(store): State<Store>) -> impl IntoResponse {
    let posts = store.posts.read().unwrap();
    let categories = store.blog_categories.read().unwrap();
    let mut posts_vec: Vec<_> = posts.values().collect();
    posts_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut rows = String::new();
    for post in posts_vec {
        let category_name = categories
            .get(&post.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown");
        
        let status = if post.published { "Published" } else { "Draft" };

        rows.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <form method="post" action="/admin/posts/toggle/{}" style="display:inline;">
                        <button type="submit" class="btn btn-sm">{}</button>
                    </form>
                    <form method="post" action="/admin/posts/delete/{}" style="display:inline;">
                        <button type="submit" class="btn btn-danger">Delete</button>
                    </form>
                </td>
            </tr>"#,
            post.title, category_name, status, post.created_at.format("%Y-%m-%d"), 
            post.id, if post.published { "Unpublish" } else { "Publish" },
            post.id
        ));
    }

    let mut category_options = String::new();
    for cat in categories.values() {
        category_options.push_str(&format!(r#"<option value="{}">{}</option>"#, cat.id, cat.name));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Blog Posts - Admin</title>
    <link rel="stylesheet" href="/static/css/admin.css">
</head>
<body>
    <nav class="admin-nav">
        <h1>CMS Admin Panel</h1>
        <ul>
            <li><a href="/admin">Dashboard</a></li>
            <li><a href="/admin/categories">Product Categories</a></li>
            <li><a href="/admin/products">Products</a></li>
            <li><a href="/admin/blog-categories">Blog Categories</a></li>
            <li><a href="/admin/posts" class="active">Blog Posts</a></li>
            <li><a href="/">View Site</a></li>
        </ul>
    </nav>
    <div class="container">
        <h2>Blog Posts</h2>
        <form method="post" action="/admin/posts/create" class="create-form">
            <input type="text" name="title" placeholder="Post Title" required>
            <textarea name="excerpt" placeholder="Excerpt" required></textarea>
            <textarea name="content" placeholder="Content" required rows="5"></textarea>
            <input type="text" name="author" placeholder="Author" required>
            <select name="category_id" required>
                <option value="">Select Category</option>
                {}
            </select>
            <button type="submit" class="btn">Create Post</button>
        </form>
        <table>
            <thead>
                <tr>
                    <th>Title</th>
                    <th>Category</th>
                    <th>Status</th>
                    <th>Created</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        category_options, rows
    );

    Html(html)
}

#[derive(Deserialize)]
pub struct CreatePostForm {
    title: String,
    content: String,
    excerpt: String,
    category_id: String,
    author: String,
}

/// Creates a new blog post
pub async fn admin_create_post(
    State(store): State<Store>,
    Form(form): Form<CreatePostForm>,
) -> impl IntoResponse {
    let post = Post::new(form.title, form.content, form.excerpt, form.category_id, form.author);
    store.posts.write().unwrap().insert(post.id.clone(), post);
    Redirect::to("/admin/posts")
}

/// Toggles post published status
pub async fn admin_toggle_post(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Some(post) = store.posts.write().unwrap().get_mut(&id) {
        if post.published {
            post.unpublish();
        } else {
            post.publish();
        }
    }
    Redirect::to("/admin/posts")
}

/// Deletes a blog post
pub async fn admin_delete_post(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    store.posts.write().unwrap().remove(&id);
    Redirect::to("/admin/posts")
}


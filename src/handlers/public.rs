use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use crate::store::Store;

/// Public homepage handler
/// Shows links to shop and blog sections
pub async fn public_home() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Our Store</title>
    <link rel="stylesheet" href="/static/css/public.css">
</head>
<body>
    <header>
        <nav class="main-nav">
            <div class="logo">MyStore CMS</div>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/shop">Shop</a></li>
                <li><a href="/blog">Blog</a></li>
                <li><a href="/admin">Admin</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section class="hero">
            <h1>Welcome to MyStore</h1>
            <p>Discover amazing products and read our latest articles</p>
            <div class="hero-buttons">
                <a href="/shop" class="btn btn-primary">Browse Products</a>
                <a href="/blog" class="btn btn-secondary">Read Blog</a>
            </div>
        </section>
        <section class="features">
            <div class="feature">
                <h3>Quality Products</h3>
                <p>We offer only the best products carefuly selected for you</p>
            </div>
            <div class="feature">
                <h3>Fast Shipping</h3>
                <p>Get your orders delivered quickly and securly</p>
            </div>
            <div class="feature">
                <h3>Great Content</h3>
                <p>Stay updated with our informative blog posts</p>
            </div>
        </section>
    </main>
    <footer>
        <p>&copy; 2025 MyStore CMS. All rights reserved.</p>
    </footer>
</body>
</html>"#;

    Html(html)
}

/// Shop page handler
/// Displays all available products organized by category
pub async fn public_shop(State(store): State<Store>) -> impl IntoResponse {
    let products = store.products.read().unwrap();
    let categories = store.categories.read().unwrap();
    
    let mut products_vec: Vec<_> = products.values().filter(|p| p.is_available()).collect();
    products_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut products_html = String::new();
    for product in products_vec {
        let category_name = categories
            .get(&product.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Uncategorized");

        products_html.push_str(&format!(
            r#"<div class="product-card">
                <div class="product-image"></div>
                <h3>{}</h3>
                <p class="category">{}</p>
                <p class="description">{}</p>
                <div class="product-footer">
                    <span class="price">${:.2}</span>
                    <span class="stock">{} in stock</span>
                </div>
            </div>"#,
            product.name, category_name, product.description, product.price, product.stock
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Shop - MyStore</title>
    <link rel="stylesheet" href="/static/css/public.css">
</head>
<body>
    <header>
        <nav class="main-nav">
            <div class="logo">MyStore CMS</div>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/shop" class="active">Shop</a></li>
                <li><a href="/blog">Blog</a></li>
                <li><a href="/admin">Admin</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section class="page-header">
            <h1>Our Products</h1>
            <p>Browse our collection of quality products</p>
        </section>
        <section class="products-grid">
            {}
        </section>
    </main>
    <footer>
        <p>&copy; 2025 MyStore CMS. All rights reserved.</p>
    </footer>
</body>
</html>"#,
        products_html
    );

    Html(html)
}

/// Blog listing page handler
/// Shows all published blog posts
pub async fn public_blog(State(store): State<Store>) -> impl IntoResponse {
    let posts = store.posts.read().unwrap();
    let categories = store.blog_categories.read().unwrap();
    
    let mut posts_vec: Vec<_> = posts.values().filter(|p| p.published).collect();
    posts_vec.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut posts_html = String::new();
    for post in posts_vec {
        let category_name = categories
            .get(&post.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Uncategorized");

        posts_html.push_str(&format!(
            r#"<article class="blog-card">
                <h3><a href="/blog/{}">{}</a></h3>
                <div class="post-meta">
                    <span class="category">{}</span>
                    <span class="author">by {}</span>
                    <span class="date">{}</span>
                </div>
                <p class="excerpt">{}</p>
                <a href="/blog/{}" class="read-more">Read More &rarr;</a>
            </article>"#,
            post.id, post.title, category_name, post.author, 
            post.created_at.format("%B %d, %Y"), post.excerpt, post.id
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Blog - MyStore</title>
    <link rel="stylesheet" href="/static/css/public.css">
</head>
<body>
    <header>
        <nav class="main-nav">
            <div class="logo">MyStore CMS</div>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/shop">Shop</a></li>
                <li><a href="/blog" class="active">Blog</a></li>
                <li><a href="/admin">Admin</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section class="page-header">
            <h1>Our Blog</h1>
            <p>Read our latest articles and updates</p>
        </section>
        <section class="blog-list">
            {}
        </section>
    </main>
    <footer>
        <p>&copy; 2025 MyStore CMS. All rights reserved.</p>
    </footer>
</body>
</html>"#,
        posts_html
    );

    Html(html)
}

/// Individual blog post handler
/// Displays a single blog post with full content
pub async fn public_blog_post(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let posts = store.posts.read().unwrap();
    let categories = store.blog_categories.read().unwrap();

    if let Some(post) = posts.get(&id) {
        if !post.published {
            return Html("<h1>Post not found</h1>".to_string());
        }

        let category_name = categories
            .get(&post.category_id)
            .map(|c| c.name.as_str())
            .unwrap_or("Uncategorized");

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - MyStore Blog</title>
    <link rel="stylesheet" href="/static/css/public.css">
</head>
<body>
    <header>
        <nav class="main-nav">
            <div class="logo">MyStore CMS</div>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/shop">Shop</a></li>
                <li><a href="/blog" class="active">Blog</a></li>
                <li><a href="/admin">Admin</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <article class="blog-post">
            <header class="post-header">
                <h1>{}</h1>
                <div class="post-meta">
                    <span class="category">{}</span>
                    <span class="author">by {}</span>
                    <span class="date">{}</span>
                </div>
            </header>
            <div class="post-content">
                <p>{}</p>
            </div>
            <footer class="post-footer">
                <a href="/blog" class="btn">&larr; Back to Blog</a>
            </footer>
        </article>
    </main>
    <footer>
        <p>&copy; 2025 MyStore CMS. All rights reserved.</p>
    </footer>
</body>
</html>"#,
            post.title, post.title, category_name, post.author,
            post.created_at.format("%B %d, %Y"), post.content
        );

        Html(html)
    } else {
        Html("<h1>Post not found</h1>".to_string())
    }
}


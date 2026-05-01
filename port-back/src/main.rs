mod models;
mod handlers;
mod auth;
mod database;

use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initialize database
    let db = database::init().await.expect("Failed to initialize database");

    // Build our application with routes
    let app = Router::new()
        // Public routes
        .route("/api/blogs", get(handlers::blogs::get_blogs))
        .route("/api/blogs/:slug", get(handlers::blogs::get_blog))
        .route("/api/projects", get(handlers::projects::get_projects))
        .route("/api/books", get(handlers::books::get_books))
        .route("/api/thoughts", get(handlers::thoughts::get_thoughts))
        .route("/api/about", get(handlers::about::get_about))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/signup", post(handlers::auth::signup))

        // Admin routes (protected)
        .route("/api/admin/blogs", get(handlers::blogs::get_admin_blogs))
        .route("/api/admin/blogs", post(handlers::blogs::create_blog))
        .route("/api/admin/blogs/:id", put(handlers::blogs::update_blog))
        .route("/api/admin/blogs/:id", delete(handlers::blogs::delete_blog))
        .route("/api/admin/projects", get(handlers::projects::get_admin_projects))
        .route("/api/admin/projects", post(handlers::projects::create_project))
        .route("/api/admin/projects/:id", put(handlers::projects::update_project))
        .route("/api/admin/projects/:id", delete(handlers::projects::delete_project))
        .route("/api/admin/books", get(handlers::books::get_admin_books))
        .route("/api/admin/books", post(handlers::books::create_book))
        .route("/api/admin/books/:id", put(handlers::books::update_book))
        .route("/api/admin/books/:id", delete(handlers::books::delete_book))
        .route("/api/admin/thoughts", get(handlers::thoughts::get_admin_thoughts))
        .route("/api/admin/thoughts", post(handlers::thoughts::create_thought))
        .route("/api/admin/thoughts/:id", put(handlers::thoughts::update_thought))
        .route("/api/admin/thoughts/:id", delete(handlers::thoughts::delete_thought))
        .route("/api/admin/about", get(handlers::about::get_admin_about))
        .route("/api/admin/about", put(handlers::about::update_about))
        .route("/api/auth/me", get(handlers::auth::me).layer(middleware::from_fn(auth::auth_middleware)))

        .layer(CorsLayer::permissive())
        .with_state(db);

    // Run the server
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000);
    let addr = SocketAddr::new(host.parse().expect("Invalid HOST"), port);
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

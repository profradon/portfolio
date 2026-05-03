mod models;
mod handlers;
mod auth;
mod database;

use axum::{
    http::Method,
    middleware,
    routing::{delete, get, options, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::{env, net::SocketAddr};

async fn preflight() -> axum::http::StatusCode {
    axum::http::StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("DEBUG: Starting server...");

    // Initialize database
    println!("DEBUG: Initializing database...");
    let db = database::init().await.expect("Failed to initialize database");
    println!("DEBUG: Database initialized successfully");

    // Build our application with routes
    println!("DEBUG: Building application routes...");
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
        .route("/api/admin/blogs", get(handlers::blogs::get_admin_blogs).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/blogs", post(handlers::blogs::create_blog).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/blogs/:id", put(handlers::blogs::update_blog).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/blogs/:id", delete(handlers::blogs::delete_blog).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/projects", get(handlers::projects::get_admin_projects).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/projects", post(handlers::projects::create_project).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/projects/:id", put(handlers::projects::update_project).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/projects/:id", delete(handlers::projects::delete_project).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/books", get(handlers::books::get_admin_books).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/books", post(handlers::books::create_book).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/books/:id", put(handlers::books::update_book).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/books/:id", delete(handlers::books::delete_book).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/thoughts", get(handlers::thoughts::get_admin_thoughts).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/thoughts", post(handlers::thoughts::create_thought).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/thoughts/:id", put(handlers::thoughts::update_thought).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/thoughts/:id", delete(handlers::thoughts::delete_thought).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/about", get(handlers::about::get_admin_about).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/admin/about", put(handlers::about::update_about).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/auth/me", get(handlers::auth::me).layer(middleware::from_fn(auth::auth_middleware)))
        .route("/api/*path", options(preflight))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any),
        )
        .with_state(db);

    // Run the server
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000);
    let addr = SocketAddr::new(host.parse().expect("Invalid HOST"), port);
    println!("DEBUG: Starting server on http://{}", addr);
    println!("DEBUG: JWT_SECRET is set: {}", env::var("JWT_SECRET").is_ok());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("DEBUG: Server bound to address, starting to serve...");
    axum::serve(listener, app).await.unwrap();
    println!("DEBUG: Server stopped");
}

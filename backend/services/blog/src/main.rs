mod error;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/posts", get(handlers::posts::list_posts).post(handlers::posts::create_post))
        .route(
            "/posts/:id",
            get(handlers::posts::get_post)
                .put(handlers::posts::update_post)
                .delete(handlers::posts::delete_post),
        )
        .route("/categories", get(handlers::categories::list_categories))
        .route("/tags", get(handlers::tags::list_tags))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Get the port from environment variable or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
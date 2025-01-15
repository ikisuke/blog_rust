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
                .unwrap_or_else(|_| "comment_service=debug,tower_http=debug".into()),
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
        .route(
            "/posts/:post_id/comments",
            get(handlers::comments::list_comments)
                .post(handlers::comments::create_comment),
        )
        .route(
            "/comments/:id",
            get(handlers::comments::get_comment)
                .put(handlers::comments::update_comment)
                .delete(handlers::comments::delete_comment),
        )
        .route(
            "/comments/:id/replies",
            get(handlers::comments::list_replies)
                .post(handlers::comments::create_reply),
        )
        .route(
            "/comments/:id/moderate",
            put(handlers::comments::moderate_comment),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Get the port from environment variable or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3004);

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
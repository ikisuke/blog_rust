mod error;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{get, post, put},
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
                .unwrap_or_else(|_| "user_service=debug,tower_http=debug".into()),
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
        .route("/users/:username", get(handlers::users::get_user))
        .route(
            "/users/:id/profile",
            get(handlers::profile::get_profile)
                .put(handlers::profile::update_profile),
        )
        .route(
            "/users/:username/follow",
            post(handlers::follows::follow_user)
                .delete(handlers::follows::unfollow_user),
        )
        .route(
            "/users/:username/followers",
            get(handlers::follows::get_followers),
        )
        .route(
            "/users/:username/following",
            get(handlers::follows::get_following),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Get the port from environment variable or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3003);

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
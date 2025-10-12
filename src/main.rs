mod models;
mod handlers;
mod services;
mod middleware;
mod errors;
mod database;
mod validation;

use axum::{
    Router,
    http::{Method, HeaderValue, HeaderName},
    routing::{get, post, put, patch, delete},
};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use tower_http::{trace::TraceLayer, cors::{CorsLayer, AllowOrigin}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::database::AppState;
use crate::handlers::*;
use crate::middleware::auth_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_restful_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let state = Arc::new(AppState { pool });

    // Public routes
    let public_routes = Router::new()
        .route("/api/users", post(user_handler::register))
        .route("/api/users/login", post(user_handler::login))
        .route("/ping", get(health_handler::ping));

    // Protected routes
    let protected_routes = Router::new()
        .route("/api/users/current", get(user_handler::get_current))
        .route("/api/users/current", patch(user_handler::update))
        .route("/api/users/logout", delete(user_handler::logout))
        .route("/api/contacts", post(contact_handler::create))
        .route("/api/contacts", get(contact_handler::search))
        .route("/api/contacts/:id", get(contact_handler::get))
        .route("/api/contacts/:id", put(contact_handler::update))
        .route("/api/contacts/:id", delete(contact_handler::remove))
        .route("/api/contacts/:contact_id/addresses", post(address_handler::create))
        .route("/api/contacts/:contact_id/addresses", get(address_handler::list))
        .route("/api/contacts/:contact_id/addresses/:id", get(address_handler::get))
        .route("/api/contacts/:contact_id/addresses/:id", put(address_handler::update))
        .route("/api/contacts/:contact_id/addresses/:id", delete(address_handler::remove))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // ✅ CORS configuration
    let allowed_origins = AllowOrigin::list([
        HeaderValue::from_static("http://localhost:5173")
    ]);

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);

    // Combine routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors) // ✅ CORS diaktifkan di sini!
        .with_state(state);

    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

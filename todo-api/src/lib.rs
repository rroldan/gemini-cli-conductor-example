use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub mod error;
pub mod handlers;
pub mod models;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub pool: PgPool,
}

/// Creates the Axum router for the Todo API.
///
/// # Arguments
///
/// * `state` - The application state to be shared across handlers.
///
/// # Returns
///
/// * `Router` - The configured Axum router.
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", post(handlers::create_todo))
        .with_state(state)
}

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// Custom error type for the Todo API.
#[derive(Debug, Error)]
pub enum Error {
    /// Error occurred while interacting with the database.
    #[error("database error")]
    Database(#[from] sqlx::Error),

    /// Resource was not found.
    #[error("resource not found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

use crate::{
    AppState,
    error::Error,
    models::{CreateTodo, Todo},
};
use axum::{Json, extract::State, http::StatusCode};

/// Creates a new Todo item.
///
/// # Arguments
///
/// * `state` - The application state containing the database pool.
/// * `payload` - The JSON payload containing the Todo details.
///
/// # Returns
///
/// * `Result<(StatusCode, Json<Todo>), Error>` - The created Todo or an error.
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), Error> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, completed
        "#,
        payload.title,
        payload.description
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

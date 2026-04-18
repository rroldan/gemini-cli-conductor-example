use serde::{Deserialize, Serialize};

/// Represents a Todo item in the system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    /// Unique identifier for the Todo.
    pub id: i32,
    /// Title of the Todo.
    pub title: String,
    /// Optional detailed description.
    pub description: Option<String>,
    /// Whether the Todo is completed.
    pub completed: Option<bool>,
}

/// Data required to create a new Todo item.
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    /// Title of the Todo.
    pub title: String,
    /// Optional detailed description.
    pub description: Option<String>,
}

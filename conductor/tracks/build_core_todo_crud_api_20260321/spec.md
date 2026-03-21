# Specification: Build Core Todo CRUD API

## 1. Overview
This track focuses on implementing the core CRUD (Create, Read, Update, Delete) functionality for the Todo API. The goal is to establish a solid foundation using Rust, Axum, and SQLx, ensuring high performance, type safety, and reliability.

## 2. Goals
-   Initialize a new Rust project with Axum and SQLx.
-   Set up a PostgreSQL database schema for storing Todo items.
-   Implement RESTful endpoints for managing Todos.
-   Ensure comprehensive test coverage for all operations.

## 3. User Stories
-   **As a user**, I want to create a new todo note so that I can remember tasks.
-   **As a user**, I want to view all my todo notes to see what I need to do.
-   **As a user**, I want to view a specific todo note to see its details.
-   **As a user**, I want to update a todo note (e.g., mark as completed) to keep my list current.
-   **As a user**, I want to delete a todo note when it's no longer needed.

## 4. API Definition
| Method | Endpoint | Description | Request Body | Response |
| :--- | :--- | :--- | :--- | :--- |
| `GET` | `/todos` | List all todos | N/A | `200 OK` (JSON List) |
| `GET` | `/todos/:id` | Get a specific todo | N/A | `200 OK` (JSON) or `404 Not Found` |
| `POST` | `/todos` | Create a new todo | `{"note": "string"}` | `201 Created` (JSON) |
| `PUT` | `/todos/:id` | Update a todo | `{"note": "string", "completed": boolean}` | `200 OK` (JSON) or `404 Not Found` |
| `DELETE` | `/todos/:id` | Delete a todo | N/A | `204 No Content` or `404 Not Found` |

## 5. Database Schema
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

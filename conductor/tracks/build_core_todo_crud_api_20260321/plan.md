# Implementation Plan - Build Core Todo CRUD API

This plan outlines the steps to build the core Todo CRUD API using Rust, Axum, and SQLx.

## Phase 1: Project Initialization & Database Setup

- [x] Task: Initialize Rust Project (1109197) [a021cfd]
    - [x] Create new Cargo project with `cargo new todo-api`
    - [x] Add dependencies: `axum`, `tokio`, `serde`, `serde_json`, `sqlx`, `dotenvy`, `tracing`, `tracing-subscriber`
    - [x] Create basic `main.rs` that starts an Axum server
    - [x] Verification: Run `cargo run` and ensure server starts
    - [x] Ask user to prepare pull request

- [x] Task: Configure Database & Migrations [f3e808a]
    - [x] Set up `docker-compose.yml` for PostgreSQL
    - [x] Create `.env` file with `DATABASE_URL`
    - [x] **Launch `docker-compose`**
    - [x] Initialize SQLx: `sqlx database create`
    - [x] Create migration: `sqlx migrate add create_todos_table`
    - [x] Write SQL for `todos` table in migration file
    - [x] Run migration: `sqlx migrate run`
    - [x] Verification: Connect to DB and verify table exists
    - [x] Ask user to prepare pull request

- [ ] Task: Conductor - User Manual Verification 'Phase 1: Project Initialization & Database Setup' (Protocol in workflow.md)

## Phase 2: Core CRUD Implementation

- [ ] Task: Implement 'Create Todo' (POST /todos)
    - [ ] Write failing test: Create a test that POSTs a valid todo and expects 201 Created
    - [ ] Implement `Todo` struct and `CreateTodo` payload struct
    - [ ] Implement `create_todo` handler
    - [ ] Register route `POST /todos`
    - [ ] Pass tests (Green Phase)
    - [ ] Refactor
    - [ ] Ask user to prepare pull request

- [ ] Task: Implement 'List Todos' (GET /todos)
    - [ ] Write failing test: Create a test that GETs /todos and expects a list
    - [ ] Implement `list_todos` handler
    - [ ] Register route `GET /todos`
    - [ ] Pass tests (Green Phase)
    - [ ] Refactor
    - [ ] Ask user to prepare pull request

- [ ] Task: Implement 'Get Todo' (GET /todos/:id)
    - [ ] Write failing test: Create a test that GETs a specific ID and expects the correct todo
    - [ ] Implement `get_todo` handler
    - [ ] Register route `GET /todos/:id`
    - [ ] Pass tests (Green Phase)
    - [ ] Refactor
    - [ ] Ask user to prepare pull request

- [ ] Task: Implement 'Update Todo' (PUT /todos/:id)
    - [ ] Write failing test: Create a test that updates a todo and verifies the change
    - [ ] Implement `UpdateTodo` payload struct
    - [ ] Implement `update_todo` handler
    - [ ] Register route `PUT /todos/:id`
    - [ ] Pass tests (Green Phase)
    - [ ] Refactor
    - [ ] Ask user to prepare pull request

- [ ] Task: Implement 'Delete Todo' (DELETE /todos/:id)
    - [ ] Write failing test: Create a test that deletes a todo and verifies it's gone
    - [ ] Implement `delete_todo` handler
    - [ ] Register route `DELETE /todos/:id`
    - [ ] Pass tests (Green Phase)
    - [ ] Refactor
    - [ ] Ask user to prepare pull request

- [ ] Task: Conductor - User Manual Verification 'Phase 2: Core CRUD Implementation' (Protocol in workflow.md)

## Phase 3: Final Verification & Polish

- [ ] Task: Comprehensive API Testing
    - [ ] Run full integration test suite
    - [ ] Verify error handling for invalid inputs
    - [ ] Verify error handling for non-existent IDs
    - [ ] Check code coverage
    - [ ] Ask user to prepare pull request

- [ ] Task: Documentation Update
    - [ ] Add comments to public functions
    - [ ] Update README.md with run instructions
    - [ ] Ask user to prepare pull request

- [ ] Task: Conductor - User Manual Verification 'Phase 3: Final Verification & Polish' (Protocol in workflow.md)

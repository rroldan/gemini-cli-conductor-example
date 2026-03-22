# Initial Concept

#TODO API

Simple todo API using Rust Axum and SQLx with PostgresQL

The following routes are provided:
GET /todos - List all to-do items.
GET /todos/<id> - Get a to-do item by ID.
POST /todos - Create a to-do item. Takes “note” as a JSON body parameter.
PUT /todos/<id> - Update a to-do item. Takes “note” and “completed” as JSON body parameters.
DELETE /todos/<id> - Delete a to-do item by ID.

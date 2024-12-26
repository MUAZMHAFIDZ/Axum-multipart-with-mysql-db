use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::todo::Todo;
use crate::AppState;

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Todo>,
) -> Result<Json<Todo>, StatusCode> {
    let id = Uuid::new_v4();
    let query = "INSERT INTO todos (id, title, description) VALUES (?, ?, ?)";

    sqlx::query(query)
        .bind(id.to_string())
        .bind(&payload.title)
        .bind(&payload.description)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Todo { id: id, ..payload }))
}

pub async fn get_todos(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let query = "SELECT id, title, description FROM todos";
    let todos = sqlx::query_as::<_, Todo>(query)
        .fetch_all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let query = "DELETE FROM todos WHERE id = ?";

    sqlx::query(query)
        .bind(id.to_string())
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Todo>,
) -> Result<Json<Todo>, StatusCode> {
    let query = "UPDATE todos SET title = ?, description = ? WHERE id = ?";

    sqlx::query(query)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(id.to_string())
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated_todo = Todo {
        id,
        title: payload.title.clone(),
        description: payload.description.clone(),
    };

    Ok(Json(updated_todo))
}

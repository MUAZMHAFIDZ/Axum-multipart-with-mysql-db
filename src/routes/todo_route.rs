use crate::handlers::todo::{create_todo, delete_todo, get_todos, update_todo};
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn todo_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_todo).get(get_todos))
        .route("/:id", delete(delete_todo).put(update_todo))
        .with_state(app_state.clone())
}

use std::sync::Arc;

use axum::{
    routing::{delete, get, get_service, post, put},
    Router,
};
use tower_http::services::ServeDir;

use crate::{
    handlers::todo::{create_todo, delete_todo, get_todos, update_todo},
    handlers::upload::{delete_upload, get_upload, upload_file},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/public",
            Router::new().route("/*file_path", get_service(ServeDir::new("public"))),
        )
        .route("/api/todos/", post(create_todo))
        .route("/api/todos/", get(get_todos))
        .route("/api/todos/:id", delete(delete_todo))
        .route("/api/todos/:id", put(update_todo))
        .route("/api/upload/", post(upload_file))
        .route("/api/upload/", get(get_todos))
        .route("/api/upload/:id", delete(delete_todo))
        .with_state(app_state)
}

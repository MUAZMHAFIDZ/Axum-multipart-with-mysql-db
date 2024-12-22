use std::sync::Arc;

use axum::{routing::{get, get_service, post}, Router
};
use tower_http::services::ServeDir; 

use crate::{
    controllers::{
        create_todo, get_todos, upload_file
    },
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
        .route("/api/upload/", post(upload_file))
        .with_state(app_state)
}
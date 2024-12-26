pub mod todo_route;
pub mod upload_route;

use std::sync::Arc;

use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/public",
            Router::new().route("/*file_path", get_service(ServeDir::new("public"))),
        )
        .nest("/api/todos", todo_route::todo_routes(app_state.clone()))
        .nest(
            "/api/upload",
            upload_route::upload_routes(app_state.clone()),
        )
}

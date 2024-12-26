use crate::handlers::upload::{delete_upload, get_upload, upload_file};
use crate::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

pub fn upload_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(upload_file).get(get_upload))
        .route("/:id", delete(delete_upload))
        .with_state(app_state.clone())
}

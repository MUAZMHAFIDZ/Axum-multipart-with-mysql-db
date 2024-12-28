use crate::handlers::auth::{login, logout, register};
use crate::AppState;
use axum::routing::get;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .with_state(app_state.clone())
}

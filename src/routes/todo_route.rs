use crate::handlers::todo::{create_todo, delete_todo, get_todos, update_todo};
use crate::middlewares::protect::protect;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::{
    middleware::{from_fn, Next},
    routing::{delete, get, post, put},
    Router,
};
use cookie::CookieJar;
use std::sync::Arc;

pub fn todo_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_todo).get(get_todos))
        .route("/:id", delete(delete_todo).put(update_todo))
        .layer(from_fn(move |req: Request<Body>, next| {
            // Apply the protect middleware
            async move { protect(req, next).await }
        }))
        .with_state(app_state.clone())
}

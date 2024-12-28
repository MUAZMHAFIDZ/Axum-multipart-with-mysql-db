use crate::models::user::{AuthResponse, LoginPayload, User};
use crate::AppState;
use axum::http::{header, StatusCode};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::lib::jwt::{clear_jwt_cookie, create_jwt, set_jwt_cookie};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<User>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let id = Uuid::new_v4();

    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = "INSERT INTO users (id, username, password, fullname) VALUES (?, ?, ?, ?)";
    sqlx::query(query)
        .bind(id.to_string())
        .bind(&payload.username)
        .bind(&hashed_password)
        .bind(&payload.fullname)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        message: "User registered successfully".to_string(),
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let query = "SELECT id, password FROM users WHERE username = ?";
    let user_data: Result<(String, String), Error> = sqlx::query_as(query)
        .bind(&payload.username)
        .fetch_one(&*state.db)
        .await;

    let (user_id, stored_password) = match user_data {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let is_valid = verify(&payload.password, &stored_password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if is_valid {
        let token = create_jwt(user_id.clone()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut headers = HeaderMap::new();
        headers.insert("Set-Cookie", set_jwt_cookie(token).parse().unwrap());

        return Ok((
            headers,
            Json(AuthResponse {
                message: "Login successful".to_string(),
            }),
        ));
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn logout() -> impl IntoResponse {
    let cookie = clear_jwt_cookie();

    (
        [(header::SET_COOKIE, cookie)],
        Json(AuthResponse {
            message: "Logout successful".to_string(),
        }),
    )
}

// pub async fn protected_route(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
// ) -> Result<Json<AuthResponse>, StatusCode> {
//     let token = match headers.get("Authorization") {
//         Some(value) => value.to_str().unwrap_or("").replace("Bearer ", ""),
//         None => return Err(StatusCode::UNAUTHORIZED),
//     };

//     let claims = verify_jwt(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;

//     Ok(Json(AuthResponse {
//         message: format!("Welcome, user {}!", claims.sub),
//     }))
// }

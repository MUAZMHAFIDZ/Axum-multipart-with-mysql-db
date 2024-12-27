use crate::lib::jwt::{get_jwt_cookie, verify_jwt, Claims};
use axum::body::Body;
use axum::http::StatusCode;
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn protect(mut req: Request<Body>, next: Next) -> Result<Response, Response> {
    let token = get_jwt_cookie(&req);
    match &token {
        Some(t) => println!("JWT Token: {}", t),
        None => println!("No token found in the cookie"),
    }
    if token.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized: Missing token").into_response());
    }
    let token = token.unwrap();
    let claims: Claims = match verify_jwt(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((StatusCode::UNAUTHORIZED, "Unauthorized: Invalid token").into_response())
        }
    };
    let user_id = claims.sub;
    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}

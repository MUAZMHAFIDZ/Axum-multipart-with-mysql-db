use axum::{body::Body, http::Request};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &str = "your-secret-key";

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

pub fn set_jwt_cookie(token: String) -> String {
    format!("jwt={}; HttpOnly; Secure; Path=/; Max-Age=3600", token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_ref());
    let validation = Validation::default();
    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}

pub fn get_jwt_cookie(req: &Request<Body>) -> Option<String> {
    if let Some(cookie_header) = req.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let trimmed = cookie.trim();
                if trimmed.starts_with("jwt=") {
                    return Some(trimmed[4..].to_string());
                }
            }
        }
    }
    None
}

pub fn clear_jwt_cookie() -> String {
    "jwt=; HttpOnly; Secure; Path=/; Max-Age=0".to_string()
}

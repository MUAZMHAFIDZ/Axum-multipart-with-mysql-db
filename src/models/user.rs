use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct User {
    #[serde(default = "generate_uuid")]
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

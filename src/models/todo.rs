use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Todo {
    #[serde(default = "generate_uuid")]
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub description: String,
}

fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

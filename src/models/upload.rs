use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Upload {
    pub id: Uuid,
    pub title: String,
    pub file_name: String,
}

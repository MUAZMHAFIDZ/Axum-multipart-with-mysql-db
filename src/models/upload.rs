use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Upload {
    pub id: String,
    pub title: String,
    pub file_name: String,
}

use axum::{
    extract::{multipart::Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::lib::upload::save_file;
use crate::AppState;

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    let mut title = String::from("Uploaded File");

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("title") {
            title = field
                .text()
                .await
                .unwrap_or_else(|_| String::from("Uploaded File"));
            break;
        }
    }

    let file_location = String::from("public/image");

    match save_file(multipart, file_location).await {
        Ok(file_name) => {
            let unique_id = Uuid::new_v4();

            let query = "INSERT INTO uploads (id, title, file_name) VALUES (?, ?, ?)";
            match sqlx::query(query)
                .bind(&unique_id.to_string())
                .bind(&title)
                .bind(&file_name)
                .execute(&state.db)
                .await
            {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!("File uploaded successfully: {}", file_name),
                )),
                Err(err) => {
                    println!("Database error: {}", err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(err) => {
            println!("File save error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

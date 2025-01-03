use axum::{
    extract::{multipart::Multipart, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::lib::upload::{delete_file, save_file};
use crate::models::upload::Upload;
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
                .execute(&*state.db)
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

pub async fn get_upload(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Upload>>, StatusCode> {
    let query = "SELECT id, title, file_name FROM uploads";
    let uploads = sqlx::query_as::<_, Upload>(query)
        .fetch_all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(uploads))
}

pub async fn delete_upload(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let query_select = "SELECT file_name FROM uploads WHERE id = ?";
    let file_name: String = sqlx::query_scalar(query_select)
        .bind(id.to_string())
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_path = format!("public/image/{}", file_name);
    delete_file(&file_path).await?;
    let query_delete = "DELETE FROM uploads WHERE id = ?";
    sqlx::query(query_delete)
        .bind(id.to_string())
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

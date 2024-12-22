use axum::{
    extract::{Json, multipart::Multipart, State},
    http::StatusCode, response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState; 
use crate::models::Todo;  
use crate::upload::save_file;  

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Todo>,
) -> Result<Json<Todo>, StatusCode> {
    let id = Uuid::new_v4(); 
    let query = "INSERT INTO todos (id, title, description) VALUES (?, ?, ?)";
    
    sqlx::query(query)
        .bind(id.to_string()) 
        .bind(&payload.title)
        .bind(&payload.description)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Todo { id: id, ..payload }))
}


pub async fn get_todos(State(state): State<Arc<AppState>>,) -> Result<Json<Vec<Todo>>, StatusCode> {
    let query = "SELECT id, title, description FROM todos";
    let todos = sqlx::query_as::<_, Todo>(query)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart, 
) -> Result<impl IntoResponse, StatusCode> {
    let mut title = String::from("Uploaded File"); 

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("title") {
            title = field.text().await.unwrap_or_else(|_| String::from("Uploaded File"));
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
                Ok(_) => Ok((StatusCode::OK, format!("File uploaded successfully: {}", file_name))),
                Err(err) => {
                    println!("Database error: {}", err);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(err) => {
            println!("File save error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
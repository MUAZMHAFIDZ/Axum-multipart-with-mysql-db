use axum::{extract::multipart::Multipart, http::StatusCode};
use tokio::fs::File;
use tokio::io::AsyncWriteExt; 

// =============================== This function without println ===============================
// pub async fn save_file(mut multipart: Multipart) -> Result<String, StatusCode> {
//     let mut file_name = None;

//     let upload_dir = std::path::Path::new("public/image");

//     while let Some(mut field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {

//         if let Some(name) = field.name() {
//             if name == "file" {
//                 let file = field.file_name().unwrap_or("uploaded_file");
//                 file_name = Some(file.to_string());
//                 let mut file_content = Vec::new();

//                 while let Some(chunk) = field.chunk().await.map_err(|_| StatusCode::BAD_REQUEST)? {
//                     file_content.extend_from_slice(&chunk);
//                 }

//                 let mut file_path = upload_dir.join(file_name.clone().unwrap());
//                 let mut f = File::create(file_path)
//                     .await
//                     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//                 f.write_all(&file_content)
//                     .await
//                     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//             }
//         }
//     }

//     file_name.ok_or(StatusCode::BAD_REQUEST)
// }

// ======================== using original file name ==============================
// pub async fn save_file(mut multipart: Multipart) -> Result<String, StatusCode> {
//     let mut file_name = None;

//     let upload_dir = std::path::Path::new("public/image");

//     println!("Upload directory: {:?}", upload_dir);

//     while let Some(mut field) = multipart.next_field().await.map_err(|_| {
//         println!("Error reading multipart field");
//         StatusCode::BAD_REQUEST
//     })? {

//         if let Some(name) = field.name() {
//             println!("Field name: {}", name);  
//             if name == "file" {
//                 let file = field.file_name().unwrap_or("uploaded_file");
//                 file_name = Some(file.to_string());

//                 println!("File name extracted: {}", file);  

//                 let mut file_content = Vec::new();

//                 while let Some(chunk) = field.chunk().await.map_err(|_| {
//                     println!("Error reading chunk");
//                     StatusCode::BAD_REQUEST
//                 })? {
//                     file_content.extend_from_slice(&chunk);
//                 }

//                 println!("File content length: {}", file_content.len());

//                 let mut file_path = upload_dir.join(file_name.clone().unwrap());
//                 println!("Saving file to: {:?}", file_path); 

//                 let mut f = File::create(file_path)
//                     .await
//                     .map_err(|_| {
//                         println!("Error creating file");
//                         StatusCode::INTERNAL_SERVER_ERROR
//                     })?;

//                 f.write_all(&file_content)
//                     .await
//                     .map_err(|_| {
//                         println!("Error writing file content");
//                         StatusCode::INTERNAL_SERVER_ERROR
//                     })?;
//             }
//         }
//     }

//     match &file_name {
//         Some(name) => println!("File saved as: {}", name),
//         None => println!("No file saved"),
//     }

//     file_name.ok_or(StatusCode::BAD_REQUEST)
// }

use uuid::Uuid;


// ============================== using UUID as file name =====================================
pub async fn save_file(mut multipart: Multipart, locate: String) -> Result<String, StatusCode> {
    let mut file_name = None;

    let upload_dir = std::path::Path::new(&locate);
    println!("Upload directory: {:?}", upload_dir);

    while let Some(mut field) = multipart.next_field().await.map_err(|_| {
        println!("Error reading multipart field");
        StatusCode::BAD_REQUEST
    })? {
        if let Some(name) = field.name() {
            println!("Field name: {}", name);
            if name == "file" {
                let original_file_name = field.file_name().unwrap_or("uploaded_file");
                let file_extension = std::path::Path::new(original_file_name)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or(""); 
                let new_file_name = format!("{}.{}", Uuid::new_v4(), file_extension);

                println!("Original file name: {}", original_file_name);
                println!("New file name: {}", new_file_name);

                file_name = Some(new_file_name.clone());

                let mut file_content = Vec::new();
                while let Some(chunk) = field.chunk().await.map_err(|_| {
                    println!("Error reading chunk");
                    StatusCode::BAD_REQUEST
                })? {
                    file_content.extend_from_slice(&chunk);
                }

                println!("File content length: {}", file_content.len());

                let file_path = upload_dir.join(&new_file_name);
                println!("Saving file to: {:?}", file_path);

                let mut f = File::create(file_path)
                    .await
                    .map_err(|_| {
                        println!("Error creating file");
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;

                f.write_all(&file_content)
                    .await
                    .map_err(|_| {
                        println!("Error writing file content");
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
            }
        }
    }

    match &file_name {
        Some(name) => println!("File saved as: {}", name),
        None => println!("No file saved"),
    }

    file_name.ok_or(StatusCode::BAD_REQUEST)
}

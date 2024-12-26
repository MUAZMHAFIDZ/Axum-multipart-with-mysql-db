mod handlers {
    pub mod todo;
    pub mod upload;
}
mod models {
    pub mod todo;
}
mod routes;
mod lib {
    pub mod upload;
}

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use routes::create_router;
use tower_http::cors::CorsLayer;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    db: MySqlPool,
}

async fn setup_database(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let create_todos_table_query = r#"
        CREATE TABLE IF NOT EXISTS todos (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT NOT NULL
        )
    "#;

    sqlx::query(create_todos_table_query).execute(pool).await?;
    println!("Table 'todos' has been created successfully!");

    let create_uploads_table_query = r#"
        CREATE TABLE IF NOT EXISTS uploads (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            file_name VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;

    sqlx::query(create_uploads_table_query)
        .execute(pool)
        .await?;
    println!("Table 'uploads' has been created successfully!");

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL nothing!!!");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the DB Mwehehehe!");
            pool
        }
        Err(err) => {
            println!("Error connect to the DB TvT : {:?}", err);
            std::process::exit(1);
        }
    };

    setup_database(&pool).await;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("Server started Captain!!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

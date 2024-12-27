mod handlers {
    pub mod auth;
    pub mod todo;
    pub mod upload;
}
mod models {
    pub mod todo;
    pub mod upload;
    pub mod user;
}
mod routes;
mod lib {
    pub mod jwt;
    pub mod upload;
}
mod db;
mod state;
mod middlewares {
    pub mod protect;
}

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use routes::create_router;
use state::AppState;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Setup database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing!");
    let pool = match db::create_pool(&database_url).await {
        Ok(pool) => {
            println!("Connected to the database!");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    db::setup_database(&pool).await.unwrap();

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Create app state and router
    let app_state = AppState { db: Arc::new(pool) };
    let app = create_router(Arc::new(app_state)).layer(cors);

    // Start the server
    println!("Server is running on http://0.0.0.0:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

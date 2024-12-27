use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn setup_database(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let create_todos_table_query = r#"
        CREATE TABLE IF NOT EXISTS todos (
            id CHAR(36) PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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

    let create_users_table_query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id CHAR(36) PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL,
            fullname VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;

    sqlx::query(create_users_table_query).execute(pool).await?;
    println!("Table 'users' has been created successfully!");

    Ok(())
}

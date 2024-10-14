use sqlx::{Pool, Sqlite, SqlitePool};
use std::fs;
use std::path::Path;

/// Create or connect to the SQLite database
pub async fn create_pool() -> Pool<Sqlite> {
    // Ensure the 'data/' directory exists
    let db_path = "data/app.db";
    if let Some(parent) = Path::new(db_path).parent() {
        fs::create_dir_all(parent).expect("Failed to create 'data' directory");
    }

    // Connect to the database
    SqlitePool::connect(&format!("sqlite://{}", db_path))
        .await
        .expect("Failed to connect to SQLite database")
}

/// Create the `users` table if it doesn't exist
pub async fn initialize_db(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );"
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");
}

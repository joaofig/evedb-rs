use std::path::Path;
use anyhow::{Context, Result};
use sqlx::{SqlitePool, Sqlite, Pool, Error};
use tokio::fs;

pub struct SqliteDb {
    path: String,
}

impl SqliteDb {
    pub fn new(path: &str) -> Self {
        Self {
            path: format!("sqlite://{}", path.to_string()),
        }
    }

    pub async fn connect(&self) -> std::result::Result<Pool<Sqlite>, Error> {
        self.ensure_database().await.expect("Failed to create database file");
        SqlitePool::connect(&self.path).await
    }

    async fn ensure_database(&self) -> Result<()> {
        // Extract the path from the SQLite URL (format: "sqlite:///path/to/db.sqlite")
        let path_str = self.path
            .strip_prefix("sqlite://")
            .context("Invalid SQLite URL (expected 'sqlite:///path/to/db.sqlite')")?;
        let db_path = Path::new(path_str);

        // Create the file if it doesn't exist (no-op if it does)
        if !db_path.exists() {
            // Create parent directories if they don't exist
            if let Some(parent_dir) = db_path.parent() {
                fs::create_dir_all(parent_dir)
                    .await
                    .with_context(|| format!("Failed to create parent directories for {:?}", db_path))?;
            }

            fs::File::create(db_path)
                .await
                .with_context(|| format!("Failed to create database file at {:?}", db_path))?;
            println!("Created new database file: {:?}", db_path);
        }
        Ok(())
    }
}

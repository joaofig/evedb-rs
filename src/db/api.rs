use rusqlite::{Connection, Error};

pub struct SqliteDb {
    path: String,
}

impl SqliteDb {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn connect(&self) -> Result<Connection, Error> {
        Connection::open(&self.path)
    }

    // async fn ensure_database(&self) -> Result<()> {
    //     // Extract the path from the SQLite URL (format: "sqlite:///path/to/db.sqlite")
    //     let path_str = self
    //         .path
    //         .strip_prefix("sqlite://")
    //         .context("Invalid SQLite URL (expected 'sqlite:///path/to/db.sqlite')")?;
    //     let db_path = Path::new(path_str);
    // 
    //     // Create the file if it doesn't exist (no-op if it does)
    //     if !db_path.exists() {
    //         // Create parent directories if they don't exist
    //         if let Some(parent_dir) = db_path.parent() {
    //             fs::create_dir_all(parent_dir).await.with_context(|| {
    //                 format!("Failed to create parent directories for {:?}", db_path)
    //             })?;
    //         }
    // 
    //         fs::File::create(db_path)
    //             .await
    //             .with_context(|| format!("Failed to create database file at {:?}", db_path))?;
    //         println!("Created new database file: {:?}", db_path);
    //     }
    //     Ok(())
    // }
}

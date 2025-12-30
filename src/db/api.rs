use std::result::Result;
use sqlx::{SqlitePool, Sqlite, Pool, Error};

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
        SqlitePool::connect(&self.path).await
    }
}

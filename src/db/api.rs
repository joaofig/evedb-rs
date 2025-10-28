use rusqlite::{Connection, Result};

pub struct SqliteDb {
    path: String,
}

impl SqliteDb {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn connect(&self) -> Result<Connection> {
        Connection::open(self.path.clone())
    }
}

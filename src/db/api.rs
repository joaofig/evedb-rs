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
}

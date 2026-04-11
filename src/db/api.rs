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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sqlitedb_new() {
        let db = SqliteDb::new("test.db");
        assert_eq!(db.path, "test.db");
    }

    #[test]
    fn test_sqlitedb_connect() {
        let db_path = "test_api.db";
        let db = SqliteDb::new(db_path);
        let conn = db.connect();
        assert!(conn.is_ok());
        fs::remove_file(db_path).unwrap();
    }
}

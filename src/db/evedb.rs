use crate::db::api::SqliteDb;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    fn connect(&self) -> rusqlite::Result<rusqlite::Connection> {
        self.db.connect()
    }

    fn create_vehicle_table(&self) {
        let sql = "
        CREATE TABLE IF NOT EXISTS main.vehicle (
            vehicle_id    INTEGER primary key,
            vehicle_type  TEXT,
            vehicle_class TEXT,
            engine        TEXT,
            transmission  TEXT,
            drive_wheels  TEXT,
            weight        INTEGER
        ) STRICT";

        match self.connect() {
            Ok(conn) => {
                conn.execute(sql, []).unwrap();
            }
            Err(e) => {
                println!("Error creating table: {}", e);
            }
        }
    }

    pub fn create_tables(&self) {
        self.create_vehicle_table();
    }
}
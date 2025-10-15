use crate::db::api::SqliteDb;
use crate::models::vehicle::Vehicle;
use rusqlite::Connection;
use rusqlite::Result;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    fn connect(&self) -> Result<Connection> {
        self.db.connect()
    }

    pub fn create_vehicle_table(&self) -> Result<usize> {
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

        let conn = self.connect()?;
        conn.execute(sql, [])
    }

    pub fn insert_vehicles(&self, vehicles: Vec<Vehicle>) -> Result<()> {
        let sql = "
        INSERT INTO main.vehicle (
            vehicle_id,
            vehicle_type,
            vehicle_class,
            engine,
            transmission,
            drive_wheels,
            weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let mut conn = self.connect()?;
        let transaction = conn.transaction()?;

        for vehicle in vehicles {
            transaction.execute(sql, vehicle.to_tuple())?;
        }
        transaction.commit()
    }

    pub fn create_tables(&self) {
        self.create_vehicle_table().unwrap_or(0);
    }
}

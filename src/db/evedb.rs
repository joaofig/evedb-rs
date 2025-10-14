use crate::db::api::SqliteDb;
use crate::models::vehicle::Vehicle;

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

    pub fn create_vehicle_table(&self) {
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

    pub fn insert_vehicles(&self, vehicles: Vec<Vehicle>) -> bool {
        let sql = "
        INSERT INTO main.vehicle (
            vehicle_id,
            vehicle_type,
            vehicle_class,
            engine,
            transmission,
            drive_wheels,
            weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        match self.connect() {
            Ok(mut conn) => {
                match conn.transaction() {
                    Ok(transaction) => {
                        for vehicle in vehicles {
                            transaction.execute(sql, vehicle.to_tuple()).unwrap();
                        }
                        transaction.commit().unwrap_or(())
                    }
                    Err(e) => {
                        println!("Error starting transaction: {}", e);
                        return false;
                    }
                }
            }
            Err(e) => {
                println!("Error inserting vehicle data: {}", e);
                return false;
            }
        }
        true
    }

    pub fn create_tables(&self) {
        self.create_vehicle_table();
    }
}

use crate::db::api::SqliteDb;
use crate::models::vehicle::Vehicle;
use crate::models::signal::CsvSignal;
use rusqlite::Connection;
use rusqlite::Result;
use text_block_macros::text_block;

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
        let conn = self.connect()?;
        if conn.table_exists(None, "main.vehicle")? {
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
            conn.execute(sql, [])
        } else {
            Ok(0)
        }
    }

    pub fn create_signal_table(&self) -> Result<usize> {
        let conn = self.connect()?;

        if conn.table_exists(None, "main.vehicle")? {
            let sql = text_block! {
            "create table if not exists main.signal ("
            "   signal_id          INTEGER primary key,"
            "   day_num            DOUBLE  not null,"
            "   vehicle_id         INTEGER not null,"
            "   trip_id            INTEGER not null,"
            "   time_stamp         INTEGER not null,"
            "   latitude           DOUBLE  not null,"
            "   longitude          DOUBLE  not null,"
            "   speed              DOUBLE,"
            "   maf                DOUBLE,"
            "   rpm                DOUBLE,"
            "   abs_load           DOUBLE,"
            "   oat                DOUBLE,"
            "   fuel_rate          DOUBLE,"
            "   ac_power_kw        DOUBLE,"
            "   ac_power_w         DOUBLE,"
            "   heater_power_w     DOUBLE,"
            "   hv_bat_current     DOUBLE,"
            "   hv_bat_soc         DOUBLE,"
            "   hv_bat_volt        DOUBLE,"
            "   st_ftb_1           DOUBLE,"
            "   st_ftb_2           DOUBLE,"
            "   lt_ftb_1           DOUBLE,"
            "   lt_ftb_2           DOUBLE,"
            "   elevation          DOUBLE,"
            "   elevation_smooth   DOUBLE,"
            "   gradient           DOUBLE,"
            "   energy_consumption DOUBLE,"
            "   match_latitude     DOUBLE  not null,"
            "   match_longitude    DOUBLE  not null,"
            "   match_type         INTEGER not null,"
            "   speed_limit_type   INTEGER,"
            "   speed_limit        TEXT,"
            "   speed_limit_direct INTEGER,"
            "   intersection       INTEGER,"
            "   bus_stop           INTEGER,"
            "   focus_points       TEXT,"
            "   h3_12              INTEGER);"};
            conn.execute(sql, [])
        } else {
            Ok(0)
        }
    }
    
    pub fn insert_signal(&self, signal: &CsvSignal) -> Result<()> {
        let sql = text_block! {
            "INSERT INTO main.signal ("
            "   day_num, vehicle_id, trip_id, time_stamp, latitude, "
            "   longitude, speed, maf, rpm, abs_load, oat, fuel_rate, "
            "   ac_power_kw, ac_power_w, heater_power_w, hv_bat_current, "
            "   hv_bat_soc, hv_bat_volt, st_ftb_1, st_ftb_2, lt_ftb_1, "
            "   lt_ftb_2, elevation, elevation_smooth, gradient, "
            "   energy_consumption, match_latitude, match_longitude, "
            "   match_type, speed_limit_type, speed_limit, speed_limit_direct, "
            "   intersection, bus_stop, focus_points, h3_12) "
            "VALUES "
            "(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18,"
            " ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34);"
        };
        Ok(())
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

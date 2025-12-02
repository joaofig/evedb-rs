use crate::db::api::SqliteDb;
use crate::models::vehicle::Vehicle;
use crate::models::signal::CsvSignal;
use h3o::{LatLng, Resolution};
use indicatif::ProgressIterator;
use rusqlite::{params, Connection, Transaction};
use rusqlite::Result;
use text_block_macros::text_block;
use crate::models::trajectory::TrajectoryPoint;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    pub fn connect(&self) -> Result<Connection> {
        self.db.connect()
    }

    pub fn create_vehicle_table(&self) -> Result<usize> {
        let conn = self.connect()?;
        if !conn.table_exists(None, "main.vehicle")? {
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

        if !conn.table_exists(None, "main.vehicle")? {
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
    
    pub fn insert_signal(&self,
                         transaction: &Transaction,
                         signal: &CsvSignal) -> anyhow::Result<()> {
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
            " ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34,"
            " ?35, ?36);"
        };

        let coord = LatLng::new(signal.match_latitude, signal.match_longitude)
            .expect("valid coord");
        let cell = coord.to_cell(Resolution::Twelve);
        let index: u64 = cell.into();

        transaction.execute(sql,
                            params![
                                signal.day_num as i64,
                                signal.vehicle_id as i64,
                                signal.trip_id as i64,
                                signal.time_stamp as i64,
                                signal.latitude,
                                signal.longitude,
                                signal.speed,
                                signal.maf,
                                signal.rpm,
                                signal.abs_load,
                                signal.oat,
                                signal.fuel_rate,
                                signal.ac_power_kw,
                                signal.ac_power_w,
                                signal.heater_power_w,
                                signal.hv_bat_current,
                                signal.hv_bat_soc,
                                signal.hv_bat_volt,
                                signal.st_ftb_1,
                                signal.st_ftb_2,
                                signal.lt_ftb_1,
                                signal.lt_ftb_2,
                                signal.elevation,
                                signal.elevation_smooth,
                                signal.gradient,
                                signal.energy_consumption,
                                signal.match_latitude,
                                signal.match_longitude,
                                signal.match_type,
                                signal.speed_limit_type,
                                signal.speed_limit.clone(),
                                signal.speed_limit_direct.map(|f| f as i64),
                                signal.intersection.map(|f| f as i64),
                                signal.bus_stop.map(|f| f as i64),
                                signal.focus_points.clone(),
                                index
                            ])?;
        Ok(())
    }

    pub fn create_signal_indexes(&self) -> Result<usize> {
        let conn = self.connect()?;
        let sql = "CREATE INDEX IF NOT EXISTS signal_h3_idx ON main.signal (h3_12);";
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

        for vehicle in vehicles.iter().progress() {
            transaction.execute(sql, vehicle.to_tuple())?;
        }
        transaction.commit()
    }

    pub fn create_trajectory_table(&self) -> Result<usize> {
        let conn = self.connect()?;
        if !conn.table_exists(None, "main.trajectory")? {
            let sql = text_block! {
                "CREATE TABLE IF NOT EXISTS main.trajectory ("
                "    traj_id     INTEGER PRIMARY KEY AUTOINCREMENT,"
                "    vehicle_id  INTEGER NOT NULL,"
                "    trip_id     INTEGER NOT NULL,"
                "    length_m    DOUBLE,"
                "    dt_ini      TEXT,"
                "    dt_end      TEXT,"
                "    duration_s  DOUBLE,"
                "    h3_12_ini   INTEGER,"
                "    h3_12_end   INTEGER"
                ");" };
            conn.execute(sql, [])
        } else {
            Ok(0)
        }
    }

    pub fn insert_trajectories(&self) -> Result<usize> {
        let conn = self.connect()?;

        self.create_trajectory_table()?;

        let sql = text_block! {
            "INSERT INTO trajectory (vehicle_id, trip_id)"
            "    SELECT DISTINCT vehicle_id, trip_id FROM signal;"
        };
        conn.execute(sql, [])
    }

    pub fn get_trajectory_ids(&self) -> Result<Vec<i64>> {
        let conn = self.connect()?;
        let sql = text_block! {
            "SELECT traj_id FROM trajectory"
        };
        let mut stmt = conn.prepare(sql)?;
        let ids = stmt.query_map([], |row| row.get(0))?;
        let ids: Result<Vec<i64>> = ids.collect();
        ids
    }

    pub fn get_trajectory(&self, trajectory_id: i64) -> Result<Vec<TrajectoryPoint>> {
        let conn = self.connect()?;
        let sql = text_block! {
            "select     s.signal_id"
            ",          s.vehicle_id"
            ",          s.day_num"
            ",          s.time_stamp"
            ",          s.match_latitude"
            ",          s.match_longitude"
            "from       signal s"
            "inner join trajectory t on s.trip_id = t.trip_id"
            "where      t.traj_id = ?1"
        };
        let mut stmt = conn.prepare(sql)?;
        let points = stmt.query_map([trajectory_id],
                                    |row| {
                                        Ok(TrajectoryPoint {
                                            signal_id: row.get(0)?,
                                            vehicle_id: row.get(1)?,
                                            day_num: row.get(2)?,
                                            time_stamp: row.get(3)?,
                                            latitude: row.get(4)?,
                                            longitude: row.get(5)?,
                                        })
                                    });

        Ok(points?.collect::<Result<Vec<_>>>()?)
    }
}

use crate::db::api::SqliteDb;
use crate::models::signal::CsvSignal;
use crate::models::trajectory::{TrajectoryPoint, TrajectoryUpdate};
use crate::models::vehicle::Vehicle;
use crate::tools::lat_lng_to_h3_12;
use indicatif::ProgressIterator;
use std::result::Result;
use csv::DeserializeRecordsIter;
use rusqlite::{params, Connection, Error, Row, Transaction};
use text_block_macros::text_block;
use crate::models::node::Node;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    pub fn connect(&self) -> Result<Connection, Error> {
        let conn = self.db.connect()?;
        conn.execute("PRAGMA journal_mode=WAL", ())?;
        conn.execute("PRAGMA synchronous=NORMAL", ())?;
        conn.execute("PRAGMA cache_size=10000", ())?;
        conn.execute("PRAGMA temp_store=MEMORY", ())?;
        Ok(conn)
    }

    pub fn create_vehicle_table(&self) -> Result<usize, Error> {
        let try_conn = self.connect();

        let conn = match try_conn {
            Err(e) => return Err(e),
            Ok(conn) => conn,
        };

        conn.execute("DROP TABLE IF EXISTS vehicle;", ())?;

        let sql = "
            CREATE TABLE IF NOT EXISTS vehicle (
                vehicle_id    INTEGER primary key AUTOINCREMENT,
                vehicle_type  TEXT,
                vehicle_class TEXT,
                engine        TEXT,
                transmission  TEXT,
                drive_wheels  TEXT,
                weight        INTEGER
            ) STRICT";
        conn.execute(sql, ())
    }

    pub fn create_signal_table(&self) -> Result<usize, Error> {
        let try_conn = self.connect();

        let conn = match try_conn {
            Err(e) => return Err(e),
            Ok(conn) => conn,
        };

        conn.execute("DROP TABLE IF EXISTS signal;", ())?;
        let sql = text_block! {
        "create table if not exists signal ("
        "   signal_id          INTEGER primary key AUTOINCREMENT,"
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
        conn.execute(sql, ())
    }

    pub fn insert_signals(
        &self,
        signals: DeserializeRecordsIter<'_, &[u8], CsvSignal>,
    ) -> anyhow::Result<usize> {
        let mut conn = self.connect()?;
        let mut counter: usize = 0;

        let mut tx = conn.transaction()?;
        for signal in signals {
            if let Ok(s) = signal {
                self.insert_signal(&mut tx, &s)?;
                counter += 1;
            }
        }
        tx.commit()?;
        Ok(counter)
    }

    pub fn insert_signal(
        &self,
        tx: &mut Transaction<'_>,
        signal: &CsvSignal,
    ) -> Result<usize, Error> {
        let sql = text_block! {
            "INSERT INTO signal ("
            "   day_num, vehicle_id, trip_id, time_stamp, latitude, "
            "   longitude, speed, maf, rpm, abs_load, oat, fuel_rate, "
            "   ac_power_kw, ac_power_w, heater_power_w, hv_bat_current, "
            "   hv_bat_soc, hv_bat_volt, st_ftb_1, st_ftb_2, lt_ftb_1, "
            "   lt_ftb_2, elevation, elevation_smooth, gradient, "
            "   energy_consumption, match_latitude, match_longitude, "
            "   match_type, speed_limit_type, speed_limit, speed_limit_direct, "
            "   intersection, bus_stop, focus_points, h3_12) "
            "VALUES "
            "($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,"
            " $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34,"
            " $35, $36);"
        };

        let index: i64 = lat_lng_to_h3_12(signal.match_latitude, signal.match_longitude) as i64;
        let params = params!(
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
            signal.speed_limit,
            signal.speed_limit_direct,
            signal.intersection,
            signal.bus_stop,
            signal.focus_points.clone(),
            index,
            );
        tx.execute(sql, params)
    }

    pub fn create_signal_indexes(&self) -> Result<usize, Error> {
        let conn = self.connect()?;
        conn.execute("
        CREATE INDEX IF NOT EXISTS signal_vehicle_trip_idx ON signal (
            vehicle_id ASC,
            trip_id ASC,
            time_stamp ASC
        );", ())?;
        conn.execute("CREATE INDEX IF NOT EXISTS signal_h3_idx ON signal (h3_12);",
                     ())
    }

    pub fn insert_vehicles(
        &self,
        vehicles: Vec<Vehicle>,
    ) -> Result<usize, Error> {
        let sql = "
        INSERT INTO vehicle (
            vehicle_id,
            vehicle_type,
            vehicle_class,
            engine,
            transmission,
            drive_wheels,
            weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let mut conn = self.connect()?;
        let tx = conn.transaction()?;

        for vehicle in vehicles.iter().progress() {
            let params = params!(
                vehicle.vehicle_id,
                vehicle.vehicle_type,
                vehicle.vehicle_class,
                vehicle.engine,
                vehicle.transmission,
                vehicle.drive_wheels,
                vehicle.weight
            );
            tx.execute(sql, params)?;
        }
        tx.commit()?;
        Ok(vehicles.len())
    }

    pub fn create_trajectory_table(&self) -> Result<usize, Error> {
        let conn = self.connect()?;

        conn.execute("DROP TABLE IF EXISTS trajectory;", ())?;

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
        conn.execute(sql, ())
    }

    pub fn insert_trajectories(&self) -> Result<usize, Error> {
        let conn = self.connect()?;

        self.create_trajectory_table()?;

        let sql = text_block! {
            "INSERT INTO trajectory (vehicle_id, trip_id)"
            "    SELECT DISTINCT vehicle_id, trip_id FROM signal;"
        };
        conn.execute(sql, ())
    }

    pub fn update_trajectories(&self, updates: &[TrajectoryUpdate]) -> Result<(), Error> {
        let mut conn = self.connect()?;
        let tx = conn.transaction()?;
        let sql: String = String::from(
            "
            UPDATE      trajectory
            SET         length_m = ?
            ,           duration_s = ?
            ,           dt_ini = ?
            ,           dt_end = ?
            ,           h3_12_ini = ?
            ,           h3_12_end = ?
            WHERE       traj_id = ?
            ",
        );

        for update in updates.iter().progress() {
            let params = params!(
                update.length_m,
                update.duration_s,
                update.dt_ini,
                update.dt_end,
                update.h3_12_ini as i64,
                update.h3_12_end as i64,
                update.traj_id
            );
            tx.execute(&sql, params)?;
        }
        tx.commit()
    }

    pub fn get_trajectory_ids(&self) -> Result<Vec<i64>, Error> {
        let conn = self.connect()?;
        let sql = text_block! {
            "SELECT traj_id FROM trajectory"
        };
        let mut stmt = conn.prepare(sql)?;
        let traj_ids = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<i64>, Error>>()?;
        Ok(traj_ids)
    }

    pub fn get_trajectory_points(
        &self,
        trajectory_id: i64,
    ) -> Result<Vec<TrajectoryPoint>, Error> {
        let conn = self.connect()?;
        let sql = text_block! {
            "select     s.signal_id "
            ",          s.vehicle_id "
            ",          s.day_num "
            ",          s.time_stamp "
            ",          s.match_latitude "
            ",          s.match_longitude "
            "from       signal s "
            "inner join trajectory t on s.vehicle_id = t.vehicle_id and  s.trip_id = t.trip_id "
            "where      t.traj_id = ?1 "
            "order by   s.time_stamp "
        };
        let mut stmt = conn.prepare(sql)?;
        let points = stmt.query_map([trajectory_id], |row: &Row| {
            Ok(TrajectoryPoint {
                signal_id: row.get(0)?,
                vehicle_id: row.get(1)?,
                day_num: row.get(2)?,
                time_stamp: row.get(3)?,
                latitude: row.get(4)?,
                longitude: row.get(5)?,
            })
        })?;
        let results = points.collect::<Result<Vec<TrajectoryPoint>, Error>>()?;
        Ok(results)
    }

    pub fn create_trajectory_indexes(&self) -> Result<usize, Error> {
        let conn = self.connect()?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS traj_vehicle_idx ON trajectory (vehicle_id, trip_id);",
            ()
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS traj_h3_idx ON trajectory (h3_12_ini);",
            ()
        )
    }

    pub fn create_node_table(&self) -> Result<usize, Error> {
        let conn = self.connect()?;

        conn.execute("DROP TABLE IF EXISTS main.node;", ())?;
        let sql = text_block! {
        "CREATE TABLE IF NOT EXISTS node ("
        "    node_id         INTEGER PRIMARY KEY AUTOINCREMENT,"
        "    traj_id         INTEGER NOT NULL,"
        "    latitude        DOUBLE,"
        "    longitude       DOUBLE,"
        "    h3_12           INTEGER,"
        "    match_error     TEXT"
        ");" };

        conn.execute(sql, ())
    }
    
    pub fn insert_match_error(
        &self, 
        trajectory_id: i64, 
        match_error: &str
    ) -> Result<usize, Error> {
        let conn = self.connect()?;
        let sql = text_block! {
            "INSERT INTO node "
            "    (traj_id, match_error) "
            "VALUES "
            "    (?1, ?2);"
        };
        conn.execute(sql, params!(trajectory_id, match_error))
    }

    pub fn insert_nodes(&self, nodes: Vec<Node>) -> Result<(), Error> {
        let mut conn = self.connect()?;

        let sql = text_block! {
            "INSERT INTO node "
            "    (traj_id, latitude, longitude, h3_12) "
            "VALUES "
            "    (?1, ?2, ?3, ?4);"
        };
        let tx = conn.transaction()?;
        for node in nodes.iter() {
            tx.execute(sql, params!(
                node.trajectory_id,
                node.latitude,
                node.longitude,
                node.h3_12))?;
        }
        tx.commit()
    }
}

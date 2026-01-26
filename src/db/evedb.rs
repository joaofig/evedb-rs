use crate::db::api::SqliteDb;
use crate::models::signal::CsvSignal;
use crate::models::trajectory::{TrajectoryPoint, TrajectoryUpdate};
use crate::models::vehicle::Vehicle;
use crate::tools::lat_lng_to_h3_12;
use csv::DeserializeRecordsIter;
use indicatif::ProgressIterator;
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use sqlx::{Error, Executor, Pool, Row, Sqlite, Transaction};
use std::result::Result;
use text_block_macros::text_block;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    pub async fn connect(&self) -> Result<Pool<Sqlite>, Error> {
        self.db.connect().await
    }

    pub async fn create_vehicle_table(&self) -> Result<SqliteQueryResult, Error> {
        let try_conn = self.connect().await;

        let conn = match try_conn {
            Err(e) => return Err(e),
            Ok(conn) => conn,
        };

        conn.execute("DROP TABLE IF EXISTS vehicle;")
            .await
            .expect("Failed to drop vehicle table");

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
        conn.execute(sql).await
    }

    pub async fn create_signal_table(&self) -> Result<SqliteQueryResult, Error> {
        let try_conn = self.connect().await;

        let conn = match try_conn {
            Err(e) => return Err(e),
            Ok(conn) => conn,
        };

        conn.execute("DROP TABLE IF EXISTS signal;")
            .await
            .expect("Failed to drop signal table");
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
        conn.execute(sql).await
    }

    pub async fn insert_signals(
        &self,
        signals: DeserializeRecordsIter<'_, &[u8], CsvSignal>,
    ) -> Result<SqliteQueryResult, Error> {
        let conn = self.connect().await?;

        sqlx::query("PRAGMA synchronous = OFF")
            .execute(&conn)
            .await?;
        sqlx::query("PRAGMA journal_mode = MEMORY")
            .execute(&conn)
            .await?;

        let mut tx = conn.begin().await?;
        for signal in signals.into_iter().flatten() {
            self.insert_signal(&mut tx, &signal).await?;
        }
        tx.commit().await?;
        Ok(SqliteQueryResult::default())
    }

    // pub async fn prepare_insert_signal(&self, conn: &mut SqlitePool,) {
    //     let sql = text_block! {
    //         "INSERT INTO signal ("
    //         "   day_num, vehicle_id, trip_id, time_stamp, latitude, "
    //         "   longitude, speed, maf, rpm, abs_load, oat, fuel_rate, "
    //         "   ac_power_kw, ac_power_w, heater_power_w, hv_bat_current, "
    //         "   hv_bat_soc, hv_bat_volt, st_ftb_1, st_ftb_2, lt_ftb_1, "
    //         "   lt_ftb_2, elevation, elevation_smooth, gradient, "
    //         "   energy_consumption, match_latitude, match_longitude, "
    //         "   match_type, speed_limit_type, speed_limit, speed_limit_direct, "
    //         "   intersection, bus_stop, focus_points, h3_12) "
    //         "VALUES "
    //         "($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,"
    //         " $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34,"
    //         " $35, $36);"
    //     };
    //     let statement = sqlx::query(sql)
    //         .persistent(false)
    //         .prepare(conn)
    //         .await;
    // }

    pub async fn insert_signal(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        signal: &CsvSignal,
    ) -> Result<SqliteQueryResult, Error> {
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

        sqlx::query(sql)
            .bind(signal.day_num as i64)
            .bind(signal.vehicle_id as i64)
            .bind(signal.trip_id as i64)
            .bind(signal.time_stamp as i64)
            .bind(signal.latitude)
            .bind(signal.longitude)
            .bind(signal.speed)
            .bind(signal.maf)
            .bind(signal.rpm)
            .bind(signal.abs_load)
            .bind(signal.oat)
            .bind(signal.fuel_rate)
            .bind(signal.ac_power_kw)
            .bind(signal.ac_power_w)
            .bind(signal.heater_power_w)
            .bind(signal.hv_bat_current)
            .bind(signal.hv_bat_soc)
            .bind(signal.hv_bat_volt)
            .bind(signal.st_ftb_1)
            .bind(signal.st_ftb_2)
            .bind(signal.lt_ftb_1)
            .bind(signal.lt_ftb_2)
            .bind(signal.elevation)
            .bind(signal.elevation_smooth)
            .bind(signal.gradient)
            .bind(signal.energy_consumption)
            .bind(signal.match_latitude)
            .bind(signal.match_longitude)
            .bind(signal.match_type)
            .bind(signal.speed_limit_type)
            .bind(&signal.speed_limit)
            .bind(signal.speed_limit_direct.map(|f| f as i64))
            .bind(signal.intersection.map(|f| f as i64))
            .bind(signal.bus_stop.map(|f| f as i64))
            .bind(signal.focus_points.clone())
            .bind(index)
            .execute(&mut **tx)
            .await
    }

    pub async fn create_signal_indexes(&self) -> Result<SqliteQueryResult, Error> {
        let conn = self.connect().await?;
        conn.execute("CREATE INDEX IF NOT EXISTS signal_trip_idx ON signal (trip_id);")
            .await?;
        conn.execute("CREATE INDEX IF NOT EXISTS signal_h3_idx ON signal (h3_12);")
            .await
    }

    pub async fn insert_vehicles(
        &self,
        vehicles: Vec<Vehicle>,
    ) -> Result<SqliteQueryResult, Error> {
        let sql = "
        INSERT INTO vehicle (
            vehicle_id,
            vehicle_type,
            vehicle_class,
            engine,
            transmission,
            drive_wheels,
            weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let conn = self.connect().await?;
        let tx = conn.begin().await?;

        for vehicle in vehicles.iter().progress() {
            let _ = sqlx::query(sql)
                .bind(vehicle.vehicle_id)
                .bind(&vehicle.vehicle_type)
                .bind(&vehicle.vehicle_class)
                .bind(&vehicle.engine)
                .bind(&vehicle.transmission)
                .bind(&vehicle.drive_wheels)
                .bind(vehicle.weight)
                .execute(&conn)
                .await?;
        }
        match tx.commit().await {
            Ok(_) => Ok(SqliteQueryResult::default()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_trajectory_table(&self) -> Result<SqliteQueryResult, Error> {
        let conn = self.connect().await?;

        conn.execute("DROP TABLE IF EXISTS trajectory;").await?;

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
        conn.execute(sql).await
    }

    pub async fn insert_trajectories(&self) -> Result<SqliteQueryResult, Error> {
        let conn = self.connect().await?;

        self.create_trajectory_table().await?;

        let sql = text_block! {
            "INSERT INTO trajectory (vehicle_id, trip_id)"
            "    SELECT DISTINCT vehicle_id, trip_id FROM signal;"
        };
        conn.execute(sql).await
    }

    pub async fn update_trajectories(&self, updates: &Vec<TrajectoryUpdate>) -> Result<(), Error> {
        let conn = self.connect().await?;
        let mut tx = conn.begin().await?;
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
            sqlx::query(&sql)
                .bind(update.length_m)
                .bind(update.duration_s)
                .bind(&update.dt_ini)
                .bind(&update.dt_end)
                .bind(update.h3_12_ini as i64)
                .bind(update.h3_12_end as i64)
                .bind(update.traj_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await
    }

    pub async fn get_trajectory_ids(&self) -> Result<Vec<SqliteRow>, Error> {
        let conn = self.connect().await?;
        let sql = text_block! {
            "SELECT traj_id FROM trajectory"
        };
        sqlx::query(sql).fetch_all(&conn).await
    }

    pub async fn get_trajectory_points(
        &self,
        trajectory_id: i64,
    ) -> Result<Vec<TrajectoryPoint>, Error> {
        let conn = self.connect().await?;
        let sql = text_block! {
            "select     s.signal_id"
            ",          s.vehicle_id"
            ",          s.day_num"
            ",          s.time_stamp"
            ",          s.match_latitude"
            ",          s.match_longitude"
            "from       signal s"
            "inner join trajectory t on s.trip_id = t.trip_id and s.vehicle_id = t.vehicle_id"
            "where      t.traj_id = ?1"
            "order by   s.time_stamp"
        };
        sqlx::query(sql)
            .bind(trajectory_id)
            .fetch_all(&conn)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| TrajectoryPoint {
                        signal_id: row.get(0),
                        vehicle_id: row.get(1),
                        day_num: row.get(2),
                        time_stamp: row.get(3),
                        latitude: row.get(4),
                        longitude: row.get(5),
                    })
                    .collect::<Vec<_>>()
            })
    }

    pub async fn update_trajectory(
        &self,
        trajectory: &TrajectoryUpdate,
    ) -> Result<SqliteRow, Error> {
        let conn = self.connect().await?;
        let sql = text_block! {
            "UPDATE trajectory"
            "SET    length_m = ?1"
            ",      dt_ini = ?2"
            ",      dt_end = ?3"
            ",      duration_s = ?4"
            ",      h3_12_ini = ?5"
            ",      h3_12_end = ?6"
            "WHERE  traj_id = ?7;"
        };
        sqlx::query(sql)
            .bind(trajectory.length_m)
            .bind(&trajectory.dt_ini)
            .bind(&trajectory.dt_end)
            .bind(trajectory.duration_s)
            .bind(trajectory.h3_12_ini as i64)
            .bind(trajectory.h3_12_end as i64)
            .bind(trajectory.traj_id)
            .fetch_one(&conn)
            .await
    }

    pub async fn create_node_table(&self) -> Result<SqliteQueryResult, Error> {
        let conn = self.connect().await?;

        conn.execute("DROP TABLE IF EXISTS main.node;").await?;
        let sql = text_block! {
        "CREATE TABLE IF NOT EXISTS node ("
        "    node_id         INTEGER PRIMARY KEY AUTOINCREMENT,"
        "    traj_id         INTEGER NOT NULL,"
        "    latitude        DOUBLE,"
        "    longitude       DOUBLE,"
        "    h3_12           INTEGER,"
        "    match_error     TEXT"
        ");" };

        conn.execute(sql).await
    }
}

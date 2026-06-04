use anyhow::anyhow;
use indicatif::ProgressIterator;
use rusqlite::{params, Error, Row};
use text_block_macros::text_block;
use crate::db::evedb::EveDb;
use crate::models::trajectory::{TrajectoryPoint, TrajectoryUpdate, WayPoint};

pub fn insert_trajectories(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    db.create_trajectory_table()?;
    db.create_trajectory_indexes()?;

    let sql = text_block! {
            "INSERT INTO trajectory (vehicle_id, trip_id)"
            "    SELECT DISTINCT vehicle_id, trip_id FROM signal;"
        };
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to insert trajectories: {:?}", e))
}

pub fn update_trajectories(db: &EveDb, updates: &[TrajectoryUpdate]) -> anyhow::Result<()> {
    let mut conn = db.connect()?;
    let tx = conn.transaction()?;
    let sql: String = String::from(
        "
            UPDATE trajectory
            SET    length_m = ?
            ,      duration_s = ?
            ,      dt_ini = ?
            ,      dt_end = ?
            ,      h3_12_ini = ?
            ,      h3_12_end = ?
            WHERE  traj_id = ?
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
                update.trajectory_id
            );
        tx.execute(&sql, params)?;
    }
    tx.commit()
        .map_err(|e| anyhow!("Failed to update trajectories: {:?}", e))
}

pub fn get_trajectory_ids(db: &EveDb) -> anyhow::Result<Vec<i64>> {
    let conn = db.connect()?;
    let sql = text_block! {
            "SELECT traj_id FROM trajectory"
        };
    let mut stmt = conn.prepare(sql)?;
    let traj_ids = stmt
        .query_map([], |row| row.get(0))?
        .collect::<anyhow::Result<Vec<i64>, Error>>()?;
    Ok(traj_ids)
}

pub fn get_trajectory_points(db: &EveDb, trajectory_id: i64) -> anyhow::Result<Vec<TrajectoryPoint>> {
    let conn = db.connect()?;
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
    let results = points.collect::<anyhow::Result<Vec<TrajectoryPoint>, Error>>()?;
    Ok(results)
}

pub fn get_way_points(db: &EveDb, trajectory_id: i64) -> anyhow::Result<Vec<WayPoint>> {
    let conn = db.connect()?;
    let sql = text_block! {
            "select     s.latitude as lat"
            ",          s.longitude as lon"
            ",          min(s.time_stamp) / 1000 as time"
            "from       signal s"
            "inner join trajectory t on s.vehicle_id = t.vehicle_id and s.trip_id = t.trip_id"
            "where      t.traj_id = ?1"
            "group by   s.latitude, s.longitude"
            "order by   time;"
        };
    let mut stmt = conn.prepare(sql)?;
    let points = stmt.query_map([trajectory_id], |row: &Row| {
        Ok(WayPoint {
            time: row.get(2)?,
            latitude: row.get(0)?,
            longitude: row.get(1)?,
        })
    })?;
    let results = points.collect::<anyhow::Result<Vec<WayPoint>, Error>>()?;
    Ok(results)
}

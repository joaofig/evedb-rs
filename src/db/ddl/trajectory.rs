use crate::db::evedb::EveDb;
use anyhow::anyhow;
use text_block_macros::text_block;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS trajectory;", ())?;

    let sql = text_block! {
    "CREATE TABLE IF NOT EXISTS main.trajectory ("
    "    traj_id     INTEGER PRIMARY KEY,"
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
        .map_err(|e| anyhow!("Failed to create trajectory table: {:?}", e))
}

pub fn create_error_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "DROP TABLE IF EXISTS trajectory_match_error;",
        (),
    )?;
    let sql = text_block! {
        "CREATE TABLE IF NOT EXISTS trajectory_match_error ("
        "    traj_id     INTEGER NOT NULL,"
        "    match_error TEXT NOT NULL,"
        "    FOREIGN KEY (traj_id) REFERENCES trajectory(traj_id)"
        ");"
    };
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create trajectory match error table: {:?}", e))
}

pub fn create_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS traj_vehicle_idx ON trajectory (vehicle_id, trip_id);",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS traj_h3_idx ON trajectory (h3_12_ini);",
        (),
    )
    .map_err(|e| anyhow!("Failed to create trajectory indexes: {:?}", e))
}

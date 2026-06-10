use crate::db::evedb::EveDb;
use anyhow::anyhow;
use text_block_macros::text_block;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS trajectory;", ())?;

    let sql = include_str!("sql/create_table_trajectory.sql");
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create trajectory table: {:?}", e))
}

pub fn create_error_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "DROP TABLE IF EXISTS trajectory_match_error;",
        (),
    )?;

    let sql = include_str!("sql/create_table_trajectory_match_error.sql");
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

use crate::db::evedb::EveDb;
use anyhow::anyhow;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS vehicle;", ())?;

    let sql = "
            CREATE TABLE IF NOT EXISTS vehicle (
                vehicle_id    INTEGER PRIMARY KEY,
                vehicle_type  TEXT,
                vehicle_class TEXT,
                engine        TEXT,
                transmission  TEXT,
                drive_wheels  TEXT,
                weight        INTEGER
            ) STRICT";
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create vehicle table: {:?}", e))
}

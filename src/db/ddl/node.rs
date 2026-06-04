use anyhow::anyhow;
use text_block_macros::text_block;
use crate::db::evedb::EveDb;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS main.node;", ())?;
    let sql = text_block! {
        "CREATE TABLE IF NOT EXISTS node ("
        "    node_id         INTEGER PRIMARY KEY,"
        "    traj_id         INTEGER NOT NULL,"
        "    latitude        DOUBLE,"
        "    longitude       DOUBLE,"
        "    h3_12           INTEGER,"
        "    match_error     TEXT"
        ");" };

    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create node table: {:?}", e))
}


pub fn create_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS node_h3_idx ON node (h3_12);",
        (),
    )
        .map_err(|e| anyhow!("Failed to create node indexes: {:?}", e))
}

use crate::db::evedb::EveDb;
use anyhow::anyhow;
use text_block_macros::text_block;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS main.edge;", ())?;
    let sql = text_block! {
    "CREATE TABLE IF NOT EXISTS edge ("
        "edge_id         INTEGER PRIMARY KEY,"
        "lat_ini         DOUBLE,"
        "lon_ini         DOUBLE,"
        "lat_end         DOUBLE,"
        "lon_end         DOUBLE,"
        "h3_12_ini       INTEGER,"
        "h3_12_end       INTEGER,"
        "length_m        DOUBLE,"
        "heading_deg     DOUBLE"
    ");" };

    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create node table: {:?}", e))
}

pub fn create_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS edge_hash_idx ON edge (hash);",
        (),
    )
    .map_err(|e| anyhow!("Failed to create edge indexes: {:?}", e))
}

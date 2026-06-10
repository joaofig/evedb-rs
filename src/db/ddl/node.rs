use crate::db::evedb::EveDb;
use anyhow::anyhow;
use text_block_macros::text_block;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS main.node;", ())?;
    let sql = text_block! {
    "CREATE TABLE IF NOT EXISTS node ("
        "node_id         INTEGER PRIMARY KEY,"
        "latitude        DOUBLE,"
        "longitude       DOUBLE,"
        "altitude        DOUBLE,"
        "h3_12           INTEGER"
    // "    match_error     TEXT"
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

pub fn create_traj_node_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS traj_node;", ())?;

    let sql = text_block! {
        "CREATE TABLE IF NOT EXISTS traj_node ("
        "    traj_node_id   INTEGER PRIMARY KEY,"
        "    traj_id        INTEGER NOT NULL,"
        "    node_id        INTEGER NOT NULL,"
        "    FOREIGN KEY (traj_id) REFERENCES trajectory(traj_id),"
        "    FOREIGN KEY (node_id) REFERENCES node(node_id)"
        ");"
    };
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create trajectory node table: {:?}", e))
}

pub fn create_traj_node_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS traj_node_idx ON traj_node (traj_id, node_id);",
        (),
    )
        .map_err(|e| anyhow!("Failed to create trajectory node indexes: {:?}", e))
}

use crate::db::evedb::EveDb;
use anyhow::anyhow;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS main.edge;", ())?;
    let sql = include_str!("sql/create_table_edge.sql");

    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create edge table: {:?}", e))
}

pub fn create_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        include_str!("sql/create_index_edge_nodes_idx.sql"),
        (),
    )
    .map_err(|e| anyhow!("Failed to create edge indexes: {:?}", e))
}

pub fn create_traj_edge_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS traj_edge;", ())?;
    let sql = include_str!("sql/create_table_traj_edge.sql");
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create trajectory edge table: {:?}", e))
}

pub fn create_traj_edge_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS traj_edge_idx ON traj_edge (traj_id, edge_id);",
        (),
    )
        .map_err(|e| anyhow!("Failed to create trajectory edge indexes: {:?}", e))
}

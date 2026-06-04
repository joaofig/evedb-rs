use crate::db::evedb::EveDb;
use crate::models::node::Node;
use anyhow::anyhow;
use rusqlite::params;
use text_block_macros::text_block;

pub fn insert_match_error(
    db: &EveDb,
    trajectory_id: i64,
    match_error: &str,
) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    let sql = text_block! {
        "INSERT INTO node "
        "    (traj_id, match_error) "
        "VALUES "
        "    (?1, ?2);"
    };
    conn.execute(sql, params!(trajectory_id, match_error))
        .map_err(|e| anyhow!("Failed to insert match error: {:?}", e))
}

pub fn insert_nodes(db: &EveDb, nodes: impl Iterator<Item = Node>) -> anyhow::Result<()> {
    let mut conn = db.connect()?;

    let sql = text_block! {
        "INSERT INTO node "
        "    (traj_id, latitude, longitude, h3_12) "
        "VALUES "
        "    (?1, ?2, ?3, ?4);"
    };
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(sql)?;
        for node in nodes {
            stmt.execute(params!(node.id, node.latitude, node.longitude, node.h3_12))?;
        }
    }
    tx.commit()
        .map_err(|e| anyhow!("Failed to insert nodes: {:?}", e))
}

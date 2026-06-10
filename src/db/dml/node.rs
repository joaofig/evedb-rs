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
        "INSERT INTO trajectory_match_error "
        "    (traj_id, match_error) "
        "VALUES "
        "    (?1, ?2);"
    };
    conn.execute(sql, params!(trajectory_id, match_error))
        .map_err(|e| anyhow!("Failed to insert match error: {:?}", e))
}

pub fn insert_nodes(db: &EveDb, traj_id: i64,
                    nodes: &mut Vec<Node>) -> anyhow::Result<()> {
    let mut conn = db.connect()?;

    let sql = text_block! {
        "INSERT INTO node "
        "    (latitude, longitude, altitude, h3_12) "
        "VALUES "
        "    (?1, ?2, ?3, ?4)"
        " RETURNING node_id;"
    };
    let sql_traj_node = "INSERT OR IGNORE INTO traj_node (traj_id, node_id) VALUES (?1, ?2);";
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(sql)?;
        let mut stmt_traj_node = tx.prepare(sql_traj_node)?;
        for node in nodes {
            // Only insert new nodes in the table
            let node_id = if node.id == 0 {
                stmt.query_row(
                    params!(node.latitude, node.longitude, node.altitude, node.h3_12),
                    |row| row.get(0),
                )?
            } else {
                node.id
            };
            node.id = node_id;
            stmt_traj_node.execute(params!(traj_id, node_id))?;
        }
    }
    tx.commit()
        .map_err(|e| anyhow!("Failed to insert nodes: {:?}", e))
}

pub fn get_ring(db: &EveDb, ring: Vec<u64>) -> anyhow::Result<Vec<Node>> {
    let conn = db.connect()?;
    
    // rusqlite doesn't implement ToSql for u64, so we cast to i64
    let ring_i64: Vec<i64> = ring.into_iter().map(|x| x as i64).collect();
    
    let vars = vec!["?"; ring_i64.len()].join(", ");
    let sql = format!(
        "SELECT node_id, latitude, longitude, altitude, h3_12 FROM node WHERE h3_12 IN ({});",
        vars
    );

    let mut stmt = conn.prepare(&sql)?;
    let nodes = stmt.query_map(rusqlite::params_from_iter(ring_i64), |row| {
        Ok(Node {
            id: row.get(0)?,
            latitude: row.get(1)?,
            longitude: row.get(2)?,
            altitude: row.get(3)?,
            h3_12: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(nodes)
}
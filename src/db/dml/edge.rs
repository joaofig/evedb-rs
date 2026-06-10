use anyhow::anyhow;
use rusqlite::params;
use text_block_macros::text_block;
use crate::db::evedb::EveDb;
use crate::models::node::Node;


fn find_edge(db: &EveDb, node_ini: i64, node_end: i64) -> anyhow::Result<i64> {
    let conn = db.connect()?;
    let sql = text_block! {
        "SELECT edge_id FROM edge WHERE node_ini = ?1 AND node_end = ?2;"
    };
    let edge_id = conn.query_row(sql, params!(node_ini, node_end), |row| row.get(0))?;
    Ok(edge_id)
}

pub fn insert_edges(db: &EveDb, traj_id: i64,
                    nodes: &[Node]) -> anyhow::Result<()> {
    let mut conn = db.connect()?;

    let sql_edge = text_block! {
            "INSERT INTO edge "
            "    (node_ini, node_end, length_m, bearing_deg) "
            "VALUES "
            "    (?1, ?2, ?3, ?4)"
            " RETURNING edge_id;"
        };
    let sql_traj_edge = text_block! {
            "INSERT INTO traj_edge "
            "    (traj_id, edge_id) "
            "VALUES "
            "    (?1, ?2);"
        };
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(sql_edge)?;
        let mut stmt_traj_edge = tx.prepare(sql_traj_edge)?;

        for nodes in nodes.windows(2) {
            match find_edge(db, nodes[0].id, nodes[1].id) {
                Ok(edge_id) => {
                    stmt_traj_edge.execute(params!(traj_id, edge_id))?;
                }
                Err(_) => {
                    let distance_m = nodes[0].distance_to(&nodes[1]);
                    let bearing_deg = nodes[0].bearing_to(&nodes[1]);
                    let edge_id: i64 =  stmt.query_row(
                            params!(nodes[0].id, nodes[1].id, distance_m, bearing_deg),
                            |row| row.get(0))?;
                    stmt_traj_edge.execute(params!(traj_id, edge_id))?;
                },
            }
        }
    }
    tx.commit()
        .map_err(|e| anyhow!("Failed to insert nodes: {:?}", e))
}

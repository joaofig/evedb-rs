use bon::Builder;

#[derive(Builder)]
pub struct Edge {
    pub id: i64,
    pub node_ini: i64,
    pub node_end: i64,
    pub length_m: i64,
}

#[derive(Builder, Debug)]
pub struct TrajectoryEdge {
    pub id: i64,
    pub trajectory_id: i64,
    pub edge_id: i64,
}

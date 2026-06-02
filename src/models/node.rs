use bon::Builder;

#[derive(Builder)]
pub struct Node {
    pub id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub h3_12: i64,
}


#[derive(Builder, Debug)]
pub struct TrajectoryNode {
    pub id: i64,
    pub trajectory_id: i64,
    pub node_id: i64,
}

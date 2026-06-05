use bon::Builder;
use geo::{HaversineDistance, Point};
use serde::{Deserialize, Serialize};

#[derive(Builder, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub h3_12: i64,
}

impl Node {
    pub fn distance_to_point(&self, point: &Point) -> f64 {
        let node_point = Point::new(self.longitude, self.latitude);
        node_point.haversine_distance(point)
    }
}

#[derive(Builder, Debug)]
pub struct TrajectoryNode {
    pub id: i64,
    pub trajectory_id: i64,
    pub node_id: i64,
}

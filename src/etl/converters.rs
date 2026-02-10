use valhalla_client::route::ShapePoint;
use crate::models::trajectory::{TrajectoryPoint, WayPoint};

impl From<&TrajectoryPoint> for ShapePoint {
    fn from(point: &TrajectoryPoint) -> Self {
        ShapePoint {
            lat: point.latitude,
            lon: point.longitude,
        }
    }
}

impl From<&WayPoint> for ShapePoint {
    fn from(point: &WayPoint) -> Self {
        ShapePoint {
            lat: point.latitude,
            lon: point.longitude,
        }
    }
}
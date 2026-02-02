use valhalla_client::route::Location;
use crate::models::trajectory::TrajectoryPoint;

impl From<&TrajectoryPoint> for Location {
    fn from(point: &TrajectoryPoint) -> Self {
        Location::new(point.longitude as f32, point.latitude as f32)
    }
}

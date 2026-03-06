use crate::models::trajectory::{TrajectoryPoint, WayPoint};
use valhalla_client::route::ShapePoint;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_point_to_shape_point() {
        let tp = TrajectoryPoint {
            signal_id: 1,
            vehicle_id: 1,
            day_num: 1.0,
            latitude: 42.0,
            longitude: -83.0,
            time_stamp: 1000,
        };
        let sp: ShapePoint = (&tp).into();
        assert_eq!(sp.lat, 42.0);
        assert_eq!(sp.lon, -83.0);
    }

    #[test]
    fn test_way_point_to_shape_point() {
        let wp = WayPoint {
            time: 1000,
            latitude: 43.0,
            longitude: -84.0,
        };
        let sp: ShapePoint = (&wp).into();
        assert_eq!(sp.lat, 43.0);
        assert_eq!(sp.lon, -84.0);
    }
}

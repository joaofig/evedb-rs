use valhalla_client::trace_route::{Manifest, ShapeMatchType, TraceOptions};
use valhalla_client::Valhalla;
use valhalla_client::route::{Location, Trip};
use crate::tools::lat_lng_to_h3_12;
use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::trajectory::TrajectoryPoint;
use crate::models::node::Node;

/// Decode a polyline string.
pub fn decode_polyline(polyline: &str, precision: f64) -> Vec<(f64, f64)> {
    let mut shape = Vec::new();

    let mut chars = polyline.chars();
    let mut last_lat = 0;
    let mut last_lon = 0;

    // Get the next latitude/longitude tuple.
    let mut next_coordinates = || {
        last_lat = parse_polyline_coordinate(&mut chars, last_lat)?;
        last_lon = parse_polyline_coordinate(&mut chars, last_lon)?;
        Some((last_lat, last_lon))
    };

    while let Some((lat, lon)) = next_coordinates() {
        shape.push((lat as f64 / precision, lon as f64 / precision));
    }
    shape
}

/// Parse the next latitude or longitude in the polyline string.
fn parse_polyline_coordinate(mut chars: impl Iterator<Item = char>, previous: i32) -> Option<i32> {
    let mut byte = None;
    let mut result = 0;
    let mut shift = 0;

    while byte.is_none_or(|b| b >= 0x20) {
        let byte = *byte.insert(chars.next()? as i32 - 63);
        result |= (byte & 0x1f) << shift;
        shift += 5;
    }

    let value = if result & 1 != 0 {
        previous + !(result >> 1)
    } else {
        previous + (result >> 1)
    };
    Some(value)
}

#[test]
fn decode_polyline6() {
    let x = decode_polyline("e~epoA|jfpOiDaK", 1E6);
    let decoded = vec![(42.225139, -8.670911), (42.225224, -8.670718)];
    assert_eq!(x, decoded);
}

async fn map_match(locations: Vec<Location>) -> Result<Trip, valhalla_client::Error> {
    let trace_options = TraceOptions::builder()
        .search_radius(100.0)
        .gps_accuracy(10.0);
    let manifest: Manifest = Manifest::builder()
        .shape_match(ShapeMatchType::WalkOrSnap)
        .shape(locations)
        .use_timestamps(false)
        .trace_options(trace_options);

    let valhalla = Valhalla::default();
    valhalla.trace_route(manifest).await
}

pub(crate) async fn build_nodes(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    if cli.verbose {
        println!("Creating the node table")
    }

    let result = db.create_node_table().await;
    if result.is_err() {
        panic!(
            "Failed to create node table {}",
            result.err().unwrap()
        );
    }

    if cli.verbose {
        println!("Populating the node table")
    }

    let trajectory_ids = db.get_trajectory_ids().await.unwrap_or(vec![]);
    for trajectory_id in trajectory_ids {
        let trajectory = db.get_trajectory_points(trajectory_id).await.unwrap();
        let locations: Vec<Location> =
            trajectory.iter().map(|p: &TrajectoryPoint| p.into()).collect();
        let try_trip = map_match(locations).await;
        match try_trip {
            Ok(trip) => {
                if let Some(warnings) = trip.warnings {
                    let warnings = format!("{:?}", warnings);
                    db.insert_match_error(trajectory_id, &warnings).await.unwrap();
                    println!("Map matching warnings for trajectory {}: {:?}", trajectory_id, warnings);
                } else {
                    let points = trip.locations.iter().map(|pt| Node {
                        trajectory_id,
                        latitude: pt.latitude as f64,
                        longitude: pt.longitude as f64,
                        h3_12: lat_lng_to_h3_12(pt.latitude as f64, pt.longitude as f64) as i64,
                        match_error: None,
                    });
                    db.insert_nodes(points.collect()).await.unwrap();
                }
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
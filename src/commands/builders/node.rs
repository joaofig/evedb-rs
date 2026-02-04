use indicatif::ProgressIterator;
use sqlx::sqlite::SqliteQueryResult;
use valhalla_client::costing::{AutoCostingOptions, Costing};
use valhalla_client::trace_route::{Manifest, ShapeMatchType, TraceOptions};
use valhalla_client::Valhalla;
use valhalla_client::route::{Location, Trip};
use crate::tools::lat_lng_to_h3_12;
use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::trajectory::TrajectoryPoint;
use crate::models::node::Node;

async fn map_match(locations: Vec<Location>) -> Result<Trip, valhalla_client::Error> {
    let trace_options = TraceOptions::builder()
        .search_radius(100.0)
        .gps_accuracy(10.0);
    let manifest: Manifest = Manifest::builder()
        .shape_match(ShapeMatchType::WalkOrSnap)
        .shape(locations)
        .use_timestamps(false)
        .trace_options(trace_options)
        .costing(Costing::Auto(AutoCostingOptions::default()));

    let valhalla = Valhalla::default();
    valhalla.trace_route(manifest).await
}

pub(crate) async fn build_nodes(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    if cli.verbose {
        println!("Creating the node table")
    }

    let result: Result<SqliteQueryResult, sqlx::Error> = db.create_node_table().await;
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
    for trajectory_id in trajectory_ids.iter().progress() {
        let trajectory = db.get_trajectory_points(*trajectory_id).await.unwrap();
        let locations: Vec<Location> =
            trajectory.iter().map(|p: &TrajectoryPoint| p.into()).collect();
        let result_trip = map_match(locations).await;
        match result_trip {
            Ok(trip) => {
                if let Some(warnings) = trip.warnings {
                    let message = format!("{:?}", warnings);
                    db.insert_match_error(*trajectory_id, &message).await.unwrap();
                    // println!("Map matching warnings for trajectory {}: {:?}", trajectory_id, warnings);
                } else {
                    let nodes =
                        trip.legs.iter()
                            .flat_map(|leg| leg.shape.iter()).map(|pt|
                                Node {
                                    trajectory_id: *trajectory_id,
                                    latitude: pt.lat,
                                    longitude: pt.lon,
                                    h3_12: lat_lng_to_h3_12(pt.lat, pt.lon) as i64,
                                }
                            );
                    db.insert_nodes(nodes.collect()).await.unwrap();
                }
            }
            Err(e) => {
                let message = format!("Failed to map match trajectory {}: {:?}",
                                      trajectory_id, e);
                db.insert_match_error(*trajectory_id, &message).await.unwrap();
            }
        }
    }
}
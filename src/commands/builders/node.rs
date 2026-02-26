use anyhow::{anyhow, Result};
use indicatif::{ProgressIterator};
use valhalla_client::costing::{AutoCostingOptions, Costing};
use valhalla_client::trace_route::{Manifest, ShapeMatchType, TraceOptions};
use valhalla_client::{Valhalla};
use valhalla_client::route::{ShapePoint, Trip};
use crate::tools::lat_lng_to_h3_12;
use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::trajectory::WayPoint;
use crate::models::node::Node;
use url::Url;

async fn map_match(
    locations: impl Iterator<Item = ShapePoint>
) -> Result<Trip> {
    let valhalla = Valhalla::new(Url::parse("http://localhost:8002/")?);
    let trace_options = TraceOptions::builder()
        .search_radius(100.0)
        .gps_accuracy(10.0);
    let manifest: Manifest = Manifest::builder()
        .shape_match(ShapeMatchType::MapSnap)
        .shape(locations)
        .use_timestamps(false)
        .verbose(true)
        .trace_options(trace_options)
        .costing(Costing::Auto(AutoCostingOptions::default()));

    valhalla.trace_route(manifest).await
        .map_err(|e| anyhow!("Failed to map match: {:?}", e))
}

pub(crate) async fn build_nodes(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    if cli.verbose {
        println!("Creating the node table")
    }

    db.create_node_table().expect("Failed to create node table");

    if cli.verbose {
        println!("Populating the node table")
    }

    let trajectory_ids = db.get_trajectory_ids().unwrap_or(vec![]);
    for trajectory_id in trajectory_ids.iter().progress() {
        let way_points = db.get_way_points(*trajectory_id)
            .expect("Failed to get way points");
        let locations =
            way_points.iter().map(|p: &WayPoint| p.into());

        let result_trip = map_match(locations).await;
        match result_trip {
            Ok(trip) => {
                if let Some(warnings) = trip.warnings {
                    let message = format!("{:?}", warnings);
                    if db.insert_match_error(*trajectory_id, &message).is_err() {
                        eprintln!("{}", message);
                    }
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
                    let result = db.insert_nodes(nodes);
                    if let Err(e) = result {
                        let message = format!("Failed to insert nodes for trajectory {}: {:?}",
                                              trajectory_id, e);
                        if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                            eprintln!("{}: {}", message, e);
                        }
                    }
                }
            }
            Err(e) => {
                let message = format!("Failed to map match trajectory {}: {:?}",
                                      trajectory_id, e);
                if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                    eprintln!("{}: {}", message, e);
                }
            }
        }
    }
}
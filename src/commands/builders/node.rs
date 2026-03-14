use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::node::Node;
use crate::models::trajectory::WayPoint;
use crate::tools::lat_lng_to_h3_12;
use anyhow::{Result, anyhow};
use indicatif::ProgressIterator;
use url::Url;
use valhalla_client::Valhalla;
use valhalla_client::costing::{AutoCostingOptions, Costing};
use valhalla_client::route::{ShapePoint, Trip};
use valhalla_client::trace_route::{Manifest, ShapeMatchType, TraceOptions};

async fn map_match(
    valhalla: &Valhalla,
    locations: impl Iterator<Item = ShapePoint>,
) -> Result<Trip> {
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

    valhalla
        .trace_route(manifest)
        .await
        .map_err(|e| anyhow!("Failed to map match: {:?}", e))
}

pub async fn build_nodes(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    let valhalla_url =
        std::env::var("VALHALLA_URL").unwrap_or_else(|_| "http://localhost:8002/".to_string());
    let valhalla_url = match Url::parse(&valhalla_url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Invalid Valhalla URL '{}': {}", valhalla_url, e);
            return;
        }
    };
    let valhalla = Valhalla::new(valhalla_url.clone());

    if cli.verbose {
        println!("Checking Valhalla instance at {}", valhalla_url);
    }

    match valhalla
        .status(valhalla_client::status::Manifest::default())
        .await
    {
        Ok(_) => {
            if cli.verbose {
                println!("Valhalla instance is up and running");
            }
        }
        Err(e) => {
            eprintln!(
                "Valhalla instance at {} is unreachable or not responding correctly: {}. Please ensure Valhalla is running.",
                valhalla_url, e
            );
            return;
        }
    }

    if cli.verbose {
        println!("Creating the node table")
    }

    db.create_node_table().expect("Failed to create node table");

    if cli.verbose {
        println!("Populating the node table")
    }

    let trajectory_ids = db.get_trajectory_ids().unwrap_or(vec![]);
    for trajectory_id in trajectory_ids.iter().progress() {
        let way_points = db
            .get_way_points(*trajectory_id)
            .expect("Failed to get way points");
        let locations = way_points.iter().map(|p: &WayPoint| p.into());

        let result_trip = map_match(&valhalla, locations).await;
        match result_trip {
            Ok(trip) => {
                if let Some(warnings) = trip.warnings {
                    let message = format!("{:?}", warnings);
                    if db.insert_match_error(*trajectory_id, &message).is_err() {
                        eprintln!("{}", message);
                    }
                } else {
                    let nodes = trip.legs.iter().flat_map(|leg| leg.shape.iter()).map(|pt| {
                        Node::builder()
                            .trajectory_id(*trajectory_id)
                            .latitude(pt.lat)
                            .longitude(pt.lon)
                            .h3_12(lat_lng_to_h3_12(pt.lat, pt.lon) as i64)
                            .build()
                    });
                    let result = db.insert_nodes(nodes);
                    if let Err(e) = result {
                        let message = format!(
                            "Failed to insert nodes for trajectory {}: {:?}",
                            trajectory_id, e
                        );
                        if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                            eprintln!("{}: {}", message, e);
                        }
                    }
                }
            }
            Err(e) => {
                let message = format!("Failed to map match trajectory {}: {:?}", trajectory_id, e);
                if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                    eprintln!("{}: {}", message, e);
                }
            }
        }
    }
}

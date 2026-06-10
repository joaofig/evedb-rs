use crate::cli::Cli;
use crate::db::dml::node::get_ring;
use crate::db::evedb::EveDb;
use crate::models::node::Node;
use crate::models::trajectory::WayPoint;
use crate::tools::lat_lng_to_h3_12;
use anyhow::{anyhow, Result};
use geo::Point;
use indicatif::ProgressIterator;
use std::cmp::Ordering;
use url::Url;
use valhalla_client::costing::{AutoCostingOptions, Costing};
use valhalla_client::route::{DirectionsType, ShapePoint, Trip};
use valhalla_client::trace_route::{Manifest, ShapeMatchType, TraceOptions};
use valhalla_client::{Error, Valhalla};

async fn map_match(
    valhalla: &Valhalla,
    locations: impl Iterator<Item = ShapePoint>,
) -> Result<Trip> {
    let trace_options = TraceOptions::builder()
        .search_radius(100.0)
        .gps_accuracy(5.0);
    let manifest: Manifest = Manifest::builder()
        .shape_match(ShapeMatchType::MapSnap)
        .shape(locations)
        .use_timestamps(false)
        .verbose(true)
        .trace_options(trace_options)
        .directions_type(DirectionsType::None)
        .costing(Costing::Auto(AutoCostingOptions::default()));

    valhalla
        .trace_route(manifest)
        .await
        .map_err(|e| anyhow!("Failed to map match: {:?}", e))
}

fn build_node(db: &EveDb, pt: &ShapePoint) -> Node {
    match find_node(db, pt) {
        Some(node) => node,
        None => {
            Node::builder()
                .id(0)
                .latitude(pt.lat)
                .longitude(pt.lon)
                .altitude(0.0)
                .h3_12(lat_lng_to_h3_12(pt.lat, pt.lon) as i64)
                .build()
        }
    }
}

pub fn find_node(db: &EveDb, pt: &ShapePoint) -> Option<Node> {
    let index = lat_lng_to_h3_12(pt.lat, pt.lon);
    let ring = crate::tools::get_ring(index, 1);
    let point  = Point::new(pt.lon, pt.lat);

    match get_ring(db, ring) {
        Ok(nodes) => {
            let nearest = nodes.iter()
                .map(|n| (n, n.distance_to_point(&point)))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            if let Some((node, distance)) = nearest && distance <= 1.0 {
                return Some(*node);
            }
        }
        Err(e) => {
            eprintln!("Failed to get ring: {}", e);
            return None;
        }
    }
    None
}

fn create_tables(cli: &Cli, db: &EveDb) -> bool {
    if cli.verbose {
        println!("Creating the node table")
    }

    if db.create_node_table().is_err() {
        eprintln!("Failed to create node table");
        return false;
    }

    if db.create_edge_table().is_err() {
        eprintln!("Failed to create edge table");
        return false;
    }

    if db.create_node_indexes().is_err() {
        eprintln!("Failed to create node indexes");
        return false;
    }

    if db.create_edge_indexes().is_err() {
        eprintln!("Failed to create edge indexes");
        return false;
    }

    if db.create_traj_node_table().is_err() {
        eprintln!("Failed to create traj_node table");
        return false;
    }

    if db.create_taj_node_indexes().is_err() {
        eprintln!("Failed to create taj_node indexes");
        return false;
    }

    if db.create_traj_edge_table().is_err() {
        eprintln!("Failed to create traj_edge table");
        return false;
    }

    if db.create_taj_edge_indexes().is_err() {
        eprintln!("Failed to create taj_edge indexes");
        return false;
    }

    if db.create_trajectory_error_table().is_err() {
        eprintln!("Failed to create trajectory_error table");
        return false;
    }
    true
}

async fn connect_to_valhalla(
    cli: &Cli,
    valhalla_url: &Url,
) -> std::result::Result<Valhalla, Error> {
    let valhalla = Valhalla::new(valhalla_url.clone());

    if cli.verbose {
        println!("Checking Valhalla instance at {}", valhalla_url);
    }

    valhalla
        .status(valhalla_client::status::Manifest::default())
        .await?;
    Ok(valhalla)
}

fn get_valhalla_url() -> Result<Url> {
    let url =
        std::env::var("VALHALLA_URL").unwrap_or_else(|_| "http://localhost:8002/".to_string());
    Url::parse(&url).map_err(|e| anyhow!("Invalid Valhalla URL '{}': {}", url, e))
}

pub async fn build_nodes(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    let valhalla_url = match get_valhalla_url() {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Invalid Valhalla URL: {}", e);
            return;
        }
    };

    // Check if we can connect to Valhalla
    let valhalla_result = connect_to_valhalla(cli, &valhalla_url).await;

    let valhalla = match valhalla_result {
        Ok(valhalla) => { valhalla }
        Err(e) => {
            eprintln!("Failed to connect to Valhalla: {}", e);
            return;
        }
    };

    if !create_tables(cli, &db) {
        return;
    }

    if cli.verbose {
        println!("Populating the node table")
    }

    let trajectory_ids = db.get_trajectory_ids().unwrap_or(vec![]);
    for trajectory_id in trajectory_ids.iter().progress() {
        if let Ok(way_points) = db.get_way_points(*trajectory_id) {
            let locations = way_points.iter().map(|p: &WayPoint| p.into());

            match map_match(&valhalla, locations).await {
                Ok(trip) => {
                    if let Some(warnings) = trip.warnings {
                        let message = format!(
                            "Map match for trajectory {} has warnings: {:?}",
                            trajectory_id, warnings
                        );
                        eprintln!("{}", message);
                        if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                            eprintln!("Failed to insert match error: {}", e);
                        }
                    } else {
                        let mut nodes: Vec<Node> = trip
                            .legs
                            .iter()
                            .flat_map(|leg| leg.shape.iter())
                            .map(|pt| build_node(&db, pt))
                            .collect();

                        // Insert the nodes into the database
                        if let Err(e) = db.insert_nodes(*trajectory_id, &mut nodes) {
                            let message = format!(
                                "Failed to insert nodes for trajectory {}: {:?}",
                                trajectory_id, e
                            );
                            eprintln!("{}", message);
                            if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                                eprintln!("Failed to insert match error: {}", e);
                            }
                        }

                        // Insert the edges into the database
                        if let Err(e) = db.insert_edges(*trajectory_id, &nodes) {
                            let message = format!(
                                "Failed to insert edges for trajectory {}: {:?}",
                                trajectory_id, e
                            );
                            eprintln!("{}", message);
                        }
                    }
                }
                Err(e) => {
                    let message =
                        format!("Failed to map match trajectory {}: {:?}", trajectory_id, e.to_string());
                    eprintln!("{}", message);
                    if let Err(e) = db.insert_match_error(*trajectory_id, &message) {
                        eprintln!("Failed to insert match error: {}", e);
                    }
                    // return;
                }
            }
        } else {
            eprintln!("Failed to get way points for trajectory {}", trajectory_id);
        }
    }
}

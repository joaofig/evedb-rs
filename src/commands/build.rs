use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use crate::db::evedb::EveDb;
use crate::etl::extract::signals::{get_signal_filenames, insert_signals};
use crate::etl::extract::vehicles::read_vehicles;
use crate::models::trajectory::TrajectoryUpdate;
use crate::tools::lat_lng_to_h3_12;
use async_stream::stream;
use chrono::{DateTime, Duration, TimeZone};
use chrono_tz::America::Detroit;
use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use geo::algorithm::line_measures::{Haversine, Length};
use geo::geometry::LineString;
use indicatif::ProgressIterator;
use rusqlite::params;

fn build_vehicles(cli: &Cli) {
    if cli.verbose {
        println!("Creating the vehicle table")
    }
    let vehicles = read_vehicles(cli);
    let db: EveDb = EveDb::new(&cli.db_path);
    db.create_vehicle_table().unwrap_or(0);
    db.insert_vehicles(vehicles).unwrap_or(());
}

fn build_signals(cli: &Cli) {
    if cli.verbose {
        println!("Creating the signal table")
    }
    let db: EveDb = EveDb::new(&cli.db_path);

    db.create_signal_table().unwrap_or(0);

    let filenames = get_signal_filenames(cli);
    for filename in filenames.iter().progress() {
        // println!("Processing {}", filename);

        let result = insert_signals(cli, &filename);
        if let Err(e) = result {
            eprintln!("Failed to insert signals {}", e);
            break;
        }
    }
    db.create_signal_indexes().unwrap_or(0);
}

fn trajectory_updates(db: &EveDb) -> impl Stream<Item = TrajectoryUpdate> {
    let base_dt: DateTime<chrono_tz::Tz> = Detroit.with_ymd_and_hms(2017, 11, 1, 0, 0, 0).unwrap();
    let trajectory_ids = db.get_trajectory_ids().unwrap_or(vec![]);

    stream! {
        // Now, generate the update trajectory records
        for trajectory_id in trajectory_ids.iter().progress() {
            let trajectory_points = db.get_trajectory_points(*trajectory_id).unwrap_or(vec![]);
            let line_string = LineString::from(
                trajectory_points
                    .iter()
                    .map(|p| (p.longitude, p.latitude))
                    .collect::<Vec<_>>(),
            );
            let length_m = Haversine.length(&line_string);
            let day_num = (trajectory_points[0].day_num as i64) - 1;
            let last = trajectory_points.len() - 1;
            let dt_ini: DateTime<chrono_tz::Tz> =
                base_dt + Duration::days(day_num) + Duration::seconds(trajectory_points[0].time_stamp);
            let dt_end: DateTime<chrono_tz::Tz> = base_dt
                + Duration::days(day_num)
                + Duration::seconds(trajectory_points[last].time_stamp);
            let h3_ini = lat_lng_to_h3_12(
                trajectory_points[0].latitude,
                trajectory_points[0].longitude,
            );
            let h3_end = lat_lng_to_h3_12(
                trajectory_points[last].latitude,
                trajectory_points[last].longitude,
            );

            let traj = TrajectoryUpdate {
                length_m,
                dt_ini: dt_ini.to_rfc3339(),
                dt_end: dt_end.to_rfc3339(),
                duration_s: (trajectory_points[last].time_stamp - trajectory_points[0].time_stamp)
                    as f64
                    / 1000.0,
                h3_12_ini: h3_ini,
                h3_12_end: h3_end,
                traj_id: *trajectory_id,
            };
            yield traj;
        }
    }
}

async fn build_trajectories(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);
    let stream = trajectory_updates(&db);
    let sql: String = String::from(
        "
            UPDATE      trajectory
            SET         length_m = ?
            ,           duration_s = ?
            ,           dt_ini = ?
            ,           dt_end = ?
            ,           h3_12_ini = ?
            ,           h3_12_end = ?
            WHERE       traj_id = ?
            ",
    );

    if cli.verbose {
        println!("Creating the trajectory table")
    }

    pin_mut!(stream);

    let result = db.create_trajectory_table();
    if result.is_err() {
        panic!("Failed to create trajectory table {}", result.err().unwrap());
    }

    if cli.verbose {
        println!("Inserting trajectory records")
    }
    let result = db.insert_trajectories();
    if result.is_err() {
        panic!("Failed to insert trajectory records {}", result.err().unwrap());
    }

    let try_conn = db.connect();
    match try_conn {
        Ok(mut conn) => {
            let try_tx = conn.transaction();

            match try_tx {
                Ok(tx) => {
                    while let Some(update) = stream.next().await {
                        let result = tx.execute(
                            &sql,
                            params![
                                update.length_m,
                                update.dt_ini,
                                update.dt_end,
                                update.duration_s,
                                update.h3_12_ini,
                                update.traj_id,
                            ],
                        );
                        if result.is_err() {
                            tx.rollback().unwrap_or(());
                            panic!(
                                "Failed to insert trajectory record {}",
                                result.err().unwrap()
                            );
                        }
                    }
                    tx.commit().unwrap_or(());
                }
                Err(e) => {
                    panic!("Failed to start transaction {}", e)
                }
            }
        }
        Err(e) => panic!("Failed to connect to the database {}", e),
    }
}

pub async fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    build_vehicles(cli);
    build_signals(cli);
    build_trajectories(cli).await;

    if !args.no_clean {
        clean_data(cli);
    }
}

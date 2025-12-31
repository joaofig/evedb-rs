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
use sqlx::Row;

async fn build_vehicles(cli: &Cli) {
    if cli.verbose {
        println!("Creating the vehicle table")
    }
    let vehicles = read_vehicles(cli);
    let db: EveDb = EveDb::new(&cli.db_path);
    db.create_vehicle_table()
        .await
        .expect("Failed to create vehicle table");
    db.insert_vehicles(vehicles)
        .await
        .expect("Failed to insert vehicle records");
}

async fn build_signals(cli: &Cli) {
    if cli.verbose {
        println!("Creating the signal table")
    }
    let db: EveDb = EveDb::new(&cli.db_path);

    db.create_signal_table()
        .await
        .expect("Failed to create signal table");

    let filenames = get_signal_filenames(cli);
    for filename in filenames.iter().progress() {
        // println!("Processing {}", filename);

        let result = insert_signals(cli, &filename).await;
        if let Err(e) = result {
            eprintln!("Failed to insert signals {}", e);
            break;
        }
    }
    db.create_signal_indexes()
        .await
        .expect("Failed to create signal indexes");
}

async fn get_trajectory_updates(db: &EveDb) -> Vec<TrajectoryUpdate> {
    let base_dt: DateTime<chrono_tz::Tz> = Detroit.with_ymd_and_hms(2017, 11, 1, 0, 0, 0).unwrap();
    let trajectory_ids = db.get_trajectory_ids().await.unwrap_or(vec![]);
    let mut updates: Vec<TrajectoryUpdate> = Vec::with_capacity(trajectory_ids.len());

    // Now, generate the update trajectory records
    for row in trajectory_ids.iter().progress() {
        let trajectory_id: i64 = row.get(0);
        let trajectory_points = db
            .get_trajectory_points(trajectory_id)
            .await
            .unwrap_or(vec![]);

        if trajectory_points.len() < 2 {
            continue;
        }
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

        let update = TrajectoryUpdate {
            length_m,
            dt_ini: dt_ini.to_rfc3339(),
            dt_end: dt_end.to_rfc3339(),
            duration_s: (trajectory_points[last].time_stamp - trajectory_points[0].time_stamp)
                as f64
                / 1000.0,
            h3_12_ini: h3_ini,
            h3_12_end: h3_end,
            traj_id: trajectory_id,
        };
        updates.push(update);
    }
    updates
}

async fn build_trajectories(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    if cli.verbose {
        println!("Creating the trajectory table")
    }

    let result = db.create_trajectory_table().await;
    if result.is_err() {
        panic!(
            "Failed to create trajectory table {}",
            result.err().unwrap()
        );
    }

    if cli.verbose {
        println!("Inserting trajectory records")
    }
    let result = db.insert_trajectories().await;
    if result.is_err() {
        panic!(
            "Failed to insert trajectory records {}",
            result.err().unwrap()
        );
    }

    if cli.verbose {
        println!("Updating trajectory records")
    }
    let updates = get_trajectory_updates(&db).await;
    match db.update_trajectories(&updates).await {
        Ok(_) => {}
        Err(e) => panic!("Failed to update trajectory records {}", e),
    }
}

pub async fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    build_vehicles(cli).await;
    build_signals(cli).await;
    build_trajectories(cli).await;

    if !args.no_clean {
        clean_data(cli);
    }
}

use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::trajectory::TrajectoryUpdate;
use crate::tools::lat_lng_to_h3_12;
use chrono::{DateTime, Duration, TimeZone};
use chrono_tz::America::Detroit;
use geo::line_measures::LengthMeasurable;
use geo::{Haversine, LineString};
use indicatif::ProgressIterator;

async fn get_trajectory_updates(db: &EveDb) -> Vec<TrajectoryUpdate> {
    let base_dt: DateTime<chrono_tz::Tz> = Detroit.with_ymd_and_hms(2017, 11, 1, 0, 0, 0).unwrap();
    let trajectory_ids = db.get_trajectory_ids().await.unwrap_or(vec![]);
    let mut updates: Vec<TrajectoryUpdate> = Vec::with_capacity(trajectory_ids.len());

    // Now, generate the update trajectory records
    for trajectory_id in trajectory_ids.iter().progress() {
        let trajectory_points = db
            .get_trajectory_points(*trajectory_id)
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
        let length_m = line_string.length(&Haversine); // Haversine.length(&line_string);
        let day_num = (trajectory_points[0].day_num as i64) - 1;
        let last = trajectory_points.len() - 1;
        let dt_ini: DateTime<chrono_tz::Tz> = base_dt
            + Duration::days(day_num)
            + Duration::milliseconds(trajectory_points[0].time_stamp);
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
            traj_id: *trajectory_id,
        };
        updates.push(update);
    }
    updates
}

pub(crate) async fn build_trajectories(cli: &Cli) {
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

    let updates = get_trajectory_updates(&db).await;
    if cli.verbose {
        println!("Updating {} trajectory records", updates.len())
    }
    match db.update_trajectories(&updates).await {
        Ok(_) => {}
        Err(e) => panic!("Failed to update trajectory records {}", e),
    }
}

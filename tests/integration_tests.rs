use evedb::cli::{Cli, Commands, BuildCommandArgs};
use evedb::commands::build::build_database;
use evedb::commands::builders::node::build_nodes;
use evedb::db::evedb::EveDb;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use rust_xlsxwriter::{Workbook, XlsxError};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;

fn create_mock_xlsx(path: &Path, vehicle_id: i64) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    
    // Header
    worksheet.write(0, 0, "VehId")?;
    worksheet.write(0, 1, "Vehicle Type")?;
    worksheet.write(0, 2, "Vehicle Class")?;
    worksheet.write(0, 3, "Engine")?;
    worksheet.write(0, 4, "Transmission")?;
    worksheet.write(0, 5, "Drive Wheels")?;
    worksheet.write(0, 6, "Weight")?;
    
    // Data
    worksheet.write(1, 0, vehicle_id)?;
    worksheet.write(1, 1, "ICE")?;
    worksheet.write(1, 2, "Sedan")?;
    worksheet.write(1, 3, "V6")?;
    worksheet.write(1, 4, "Auto")?;
    worksheet.write(1, 5, "FWD")?;
    worksheet.write(1, 6, 1500)?;
    
    workbook.save(path)?;
    Ok(())
}

fn create_mock_zip(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    
    zip.start_file("test_signals.csv", options)?;
    let csv_content = "DayNum,VehId,Trip,Timestamp(ms),Latitude[deg],Longitude[deg],Vehicle Speed[km/h],MAF[g/sec],Engine RPM[RPM],Absolute Load[%],OAT[DegC],Fuel Rate[L/hr],Air Conditioning Power[kW],Air Conditioning Power[Watts],Heater Power[Watts],HV Battery Current[A],HV Battery SOC[%],HV Battery Voltage[V],Short Term Fuel Trim Bank 1[%],Short Term Fuel Trim Bank 2[%],Long Term Fuel Trim Bank 1[%],Long Term Fuel Trim Bank 2[%],Elevation Raw[m],Elevation Smoothed[m],Gradient,Energy Consumption[,Matchted Latitude[deg],Matched Longitude[deg],Match Type,Class of Speed Limit,Speed Limit[km/h],Speed Limit Direction[km/h],Intersection,Bus Stops,Focus Points\n\
                       1,101,100,1000,42.1,-83.1,60.0,10.0,2000,50,20,1.5,0.5,500,0,10,80,350,0,0,0,0,200,200,0,0.1,42.1001,-83.1001,1,1,50,50,0,0,";
    zip.write_all(csv_content.as_bytes())?;
    zip.finish()?;
    Ok(())
}

#[tokio::test]
async fn test_full_build_command() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().join("repo");
    let db_path = tmp_dir.path().join("evedb.db");
    
    // Setup repository structure
    let ved_data_dir = repo_path.join("ved").join("Data");
    let eved_data_dir = repo_path.join("eved").join("data");
    fs::create_dir_all(&ved_data_dir).unwrap();
    fs::create_dir_all(&eved_data_dir).unwrap();
    
    create_mock_xlsx(&ved_data_dir.join("VED_Static_Data_ICE&HEV.xlsx"), 101).unwrap();
    create_mock_xlsx(&ved_data_dir.join("VED_Static_Data_PHEV&EV.xlsx"), 102).unwrap();
    create_mock_zip(&eved_data_dir.join("eVED.zip")).unwrap();
    
    let cli = Cli {
        repo_path: repo_path.to_str().unwrap().to_string(),
        db_path: db_path.to_str().unwrap().to_string(),
        verbose: true,
        command: Commands::Build(BuildCommandArgs {
            no_clone: true,
            no_clean: true,
        }),
    };
    
    if let Commands::Build(args) = &cli.command {
        build_database(&cli, args).await;
    }
    
    // Verification
    assert!(db_path.exists());
    let db = EveDb::new(db_path.to_str().unwrap());
    let conn = db.connect().unwrap();
    
    let vehicle_count: i64 = conn.query_row("SELECT COUNT(*) FROM vehicle", [], |r| r.get::<_, i64>(0)).unwrap();
    // We created 2 XLSX files, each with 1 vehicle (101). But since it's the same ID, maybe it's fine. 
    // Actually, create_vehicle_table drops table, and then we insert.
    // build_vehicles reads both and extends the list.
    assert_eq!(vehicle_count, 2); 
    
    let signal_count: i64 = conn.query_row("SELECT COUNT(*) FROM signal", [], |r| r.get::<_, i64>(0)).unwrap();
    assert_eq!(signal_count, 1);
    
    let trajectory_count: i64 = conn.query_row("SELECT COUNT(*) FROM trajectory", [], |r| r.get::<_, i64>(0)).unwrap();
    assert_eq!(trajectory_count, 1);
}

#[tokio::test]
async fn test_match_command_with_mock_valhalla() {
    let mock_server = MockServer::start().await;
    unsafe { std::env::set_var("VALHALLA_URL", &mock_server.uri()); }

    let tmp_dir = tempdir().unwrap();
    let db_path = tmp_dir.path().join("evedb.db");
    
    // Initialize DB with a trajectory
    let db = EveDb::new(db_path.to_str().unwrap());
    db.create_signal_table().unwrap();
    db.create_trajectory_table().unwrap();
    db.create_node_table().unwrap();
    
    // Insert a signal and trajectory
    let conn = db.connect().unwrap();
    conn.execute("INSERT INTO signal (day_num, vehicle_id, trip_id, time_stamp, latitude, longitude, match_latitude, match_longitude, match_type) VALUES (1, 101, 100, 1000, 42.1, -83.1, 42.1, -83.1, 1)", []).unwrap();
    conn.execute("INSERT INTO signal (day_num, vehicle_id, trip_id, time_stamp, latitude, longitude, match_latitude, match_longitude, match_type) VALUES (1, 101, 100, 2000, 42.2, -83.2, 42.2, -83.2, 1)", []).unwrap();
    db.insert_trajectories().unwrap();
    
    // Mock Valhalla response
    let valhalla_response = json!({
        "trip": {
            "legs": [
                {
                    "maneuvers": [
                        {
                            "begin_shape_index": 0,
                            "end_shape_index": 1,
                            "street_names": ["Test St"]
                        }
                    ],
                    "shape": "42.1,-83.1,42.2,-83.2"
                }
            ],
            "summary": {
                "length": 0.5,
                "time": 60
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/trace_route"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valhalla_response))
        .mount(&mock_server)
        .await;

    let cli = Cli {
        repo_path: "".to_string(),
        db_path: db_path.to_str().unwrap().to_string(),
        verbose: true,
        command: Commands::Match,
    };
    
    build_nodes(&cli).await;
    
    // Verification
    let node_count: i64 = conn.query_row("SELECT COUNT(*) FROM node", [], |r| r.get::<_, i64>(0)).unwrap();
    // valhalla-client might return more or fewer nodes depending on how it parses the response,
    // but we expect at least the nodes from the shape.
    assert!(node_count > 0);

    unsafe { std::env::remove_var("VALHALLA_URL"); }
}

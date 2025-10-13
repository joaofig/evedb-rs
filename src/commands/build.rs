use calamine::{Reader, open_workbook, Xlsx, Data, DataType};
use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use crate::db::evedb::EveDb;
use crate::models::vehicle::Vehicle;

fn no_data_str(data: &Option<String>) -> Option<String> {
    match data {
        Some(data) => {
            let text = data.to_string();
            if text == "NO DATA" {
                None
            } else {
                Some(text)
            }
        },
        None => None,
    }
}

fn read_vehicles_from_excel(path: &str) -> Vec<Vehicle> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    if let Ok(range) = workbook.worksheet_range("Sheet1") {
        let mut vehicles: Vec<Vehicle> = Vec::new();

        for row in range.rows().skip(1) {
            let vehicle: Vehicle = Vehicle {
                vehicle_id: row[0].as_i64().unwrap(),
                vehicle_type: no_data_str(&row[1].as_string()),
                vehicle_class: no_data_str(&row[2].as_string()),
                engine: no_data_str(&row[3].as_string()),
                transmission: no_data_str(&row[4].as_string()),
                drive_wheels: no_data_str(&row[5].as_string()),
                weight: if row[6].as_string().unwrap() == "NO DATA" {
                    None
                } else {
                    row[6].as_i64()
                },
            };
            vehicles.push(vehicle);
        }
        vehicles
    } else {
        vec![]
    }
}

fn read_vehicles(cli: &Cli) -> Vec<Vehicle> {
    let path_ice = format!("{}/ved/Data/VED_Static_Data_ICE&HEV.xlsx", cli.repo_path);
    let path_xev = format!("{}/ved/Data/VED_Static_Data_PHEV&EV.xlsx", cli.repo_path);

    let mut vehicles: Vec<Vehicle> = read_vehicles_from_excel(&path_ice);
    vehicles.extend(read_vehicles_from_excel(&path_xev));
    vehicles
}

fn build_vehicles(cli: &Cli) {
    let vehicles = read_vehicles(cli);
    let db: EveDb = EveDb::new(&cli.db_path);
    db.create_tables();
}

pub fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    build_vehicles(cli);

    if !args.no_clean {
        clean_data(cli);
    }
}
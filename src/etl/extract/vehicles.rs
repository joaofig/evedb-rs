use crate::cli::Cli;
use crate::models::vehicle::Vehicle;
use calamine::{DataType, Reader, Xlsx, open_workbook};
use anyhow::Result;

fn no_data_str(data: &Option<String>) -> Option<String> {
    match data {
        Some(data) => {
            let text = data.to_string();
            if text.starts_with("NO DATA") {
                None
            } else {
                Some(text)
            }
        }
        None => None,
    }
}

fn read_vehicles_from_excel(path: &str) -> Result<Vec<Vehicle>> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let range = workbook.worksheet_range("Sheet1")?;
    let mut vehicles: Vec<Vehicle> = Vec::new();

    for row in range.rows().skip(1) {
        let vehicle: Vehicle = Vehicle {
            vehicle_id: row[0].as_i64()
                .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData,
                                           "Vehicle ID cannot be parsed"))?,
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
    Ok(vehicles)
}

pub fn read_vehicles(cli: &Cli) -> Result<Vec<Vehicle>> {
    let path_ice = format!("{}/ved/Data/VED_Static_Data_ICE&HEV.xlsx", cli.repo_path);
    let path_xev = format!("{}/ved/Data/VED_Static_Data_PHEV&EV.xlsx", cli.repo_path);

    let mut vehicles: Vec<Vehicle> = read_vehicles_from_excel(&path_ice)?;
    vehicles.extend(read_vehicles_from_excel(&path_xev)?);
    Ok(vehicles)
}

use crate::models::vehicle::Vehicle;
use calamine::{DataType, Reader, Xlsx, open_workbook, Range, Data};
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

fn read_vehicles_from_range(range: Range<Data>) -> Result<Vec<Vehicle>> {
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
            weight: if let Some(s) = row[6].as_string() {
                if s == "NO DATA" {
                    None
                } else {
                    row[6].as_i64()
                }
            } else {
                row[6].as_i64()
            },
        };
        vehicles.push(vehicle);
    }
    Ok(vehicles)
}

fn read_vehicles_from_excel(path: &str) -> Result<Vec<Vehicle>> {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let range = workbook.worksheet_range("Sheet1")?;
    read_vehicles_from_range(range)
}

pub fn read_vehicles(cli: &crate::cli::Cli) -> Result<Vec<Vehicle>> {
    let path_ice = format!("{}/ved/Data/VED_Static_Data_ICE&HEV.xlsx", cli.repo_path);
    let path_xev = format!("{}/ved/Data/VED_Static_Data_PHEV&EV.xlsx", cli.repo_path);

    let mut vehicles: Vec<Vehicle> = read_vehicles_from_excel(&path_ice)?;
    vehicles.extend(read_vehicles_from_excel(&path_xev)?);
    Ok(vehicles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Data;

    #[test]
    fn test_no_data_str() {
        assert_eq!(no_data_str(&Some("NO DATA AVAILABLE".to_string())), None);
        assert_eq!(no_data_str(&Some("CAR".to_string())), Some("CAR".to_string()));
        assert_eq!(no_data_str(&None), None);
    }

    #[test]
    fn test_read_vehicles_from_range() {
        let mut range = Range::new((0, 0), (1, 6));
        range.set_value((0, 0), Data::String("VehId".to_string()));
        range.set_value((0, 1), Data::String("Vehicle Type".to_string()));
        range.set_value((0, 2), Data::String("Vehicle Class".to_string()));
        range.set_value((0, 3), Data::String("Engine".to_string()));
        range.set_value((0, 4), Data::String("Transmission".to_string()));
        range.set_value((0, 5), Data::String("Drive Wheels".to_string()));
        range.set_value((0, 6), Data::String("Weight".to_string()));

        range.set_value((1, 0), Data::Int(101));
        range.set_value((1, 1), Data::String("ICE".to_string()));
        range.set_value((1, 2), Data::String("NO DATA".to_string()));
        range.set_value((1, 3), Data::String("V8".to_string()));
        range.set_value((1, 4), Data::String("Automatic".to_string()));
        range.set_value((1, 5), Data::String("FWD".to_string()));
        range.set_value((1, 6), Data::Int(1500));

        let result = read_vehicles_from_range(range).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].vehicle_id, 101);
        assert_eq!(result[0].vehicle_type, Some("ICE".to_string()));
        assert_eq!(result[0].vehicle_class, None);
        assert_eq!(result[0].engine, Some("V8".to_string()));
        assert_eq!(result[0].weight, Some(1500));
    }
}

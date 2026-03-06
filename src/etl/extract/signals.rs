use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::signal::CsvSignal;
use std::fs;
use std::io::Read;
use anyhow::Result;

pub fn get_signal_filenames(cli: &Cli) -> Result<Vec<String>> {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename)?;
    let archive = zip::ZipArchive::new(file)?;
    let filenames: Vec<String> = archive.file_names().map(|f| f.to_string()).collect();
    Ok(filenames)
}

fn get_signal_data(cli: &Cli, data_filename: &str) -> Result<String> {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut zip_file = archive.by_name(data_filename)?;

    let mut csv = String::new();
    zip_file.read_to_string(&mut csv)?;
    Ok(csv)
}

pub fn insert_signals(cli: &crate::cli::Cli, data_file: &str) -> Result<usize> {
    let csv = get_signal_data(cli, data_file)?;
    let db = EveDb::new(&cli.db_path);
    insert_signals_from_csv(&db, &csv)
}

pub fn insert_signals_from_csv(db: &EveDb, csv_content: &str) -> Result<usize> {
    // Replace "nan" and ';' with null
    let mut csv = csv_content.replace("nan", "");
    csv = csv.replace(";", "");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let iterator = reader.deserialize::<CsvSignal>();

    db.insert_signals(iterator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::evedb::EveDb;
    use std::fs;

    #[test]
    fn test_insert_signals_from_csv() {
        let db_path = "test_signals.db";
        if std::path::Path::new(db_path).exists() {
            fs::remove_file(db_path).unwrap();
        }
        let db = EveDb::new(db_path);
        db.create_signal_table().unwrap();

        let csv_content = "DayNum,VehId,Trip,Timestamp(ms),Latitude[deg],Longitude[deg],Vehicle Speed[km/h],MAF[g/sec],Engine RPM[RPM],Absolute Load[%],OAT[DegC],Fuel Rate[L/hr],Air Conditioning Power[kW],Air Conditioning Power[Watts],Heater Power[Watts],HV Battery Current[A],HV Battery SOC[%],HV Battery Voltage[V],Short Term Fuel Trim Bank 1[%],Short Term Fuel Trim Bank 2[%],Long Term Fuel Trim Bank 1[%],Long Term Fuel Trim Bank 2[%],Elevation Raw[m],Elevation Smoothed[m],Gradient,Energy Consumption[,Matchted Latitude[deg],Matched Longitude[deg],Match Type,Class of Speed Limit,Speed Limit[km/h],Speed Limit Direction[km/h],Intersection,Bus Stops,Focus Points
1,10,100,1000,42.1,-83.1,60.0,nan,2000,50,20,1.5,0.5,500,0,10,80,350,0,0,0,0,200,200,0,0.1,42.1001,-83.1001,1,1,50,50,0,0,
";
        let result = insert_signals_from_csv(&db, csv_content).unwrap();
        assert_eq!(result, 1);

        fs::remove_file(db_path).unwrap();
    }
}

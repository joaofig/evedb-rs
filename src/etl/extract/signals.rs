use std::fs;
use std::io::Read;
use crate::models::signal::CsvSignal;
use crate::cli::Cli;
use crate::db::evedb::EveDb;

pub fn get_signal_filenames(cli: &Cli) -> Vec<String> {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut filenames: Vec<String> = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();

        filenames.push(file.name().to_string());
    }
    filenames
}

fn get_signal_data(cli: &Cli, data_filename: &str) -> String {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut zip_file = archive.by_name(data_filename).unwrap();

    let mut csv = String::new();
    zip_file.read_to_string(&mut csv).unwrap();
    csv
}

pub fn insert_signals(cli: &Cli, data_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut signals: Vec<CsvSignal> = Vec::new();
    let mut csv = get_signal_data(cli, data_file);
    let db = EveDb::new(&cli.db_path);
    let mut trip_id: f64 = -1.0;

    // Replace "nan" and ';' with null
    csv = csv.replace("nan", "");
    csv = csv.replace(";", "");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for result in reader.deserialize() {
        let signal: CsvSignal = result.unwrap();

        if signal.trip_id != trip_id && signals.len() > 0 {
            db.insert_signals(&signals)?;
        }

        trip_id = signal.trip_id;
        signals.push(signal);

        if signals.len() > 0 {
            db.insert_signals(&signals)?;
        }
    }
    Ok(())
}
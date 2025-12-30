use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::signal::CsvSignal;
use std::fs;
use std::io::Read;

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

pub async fn insert_signals(cli: &Cli, data_file: &str) -> anyhow::Result<()> {
    let mut csv = get_signal_data(cli, data_file);
    let db = EveDb::new(&cli.db_path);

    // Replace "nan" and ';' with null
    csv = csv.replace("nan", "");
    csv = csv.replace(";", "");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut insert_result: anyhow::Result<()> = Ok(());

    for row in reader.deserialize::<CsvSignal>() {
        if let Ok(signal) = row {
            
        } else {
            eprintln!("Failed to deserialize CSV row")
        }
    }
    Ok(())
}

use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::signal::CsvSignal;
use std::fs;
use std::io::Read;

const BUFFER_SIZE: usize = 500000;

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

fn get_signal_data(cli: &Cli, data_filename: &str) -> anyhow::Result<String> {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut zip_file = archive.by_name(data_filename)?;

    let mut csv = String::new();
    zip_file.read_to_string(&mut csv)?;
    Ok(csv)
}

pub async fn insert_signals(cli: &Cli, data_file: &str) -> anyhow::Result<()> {
    let mut csv = get_signal_data(cli, data_file)?;
    let db = EveDb::new(&cli.db_path);

    // Replace "nan" and ';' with null
    csv = csv.replace("nan", "");
    csv = csv.replace(";", "");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let signals: Vec<CsvSignal> = reader
        .deserialize::<CsvSignal>()
        .map(|r| r.unwrap())
        .collect();

    let result = db.insert_signals(&signals).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

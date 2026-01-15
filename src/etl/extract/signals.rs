use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::models::signal::CsvSignal;
use std::fs;
use std::io::Read;

pub fn get_signal_filenames(cli: &Cli) -> Vec<String> {
    let zip_path = format!("{}/eved/data/eVED.zip", cli.repo_path);
    let filename = std::path::Path::new(&zip_path);
    let file = fs::File::open(&filename).unwrap();
    let archive = zip::ZipArchive::new(file).unwrap();
    let filenames: Vec<String> = archive.file_names().map(|f| f.to_string()).collect();
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

    let iterator = reader
        .deserialize::<CsvSignal>();

    let result = db.insert_signals(iterator).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

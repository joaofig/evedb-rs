use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::etl::extract::signals::{get_signal_filenames, insert_signals};
use indicatif::ProgressIterator;

fn process_signal_file(cli: &Cli, filename: &str) {
    let result = insert_signals(cli, filename);

    if let Err(e) = result {
        eprintln!("Failed to insert signals {}", e);
    };
}

pub fn build_signals(cli: &Cli) -> bool {
    if cli.verbose {
        println!("Creating the signal table")
    }
    let db: EveDb = EveDb::new(&cli.db_path);

    if db.create_signal_table().is_err() {
        eprintln!("Failed to create signal table");
        return false;
    }

    if let Ok(filenames) = get_signal_filenames(cli) {
        for filename in filenames.iter().progress() {
            process_signal_file(cli, filename);
        }
    } else {
        eprintln!("Failed to get signal file names");
        return false;
    }

    if db.create_signal_indexes().is_err() {
        eprintln!("Failed to create signal indexes");
        return false;
    }
    true
}

use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::etl::extract::signals::{get_signal_filenames, insert_signals};
use indicatif::ProgressIterator;

async fn process_signal_file(cli: &Cli, filename: &str) {
    let result = insert_signals(cli, filename).await;

    if let Err(e) = result {
        eprintln!("Failed to insert signals {}", e);
    };
}

pub(crate) async fn build_signals(cli: &Cli) {
    if cli.verbose {
        println!("Creating the signal table")
    }
    let db: EveDb = EveDb::new(&cli.db_path);

    db.create_signal_table()
        .await
        .expect("Failed to create signal table");

    let filenames = get_signal_filenames(cli).unwrap();
    for filename in filenames.iter().progress() {
        process_signal_file(cli, filename).await;
    }

    db.create_signal_indexes()
        .await
        .expect("Failed to create signal indexes");
}

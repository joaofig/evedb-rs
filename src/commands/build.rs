use crate::cli::{BuildCommandArgs, Cli};
use crate::commands::clean::clean_data;
use crate::commands::clone::clone_data;
use crate::db::evedb::EveDb;
use crate::etl::extract::signals::{get_signal_filenames, insert_signals};
use crate::etl::extract::vehicles::read_vehicles;

fn build_vehicles(cli: &Cli) {
    let vehicles = read_vehicles(cli);
    let db: EveDb = EveDb::new(&cli.db_path);
    db.create_vehicle_table().unwrap_or(0);
    db.insert_vehicles(vehicles).unwrap_or(());
}

fn build_signals(cli: &Cli) {
    let db: EveDb = EveDb::new(&cli.db_path);

    db.create_signal_table().unwrap_or(0);

    let filenames = get_signal_filenames(cli);
    for filename in filenames {
        println!("Processing {}", filename);

        let _ = insert_signals(cli, &filename);
    }
}

pub fn build_database(cli: &Cli, args: &BuildCommandArgs) {
    if !args.no_clone {
        clone_data(cli);
    }

    build_vehicles(cli);
    build_signals(cli);

    if !args.no_clean {
        clean_data(cli);
    }
}

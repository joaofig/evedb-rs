use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::etl::extract::vehicles::read_vehicles;

pub(crate) fn build_vehicles(cli: &Cli) {
    if cli.verbose {
        println!("Creating the vehicle table")
    }
    let vehicles = read_vehicles(cli);
    let db: EveDb = EveDb::new(&cli.db_path);
    db.create_vehicle_table()
        .expect("Failed to create vehicle table");
    db.insert_vehicles(vehicles)
        .expect("Failed to insert vehicle records");
}

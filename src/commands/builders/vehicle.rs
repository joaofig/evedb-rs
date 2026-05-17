use crate::cli::Cli;
use crate::db::evedb::EveDb;
use crate::etl::extract::vehicles::read_vehicles;

pub fn build_vehicles(cli: &Cli) -> bool {
    if cli.verbose {
        println!("Creating the vehicle table")
    }
    let vehicles = read_vehicles(cli).expect("Failed to read vehicles");
    let db: EveDb = EveDb::new(&cli.db_path);
    if db.create_vehicle_table().is_err() {
        eprintln!("Failed to create vehicle table");
        return false;
    }
    
    if db.insert_vehicles(vehicles).is_err() {
        eprintln!("Failed to insert vehicle records");
        return false;
    }
    true
}

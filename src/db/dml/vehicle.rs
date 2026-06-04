use crate::db::evedb::EveDb;
use crate::models::vehicle::Vehicle;
use indicatif::ProgressIterator;
use rusqlite::params;

pub fn insert_vehicles(db: &EveDb, vehicles: Vec<Vehicle>) -> anyhow::Result<usize> {
    let sql = "
        INSERT INTO vehicle (
            vehicle_id,
            vehicle_type,
            vehicle_class,
            engine,
            transmission,
            drive_wheels,
            weight) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

    let mut conn = db.connect()?;
    let tx = conn.transaction()?;

    for vehicle in vehicles.iter().progress() {
        let params = params!(
            vehicle.vehicle_id,
            vehicle.vehicle_type,
            vehicle.vehicle_class,
            vehicle.engine,
            vehicle.transmission,
            vehicle.drive_wheels,
            vehicle.weight
        );
        tx.execute(sql, params)?;
    }
    tx.commit()?;
    Ok(vehicles.len())
}

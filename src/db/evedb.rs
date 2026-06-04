use crate::db::api::SqliteDb;
use crate::models::node::Node;
use crate::models::signal::CsvSignal;
use crate::models::trajectory::{TrajectoryPoint, TrajectoryUpdate, WayPoint};
use crate::models::vehicle::Vehicle;
use crate::tools::lat_lng_to_h3_12;
use anyhow::{Result, anyhow};
use csv::DeserializeRecordsIter;
use indicatif::ProgressIterator;
use rusqlite::{Connection, Error, Row, Transaction, params};
use text_block_macros::text_block;
use crate::db::ddl;
use crate::db::dml;

pub struct EveDb {
    pub db: SqliteDb,
}

impl EveDb {
    pub fn new(db_path: &str) -> EveDb {
        let database: SqliteDb = SqliteDb::new(db_path);
        EveDb { db: database }
    }

    pub fn connect(&self) -> Result<Connection> {
        let conn = self.db.connect()?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "cache_size", "10000")?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;
        Ok(conn)
    }

    pub fn create_vehicle_table(&self) -> Result<usize> {
        ddl::vehicle::create_table(self)
    }

    pub fn create_signal_table(&self) -> Result<usize> {
        ddl::signal::create_table(self)
    }

    pub fn insert_signals(
        &self,
        signals: DeserializeRecordsIter<'_, &[u8], CsvSignal>,
    ) -> Result<usize> {
        dml::signal::insert_signals(self, signals)
    }

    pub fn insert_signal(&self, tx: &mut Transaction<'_>, signal: &CsvSignal) -> Result<usize> {
        dml::signal::insert_signal(self, tx, signal)
    }

    pub fn create_signal_indexes(&self) -> Result<usize> {
        ddl::signal::create_indexes(self)
    }

    pub fn insert_vehicles(&self, vehicles: Vec<Vehicle>) -> Result<usize> {
        dml::vehicle::insert_vehicles(self, vehicles)
    }

    pub fn create_trajectory_table(&self) -> Result<usize> {
        ddl::trajectory::create_table(self)
    }

    pub fn insert_trajectories(&self) -> Result<usize> {
        dml::trajectory::insert_trajectories(self)
    }

    pub fn update_trajectories(&self, updates: &[TrajectoryUpdate]) -> Result<()> {
        dml::trajectory::update_trajectories(self, updates)
    }

    pub fn get_trajectory_ids(&self) -> Result<Vec<i64>> {
        dml::trajectory::get_trajectory_ids(self)
    }

    pub fn get_trajectory_points(&self, trajectory_id: i64) -> Result<Vec<TrajectoryPoint>> {
        dml::trajectory::get_trajectory_points(self, trajectory_id)
    }

    pub fn get_way_points(&self, trajectory_id: i64) -> Result<Vec<WayPoint>> {
        dml::trajectory::get_way_points(self, trajectory_id)
    }

    pub fn create_trajectory_indexes(&self) -> Result<usize> {
        ddl::trajectory::create_indexes(self)
    }

    pub fn create_node_table(&self) -> Result<usize> {
        ddl::node::create_table(self)
    }

    pub fn create_node_indexes(&self) -> Result<usize> {
        ddl::node::create_indexes(self)
    }

    pub fn create_edge_table(&self) -> Result<usize> {
        ddl::edge::create_table(self)
    }

    pub fn create_edge_indexes(&self) -> Result<usize> {
        ddl::edge::create_indexes(self)
    }

    pub fn insert_match_error(&self, trajectory_id: i64, match_error: &str) -> Result<usize> {
        dml::node::insert_match_error(self, trajectory_id, match_error)
    }

    pub fn insert_nodes(&self, nodes: impl Iterator<Item = Node>) -> Result<()> {
        dml::node::insert_nodes(self, nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_tables_and_insert() {
        let db_path = "test_evedb.db";
        if std::path::Path::new(db_path).exists() {
            fs::remove_file(db_path).unwrap();
        }
        let db = EveDb::new(db_path);

        db.create_vehicle_table().unwrap();
        db.create_signal_table().unwrap();
        db.create_trajectory_table().unwrap();

        let vehicle = Vehicle {
            vehicle_id: 1,
            vehicle_type: Some("ICE".to_string()),
            vehicle_class: Some("Sedan".to_string()),
            engine: Some("V6".to_string()),
            transmission: Some("Auto".to_string()),
            drive_wheels: Some("FWD".to_string()),
            weight: Some(1500),
        };
        db.insert_vehicles(vec![vehicle]).unwrap();

        // Verify insertion
        let conn = db.connect().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM vehicle", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);

        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_create_trajectory_table_and_indexes() {
        let db_path = "test_traj.db";
        if std::path::Path::new(db_path).exists() {
            fs::remove_file(db_path).unwrap();
        }
        let db = EveDb::new(db_path);

        db.create_trajectory_table().unwrap();
        db.create_trajectory_indexes().unwrap();

        let conn = db.connect().unwrap();
        let table_exists: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='trajectory'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(table_exists, 1);

        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_node_operations() {
        let db_path = "test_nodes.db";
        if std::path::Path::new(db_path).exists() {
            fs::remove_file(db_path).unwrap();
        }
        let db = EveDb::new(db_path);

        db.create_node_table().unwrap();

        let nodes = vec![Node::builder()
            .id(1)
            .latitude(40.0)
            .longitude(-70.0)
            .altitude(0.0)
            .h3_12(12345)
            .build()];
        db.insert_nodes(nodes.into_iter()).unwrap();

        db.create_trajectory_indexes().unwrap();

        let conn = db.connect().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM node", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);

        db.insert_match_error(2, "Test error").unwrap();
        let error_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM node WHERE match_error IS NOT NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(error_count, 1);

        fs::remove_file(db_path).unwrap();
    }
}

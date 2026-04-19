use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub repo_path: String,

    pub db_path: String,
}

impl Config {
    pub fn new(repo_path: String, db_path: String) -> Self {
        Self { repo_path, db_path }
    }

    pub fn save(&self) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write("./evedb.json", json).unwrap();
    }

    pub fn load() -> Self {
        if Path::new("./evedb.json").exists() {
            let json = std::fs::read_to_string("./evedb.json").unwrap();
            serde_json::from_str(&json).unwrap()
        } else {
            Self::new("./data/eved/repo".to_string(), "./data/eved/evedb.db".to_string())
        }
    }
}
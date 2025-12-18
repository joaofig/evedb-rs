use crate::cli::Cli;
use crate::db::evedb::EveDb;
use rusqlite::{Connection, Transaction, params, Params};
use anyhow::Result;
use calamine::CellErrorType::Value;

pub struct Transacted<'a> {
    sql: String,
    db: EveDb,
    conn: Option<Connection>,
    tx: Option<Transaction<'a>>,
}

impl<'a> Transacted<'a> {
    pub fn new(cli: & Cli, sql: &str) -> Self {
        Self {
            sql: sql.to_string(),
            db: EveDb::new(&cli.db_path),
            conn: None,
            tx: None,
        }
    }

    pub fn execute<P: Params>(&mut self, params: P) -> Result<usize> {
        Ok(0)
    }
}
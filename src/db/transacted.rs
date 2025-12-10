use crate::cli::Cli;
use crate::db::evedb::EveDb;
use rusqlite::{Connection, Transaction, params, Params};

pub struct Transacted<'a> {
    sql: String,
    db: EveDb,
    conn: Option<Connection>,
    tx: Option<Transaction<'a>>,
}

impl<'a> Transacted<'a> {
    pub fn new(cli: &Cli, sql: &str) -> Self {
        Transacted {
            sql: sql.to_string(),
            db: EveDb::new(&cli.db_path),
            conn: None,
            tx: None,
        }
    }

    pub fn execute<P: Params>(&mut self, params: P) -> rusqlite::Result<usize> {
        if self.conn.is_none() {
            self.conn = Some(self.db.connect()?);
        }
        // if self.tx.is_none() {
        //     self.tx = Some(self.conn.as_ref().unwrap().transaction()?);
        // }
        Ok(0usize)
    }
}

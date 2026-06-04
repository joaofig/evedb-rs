use crate::db::evedb::EveDb;
use anyhow::anyhow;
use text_block_macros::text_block;

pub fn create_table(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;

    conn.execute("DROP TABLE IF EXISTS signal;", ())?;
    let sql = text_block! {
    "CREATE TABLE signal ("
    "   signal_id          INTEGER PRIMARY KEY,"
    "   day_num            DOUBLE  NOT NULL,"
    "   vehicle_id         INTEGER NOT NULL,"
    "   trip_id            INTEGER NOT NULL,"
    "   time_stamp         INTEGER NOT NULL,"
    "   latitude           DOUBLE  NOT NULL,"
    "   longitude          DOUBLE  NOT NULL,"
    "   speed              DOUBLE,"
    "   maf                DOUBLE,"
    "   rpm                DOUBLE,"
    "   abs_load           DOUBLE,"
    "   oat                DOUBLE,"
    "   fuel_rate          DOUBLE,"
    "   ac_power_kw        DOUBLE,"
    "   ac_power_w         DOUBLE,"
    "   heater_power_w     DOUBLE,"
    "   hv_bat_current     DOUBLE,"
    "   hv_bat_soc         DOUBLE,"
    "   hv_bat_volt        DOUBLE,"
    "   st_ftb_1           DOUBLE,"
    "   st_ftb_2           DOUBLE,"
    "   lt_ftb_1           DOUBLE,"
    "   lt_ftb_2           DOUBLE,"
    "   elevation          DOUBLE,"
    "   elevation_smooth   DOUBLE,"
    "   gradient           DOUBLE,"
    "   energy_consumption DOUBLE,"
    "   match_latitude     DOUBLE  NOT NULL,"
    "   match_longitude    DOUBLE  NOT NULL,"
    "   match_type         INTEGER NOT NULL,"
    "   speed_limit_type   INTEGER,"
    "   speed_limit        TEXT,"
    "   speed_limit_direct INTEGER,"
    "   intersection       INTEGER,"
    "   bus_stop           INTEGER,"
    "   focus_points       TEXT,"
    "   h3_12              INTEGER);"};
    conn.execute(sql, ())
        .map_err(|e| anyhow!("Failed to create signal table: {:?}", e))
}

pub fn create_indexes(db: &EveDb) -> anyhow::Result<usize> {
    let conn = db.connect()?;
    conn.execute(
        "
        CREATE INDEX IF NOT EXISTS signal_vehicle_trip_idx ON signal (
            vehicle_id ASC,
            trip_id ASC,
            time_stamp ASC
        );",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS signal_h3_idx ON signal (h3_12);",
        (),
    )
    .map_err(|e| anyhow!("Failed to create signal indexes: {:?}", e))
}

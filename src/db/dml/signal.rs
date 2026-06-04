use anyhow::anyhow;
use csv::DeserializeRecordsIter;
use rusqlite::{params, Transaction};
use text_block_macros::text_block;
use crate::db::evedb::EveDb;
use crate::models::signal::CsvSignal;
use crate::tools::lat_lng_to_h3_12;

pub fn insert_signal(db: &EveDb, tx: &mut Transaction<'_>, signal: &CsvSignal) -> anyhow::Result<usize> {
    let sql = text_block! {
            "INSERT INTO signal ("
            "   day_num, vehicle_id, trip_id, time_stamp, latitude, "
            "   longitude, speed, maf, rpm, abs_load, oat, fuel_rate, "
            "   ac_power_kw, ac_power_w, heater_power_w, hv_bat_current, "
            "   hv_bat_soc, hv_bat_volt, st_ftb_1, st_ftb_2, lt_ftb_1, "
            "   lt_ftb_2, elevation, elevation_smooth, gradient, "
            "   energy_consumption, match_latitude, match_longitude, "
            "   match_type, speed_limit_type, speed_limit, speed_limit_direct, "
            "   intersection, bus_stop, focus_points, h3_12) "
            "VALUES "
            "($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,"
            " $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34,"
            " $35, $36);"
        };

    let index: i64 = lat_lng_to_h3_12(signal.match_latitude, signal.match_longitude) as i64;
    let params = params!(
            signal.day_num as i64,
            signal.vehicle_id as i64,
            signal.trip_id as i64,
            signal.time_stamp as i64,
            signal.latitude,
            signal.longitude,
            signal.speed,
            signal.maf,
            signal.rpm,
            signal.abs_load,
            signal.oat,
            signal.fuel_rate,
            signal.ac_power_kw,
            signal.ac_power_w,
            signal.heater_power_w,
            signal.hv_bat_current,
            signal.hv_bat_soc,
            signal.hv_bat_volt,
            signal.st_ftb_1,
            signal.st_ftb_2,
            signal.lt_ftb_1,
            signal.lt_ftb_2,
            signal.elevation,
            signal.elevation_smooth,
            signal.gradient,
            signal.energy_consumption,
            signal.match_latitude,
            signal.match_longitude,
            signal.match_type,
            signal.speed_limit_type,
            signal.speed_limit,
            signal.speed_limit_direct,
            signal.intersection,
            signal.bus_stop,
            signal.focus_points.clone(),
            index,
        );
    tx.execute(sql, params)
        .map_err(|e| anyhow!("Failed to insert signal: {:?}", e))
}

pub fn insert_signals(
    db: &EveDb,
    signals: DeserializeRecordsIter<'_, &[u8], CsvSignal>,
) -> anyhow::Result<usize> {
    let mut conn = db.connect()?;
    let mut counter: usize = 0;

    let mut tx = conn.transaction()?;
    for signal in signals.flatten() {
        insert_signal(db, &mut tx, &signal)?;
        counter += 1;
    }
    tx.commit()?;
    Ok(counter)
}
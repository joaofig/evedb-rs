pub struct TrajectoryPoint {
    pub signal_id: i64,
    pub vehicle_id: i64,
    pub day_num: f64,
    pub time_stamp: i64,
    pub latitude: f64,
    pub longitude: f64,
}

pub struct TrajectoryUpdate {
    pub traj_id: i64,
    pub length_m: f64,
    pub dt_ini: String,
    pub dt_end: String,
    pub duration_s: f64,
    pub h3_12_ini: u64,
    pub h3_12_end: u64,
}

pub struct WayPoint {
    pub time: i64,
    pub latitude: f64,
    pub longitude: f64,
}

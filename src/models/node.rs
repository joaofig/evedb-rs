

pub struct Node {
    pub trajectory_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub h3_12: i64,
    pub match_error: Option<String>,
}

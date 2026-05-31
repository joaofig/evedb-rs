use bon::Builder;

#[derive(Builder)]
pub struct Segment {
    pub segment_id: i64,
    pub lat_ini: f64,
    pub lon_ini: f64,
    pub lat_end: f64,
    pub lon_end: f64,
    pub length_m: f64,
}
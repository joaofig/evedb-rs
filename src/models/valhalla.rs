use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum ShapeMatch {
    EdgeWalk,
    MapSnap,
    WalkOrSnap,
}

#[derive(Debug, Serialize)]
pub enum CostingModel {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "bicycle")]
    Bicycle,
    #[serde(rename = "bus")]
    Bus,
    #[serde(rename = "bikeshare")]
    Bikeshare,
    #[serde(rename = "truck")]
    Truck,
    #[serde(rename = "hov")]
    Hov,
    #[serde(rename = "taxi")]
    Taxi,
    #[serde(rename = "moto_scooter")]
    MotorScooter,
    #[serde(rename = "motorcycle")]
    Motorcycle,
    #[serde(rename = "multimodal")]
    Multimodal,
    #[serde(rename = "pedestrian")]
    Pedestrian,
}

pub enum CostingOption {
    Auto,
}

#[derive(Debug, Serialize)]
pub enum OutputFormat {
    Json,
    Gpx,
    Osrm,
    Pbf,
}

#[derive(Debug, Serialize)]
pub struct TraceRoute {
    use_timestamps: bool,
    shape_match: String,

    units: Option<String>,
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraceAttributes {
    status: i32,
    status_message: String,
    units: String,
    language: String,
}
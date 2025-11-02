use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Signal {
    #[serde(default)]
    pub signal_id: i64,
    #[serde(rename = "VehId")]
    pub vehicle_id: f64,
    #[serde(rename = "Trip")]
    pub trip_id: f64,
    #[serde(rename = "Timestamp(ms)")]
    pub time_stamp: f64,
    #[serde(rename = "Latitude[deg]")]
    pub latitude: f64,
    #[serde(rename = "Longitude[deg]")]
    pub longitude: f64,
    #[serde(rename = "Vehicle Speed[km/h]")]
    pub speed: Option<f64>,
    #[serde(rename = "MAF[g/sec]")]
    pub maf: Option<f64>,
    #[serde(rename = "Engine RPM[RPM]")]
    pub rpm: Option<f64>,
    #[serde(rename = "Absolute Load[%]")]
    pub abs_load: Option<f64>,
    #[serde(rename = "OAT[DegC]")]
    pub oat: Option<f64>,
    #[serde(rename = "Fuel Rate[L/hr]")]
    pub fuel_rate: Option<f64>,
    #[serde(rename = "Air Conditioning Power[kW]")]
    pub ac_power_kw: Option<f64>,
    #[serde(rename = "Air Conditioning Power[Watts]")]
    pub ac_power_w: Option<f64>,
    #[serde(rename = "Heater Power[Watts]")]
    pub heater_power_w: Option<f64>,
    #[serde(rename = "HV Battery Current[A]")]
    pub hv_bat_current: Option<f64>,
    #[serde(rename = "HV Battery SOC[%]")]
    pub hv_bat_soc: Option<f64>,
    #[serde(rename = "HV Battery Voltage[V]")]
    pub hv_bat_volt: Option<f64>,
    #[serde(rename = "Short Term Fuel Trim Bank 1[%]")]
    pub st_ftb_1: Option<f64>,
    #[serde(rename = "Short Term Fuel Trim Bank 2[%]")]
    pub st_ftb_2: Option<f64>,
    #[serde(rename = "Long Term Fuel Trim Bank 1[%]")]
    pub lt_ftb_1: Option<f64>,
    #[serde(rename = "Long Term Fuel Trim Bank 2[%]")]
    pub lt_ftb_2: Option<f64>,
    #[serde(rename = "Elevation Raw[m]")]
    pub elevation: Option<f64>,
    #[serde(rename = "Elevation Smoothed[m]")]
    pub elevation_smooth: Option<f64>,
    #[serde(rename = "Gradient")]
    pub gradient: Option<f64>,
    #[serde(rename = "Energy Consumption[")]
    pub energy_consumption: Option<f64>,
    #[serde(rename = "Matchted Latitude[deg]")]
    pub match_latitude: f64,
    #[serde(rename = "Matched Longitude[deg]")]
    pub match_longitude: f64,
    #[serde(rename = "Match Type")]
    pub match_type: f64,
    #[serde(rename = "Class of Speed Limit")]
    pub speed_limit_type: Option<f64>,
    #[serde(rename = "Speed Limit[km/h]")]
    pub speed_limit: Option<String>,
    #[serde(rename = "Speed Limit Direction[km/h]")]
    pub speed_limit_direct: Option<f64>,
    #[serde(rename = "Intersection")]
    pub intersection: Option<f64>,
    #[serde(rename = "Bus Stops")]
    pub bus_stop: Option<f64>,
    #[serde(rename = "Focus Points")]
    pub focus_points: Option<String>,
    #[serde(default)]
    pub h3_12: Option<i64>,
}

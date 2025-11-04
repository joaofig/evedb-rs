use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvSignal {
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
}

impl CsvSignal {
    pub fn to_tuple(&self) -> (i64, i64, i64, f64, f64, Option<f64>, Option<f64>, Option<f64>,
                               Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>,
                               Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>,
                               Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>,
                               Option<f64>, Option<f64>, f64, f64, f64, Option<f64>, Option<String>,
                               Option<i64>, Option<i64>, Option<i64>, Option<String>) {
        (
            self.vehicle_id as i64,
            self.trip_id as i64,
            self.time_stamp as i64,
            self.latitude,
            self.longitude,
            self.speed,
            self.maf,
            self.rpm,
            self.abs_load,
            self.oat,
            self.fuel_rate,
            self.ac_power_kw,
            self.ac_power_w,
            self.heater_power_w,
            self.hv_bat_current,
            self.hv_bat_soc,
            self.hv_bat_volt,
            self.st_ftb_1,
            self.st_ftb_2,
            self.lt_ftb_1,
            self.lt_ftb_2,
            self.elevation,
            self.elevation_smooth,
            self.gradient,
            self.energy_consumption,
            self.match_latitude,
            self.match_longitude,
            self.match_type,
            self.speed_limit_type,
            self.speed_limit.clone(),
            self.speed_limit_direct.map(|f| f as i64),
            self.intersection.map(|f| f as i64),
            self.bus_stop.map(|f| f as i64),
            self.focus_points.clone(),
        )
    }
}

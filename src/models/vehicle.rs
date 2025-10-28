pub struct Vehicle {
    pub vehicle_id: i64,
    pub vehicle_type: Option<String>,
    pub vehicle_class: Option<String>,
    pub engine: Option<String>,
    pub transmission: Option<String>,
    pub drive_wheels: Option<String>,
    pub weight: Option<i64>,
}

impl Vehicle {
    pub fn to_tuple(
        &self,
    ) -> (
        i64,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<i64>,
    ) {
        (
            self.vehicle_id,
            self.vehicle_type.clone(),
            self.vehicle_class.clone(),
            self.engine.clone(),
            self.transmission.clone(),
            self.drive_wheels.clone(),
            self.weight,
        )
    }
}

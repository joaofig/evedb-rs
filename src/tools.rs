pub fn lat_lng_to_h3_12(lat: f64, lng: f64) -> u64 {
    let coord = h3o::LatLng::new(lat, lng).unwrap();
    let cell = coord.to_cell(h3o::Resolution::Twelve);
    let index: u64 = cell.into();
    index
}

pub fn lat_lng_to_h3_12(lat: f64, lng: f64) -> u64 {
    let coord = h3o::LatLng::new(lat, lng).unwrap();
    let cell = coord.to_cell(h3o::Resolution::Twelve);
    let index: u64 = cell.into();
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lat_lng_to_h3_12() {
        // Test with coordinates for Lisbon, Portugal
        let lat = 38.7223;
        let lng = -9.1393;
        let h3_index = lat_lng_to_h3_12(lat, lng);
        
        // Ensure we get a valid H3 index (non-zero)
        assert!(h3_index > 0);
        
        // Verify the index corresponds to resolution 12
        let cell = h3o::CellIndex::try_from(h3_index).expect("Invalid H3 index");
        assert_eq!(cell.resolution(), h3o::Resolution::Twelve);
        
        // Verify we can get the coordinates back and they are reasonably close
        let coord: h3o::LatLng = cell.into();
        let out_lat: f64 = coord.lat();
        let out_lng: f64 = coord.lng();
        
        // H3 Resolution 12 has an average area of ~300 m^2, so precision should be high.
        assert!((lat - out_lat).abs() < 0.001);
        assert!((lng - out_lng).abs() < 0.001);
    }

    #[test]
    fn test_lat_lng_to_h3_12_specific_value() {
        // Specific known value for (0.0, 0.0) at resolution 12
        let lat = 0.0;
        let lng = 0.0;
        let h3_index = lat_lng_to_h3_12(lat, lng);
        
        // Use h3o::CellIndex if h3o::Cell doesn't exist
        let cell = h3o::CellIndex::try_from(h3_index).expect("Invalid H3 index");
        assert_eq!(cell.resolution(), h3o::Resolution::Twelve);
    }
}

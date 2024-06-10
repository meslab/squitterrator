pub fn altitude_gnss(message: &[u32]) -> Option<u32> {
    if let Some(altitude) = crate::adsb::range_value(message, 49, 60) {
        Some(altitude)
    } else {
        None
    }
}

pub fn altitude_gnss(message: &[u32]) -> Option<u32> {
    if let Some((_, altitude)) = crate::adsb::flag_and_range_value(message, 1, 49, 60) {
        Some(altitude)
    } else {
        None
    }
}

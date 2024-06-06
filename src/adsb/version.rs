pub fn version(message: &[u32]) -> Option<u32> {
    if let Some(value) = crate::adsb::flag_and_range_value(message, 1, 73, 75) {
        Some(value.1)
    } else {
        None
    }
}

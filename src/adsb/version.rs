pub fn version(message: &[u32]) -> Option<u32> {
    crate::adsb::flag_and_range_value(message, 1, 73, 75).map(|value| value.1)
}

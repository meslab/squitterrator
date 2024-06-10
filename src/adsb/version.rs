pub fn version(message: &[u32]) -> Option<u32> {
    crate::adsb::range_value(message, 73, 75).map(|value| value)
}

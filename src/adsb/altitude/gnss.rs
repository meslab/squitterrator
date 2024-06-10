pub fn altitude_gnss(message: &[u32]) -> Option<u32> {
    crate::adsb::range_value(message, 49, 60)
}

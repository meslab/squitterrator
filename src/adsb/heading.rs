pub fn heading(message: &[u32]) -> Option<u32> {
    if let Some((_, value)) = crate::adsb::flag_and_range_value(message, 1, 47, 56) {
        Some((value * 360) >> 10)
    } else {
        None
    }
}

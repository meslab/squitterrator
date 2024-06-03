pub fn heading(message: &[u32]) -> Option<f64> {
    if let Some((_, value)) = crate::adsb::flag_and_range_value(message, 1, 47, 56) {
        Some(((value * 360) >> 10) as f64)
    } else {
        None
    }
}

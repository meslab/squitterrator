pub fn ground_movement(message: &[u32]) -> Option<f64> {
    let value = ((message[9] & 7) << 4 | message[10]) as f64;
    match value {
        1.0 => Some(0.0),
        2.0..=8.0 => Some(value / 8.0),
        9.0..=12.0 => Some(value / 4.0),
        13.0..=38.0 => Some(value / 2.0),
        39.0..=93.0 => Some(value),
        94.0..=108.0 => Some(value * 2.0),
        109.0..=123.0 => Some(value * 5.0),
        124.0 => Some(175.0),
        _ => None,
    }
}

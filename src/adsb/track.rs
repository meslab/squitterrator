pub fn track(message: &[u32]) -> Option<f64> {
    let track = (((message[11] & 0x8) << 1) | message[12]) as f64 * 2.8125;
    if (message[11] & 0x8) >> 3 != 0 {
        Some(360.0 - track)
    } else {
        Some(track)
    }
}

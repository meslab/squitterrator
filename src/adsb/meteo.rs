pub fn temperature(message: &[u32]) -> Option<f64> {
    match message[13] & 0x1 {
        0 => Some(
            (((message[14] & 0xF) << 6) | ((message[15] & 0xF) << 2) | (message[16] & 0b1100) >> 2)
                as f64
                / 4.0,
        ),
        _ => Some(
            -((((message[14] & 0xF) << 6)
                | ((message[15] & 0xF) << 2)
                | (message[16] & 0b1100) >> 2) as f64
                / 4.0),
        ),
    }
}

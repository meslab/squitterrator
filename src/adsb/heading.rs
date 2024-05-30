pub fn heading(message: &[u32]) -> Option<f64> {
    Some(
        ((((message[11] & 3) << 8 | (message[12] & 0xF) << 4 | (message[13] & 0xF)) * 360) >> 10)
            as f64,
    )
}

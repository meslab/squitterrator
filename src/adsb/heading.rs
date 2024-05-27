use std::f32::consts::PI;

pub fn heading(message: &[u32]) -> Option<f64> {
    Some(
        ((message[11] & 3) << 8 | (message[12] & 0xF) << 4 | (message[13] & 0xF)) as f64 * 360.0
            / 1024.0
            / PI as f64,
    )
}

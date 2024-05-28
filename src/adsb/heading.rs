use std::f64::consts::PI;

pub fn heading(message: &[u32]) -> Option<f64> {
    let sp_west = ((message[11] & 3) << 8 | (message[12] & 0xF) << 4 | (message[13] & 0xF)) as f64;
    Some(sp_west * 360.0 / 1024.0 / PI)
}

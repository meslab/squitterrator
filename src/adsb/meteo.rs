use super::flag_and_range_value;

pub fn temperature(message: &[u32]) -> Option<f64> {
    if let Some((_, sign, temp, _, _, _, _)) = flag_and_range_value(message, 56, 57, 66) {
        match sign {
            0 => Some(temp as f64 / 4.0),
            _ => Some(-(temp as f64 / 4.0)),
        }
    } else {
        None
    }
}

pub fn wind_speed(message: &[u32]) -> Option<u32> {
    if let Some((_, status, speed, _, _, _, _)) = flag_and_range_value(message, 37, 38, 46) {
        match status {
            1 => Some(speed),
            _ => None,
        }
    } else {
        None
    }
}

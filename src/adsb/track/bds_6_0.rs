pub fn magnetic_heading_6_0(message: &[u32]) -> Option<u32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 33, 34, 44) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 34, 35, 44)
                {
                    let heading = (value * 90) >> 9;
                    match sign {
                        0 => Some(heading),
                        _ => Some(heading + 180),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

pub fn indicated_airspeed_6_0(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 45, 46, 55) {
        match status {
            0 => None,
            _ => Some(value),
        }
    } else {
        None
    }
}

pub fn mach_number_6_0(message: &[u32]) -> Option<f64> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 56, 57, 66) {
        match status {
            0 => None,
            _ => Some(value as f64 * 0.004),
        }
    } else {
        None
    }
}

pub fn barometric_altitude_rate_6_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 67, 68, 77) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 68, 69, 77)
                {
                    Some(barometric_altitude_rate(sign, value))
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn barometric_altitude_rate(sign: u32, value: u32) -> i32 {
    let rate = (value as i32) << 5;
    match sign {
        0 => rate,
        _ => rate - 16384,
    }
}

pub fn internal_vertical_velocity_6_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 78, 79, 88) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 79, 80, 88)
                {
                    let rate = (value << 5) as i32;
                    match sign {
                        0 => Some(rate),
                        _ => Some(-rate),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barometric_altitude_rate() {
        assert_eq!(barometric_altitude_rate(0, 0b1111_1111_1), 16352);
        assert_eq!(barometric_altitude_rate(1, 0b1111_1111_1), -32);
        assert_eq!(barometric_altitude_rate(0, 0b0000_0000_1), 32);
        assert_eq!(barometric_altitude_rate(1, 0b0000_0000_1), -16352);
    }
}

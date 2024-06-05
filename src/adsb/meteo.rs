use crate::adsb::flag_and_range_value;

pub fn temperature_4_4(message: &[u32]) -> Option<f64> {
    if let Some((sign, temp)) = flag_and_range_value(message, 56, 57, 66) {
        let temp = temp as f64;
        match sign {
            0 => Some(temp * 0.25),
            _ => Some(-temp * 0.25),
        }
    } else {
        None
    }
}

fn wind_speed(message: &[u32]) -> Option<u32> {
    if let Some((status, speed)) = flag_and_range_value(message, 37, 38, 46) {
        match status {
            1 => Some(speed),
            _ => None,
        }
    } else {
        None
    }
}

fn wind_direction(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = flag_and_range_value(message, 37, 47, 55) {
        let direction = (value * 180) >> 8;
        match status {
            1 => Some(direction),
            _ => None,
        }
    } else {
        None
    }
}

pub fn wind_4_4(message: &[u32]) -> Option<(u32, u32)> {
    if let Some(wind_speed) = wind_speed(message) {
        wind_direction(message).map(|wind_direction| (wind_speed, wind_direction))
    } else {
        None
    }
}

pub fn turbulence_4_4(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 79, 80, 81) {
        match status {
            1 => Some(value),
            _ => None,
        }
    } else {
        None
    }
}

pub fn humidity_4_4(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 82, 83, 88) {
        match status {
            1 => Some((value * 100) >> 6),
            _ => None,
        }
    } else {
        None
    }
}

pub fn pressure_4_4(message: &[u32]) -> Option<u32> {
    if let Some((status, pressure)) = crate::adsb::flag_and_range_value(message, 67, 68, 78) {
        match status {
            1 => Some(pressure),
            _ => None,
        }
    } else {
        None
    }
}

pub fn temperature_4_5(message: &[u32]) -> Option<f64> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 48, 49, 58) {
        match status {
            1 => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 49, 50, 58)
                {
                    match sign {
                        1 => Some(-(value as f64 * 0.25)),
                        _ => Some(value as f64 * 0.25),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shift() {
        assert_eq!(2048 / 2, 2048 >> 1);
        assert_eq!(2048 / 4, 2048 >> 2);
        assert_eq!(2048 / 8, 2048 >> 3);
        assert_eq!(2048 / 16, 2048 >> 4);
        assert_eq!(2048 / 32, 2048 >> 5);
        assert_eq!(2048 / 64, 2048 >> 6);
        assert_eq!(2048 / 128, 2048 >> 7);
        assert_eq!(2048 / 256, 2048 >> 8);
        assert_eq!(2048 / 512, 2048 >> 9);
        assert_eq!(2048 / 1024, 2048 >> 10);
    }

    #[test]
    fn test_power() {
        assert_eq!(2_i32.pow(9), 1 << 9);
        for x in 0..=539 {
            assert_eq!(-x, !x + 1);
        }
        assert_eq!(2_i32.pow(9), 1 << 9);
    }
}

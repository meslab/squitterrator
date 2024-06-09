pub(crate) fn roll_angle_5_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 33, 34, 43) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 34, 35, 43)
                {
                    Some(roll_angle(sign, value))
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn roll_angle(sign: u32, value: u32) -> i32 {
    let value = value as i32 * 45 / 256;
    match sign {
        0 => value,
        _ => value - 90,
    }
}

pub(crate) fn track_angle_5_0(message: &[u32]) -> Option<u32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 44, 45, 55) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 45, 46, 55)
                {
                    Some(track_angle(sign, value))
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn track_angle(sign: u32, value: u32) -> u32 {
    let angle = (value * 90) >> 9;
    match sign {
        0 => angle,
        _ => angle + 180,
    }
}

pub(crate) fn track_angle_rate_5_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 67, 68, 77) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 68, 69, 77)
                {
                    Some(track_angle_rate(sign, value))
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn track_angle_rate(sign: u32, value: u32) -> i32 {
    let angle = ((value << 3) >> 8) as i32;
    match sign {
        0 => angle,
        _ => angle - 16,
    }
}

pub(crate) fn ground_speed_5_0(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 56, 57, 66) {
        match status {
            0 => None,
            _ => Some(value << 1),
        }
    } else {
        None
    }
}

pub(crate) fn true_airspeed_5_0(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 78, 79, 88) {
        match status {
            0 => None,
            _ => Some(value << 1),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_angle() {
        assert_eq!(roll_angle(0, 0b111111111), 89);
        assert_eq!(roll_angle(0, 0b000000001), 0);
        assert_eq!(roll_angle(1, 0b000000001), -90);
        assert_eq!(roll_angle(1, 0b111111111), -1);
    }
}

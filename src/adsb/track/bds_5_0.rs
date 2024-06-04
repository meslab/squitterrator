pub fn roll_angle_5_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 33, 34, 43) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 34, 35, 43)
                {
                    let angle = ((value * 45) >> 8) as i32;
                    match sign {
                        0 => Some(angle),
                        _ => Some(-angle),
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

pub fn track_angle_5_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 44, 45, 55) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 45, 46, 55)
                {
                    let angle = ((value * 90) >> 9) as i32;
                    match sign {
                        0 => Some(angle),
                        _ => Some(-angle),
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

pub fn track_angle_rate_5_0(message: &[u32]) -> Option<i32> {
    if let Some((status, _)) = crate::adsb::flag_and_range_value(message, 67, 68, 77) {
        match status {
            0 => None,
            _ => {
                if let Some((sign, value)) = crate::adsb::flag_and_range_value(message, 68, 69, 77)
                {
                    let angle = ((value << 3) >> 8) as i32;
                    match sign {
                        0 => Some(angle),
                        _ => Some(-angle),
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

pub fn ground_speed_5_0(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 56, 57, 66) {
        match status {
            0 => None,
            _ => Some(value << 1),
        }
    } else {
        None
    }
}

pub fn true_airspeed_5_0(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 78, 79, 88) {
        match status {
            0 => None,
            _ => Some(value << 1),
        }
    } else {
        None
    }
}

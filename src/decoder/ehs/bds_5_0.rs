use crate::decoder::{flag_and_range_value, status_flag_and_range_value};

pub(crate) fn roll_angle_5_0(message: &[u32]) -> Option<i32> {
    status_flag_and_range_value(message, 33, 34, 35, 43)
        .filter(|&f| f.0 == 1)
        .map(|(_, sign, value)| roll_angle(sign, value))
}

fn roll_angle(sign: u32, value: u32) -> i32 {
    let value = value as i32 * 45 / 256;
    match sign {
        0 => value,
        _ => value - 90,
    }
}

pub(crate) fn track_angle_5_0(message: &[u32]) -> Option<u32> {
    status_flag_and_range_value(message, 44, 45, 46, 55)
        .filter(|&f| f.0 == 1)
        .map(|(_, sign, value)| track_angle(sign, value))
}

fn track_angle(sign: u32, value: u32) -> u32 {
    let angle = (value * 90) >> 9;
    match sign {
        0 => angle,
        _ => angle + 180,
    }
}

pub(crate) fn track_angle_rate_5_0(message: &[u32]) -> Option<i32> {
    status_flag_and_range_value(message, 67, 68, 69, 77)
        .filter(|&f| f.0 == 1)
        .map(|(_, sign, value)| track_angle_rate(sign, value))
}

fn track_angle_rate(sign: u32, value: u32) -> i32 {
    let angle = ((value << 3) >> 8) as i32;
    match sign {
        0 => angle,
        _ => angle - 16,
    }
}

pub(crate) fn ground_speed_5_0(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 56, 57, 66)
        .filter(|&f| f.0 == 1)
        .map(|v| v.1 << 1)
}

pub(crate) fn true_airspeed_5_0(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 78, 79, 88)
        .filter(|&f| f.0 == 1)
        .map(|v| v.1 << 1)
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

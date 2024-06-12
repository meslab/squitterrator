use crate::decoder::{flag_and_range_value, status_flag_and_range_value};

/// Calculates the magnetic heading based on the given ADS-B message.
/// Returns `None` if the status is 0, otherwise returns the calculated magnetic heading.
pub(crate) fn magnetic_heading_6_0(message: &[u32]) -> Option<u32> {
    status_flag_and_range_value(message, 33, 34, 35, 44)
        .filter(|&f| f.0 == 1)
        .map(|(_, sign, value)| magnetic_heading(sign, value))
}

/// Calculates the magnetic heading based on the given sign and value.
fn magnetic_heading(sign: u32, value: u32) -> u32 {
    let heading = (value * 90) >> 9;
    match sign {
        0 => heading,
        _ => heading + 180,
    }
}

/// Calculates the indicated airspeed based on the given ADS-B message.
/// Returns `None` if the status is 0, otherwise returns the indicated airspeed.
pub(crate) fn indicated_airspeed_6_0(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 45, 46, 55)
        .filter(|&f| f.0 == 1 && f.1 != 0)
        .map(|v| v.1)
}

/// Calculates the Mach number based on the given ADS-B message.
/// Returns `None` if the status is 0, otherwise returns the Mach number.
pub(crate) fn mach_number_6_0(message: &[u32]) -> Option<f64> {
    flag_and_range_value(message, 56, 57, 66)
        .filter(|&f| f.0 == 1 && f.1 != 0)
        .map(|v| v.1 as f64 * 0.004)
}

/// Calculates the barometric altitude rate based on the given ADS-B message.
/// Returns `None` if the status is 0, otherwise returns the barometric altitude rate.
pub(crate) fn barometric_altitude_rate_6_0(message: &[u32]) -> Option<i32> {
    status_flag_and_range_value(message, 67, 68, 69, 77)
        .filter(|&f| f.0 == 1 && f.2 != 0)
        .map(|(_, sign, value)| barometric_altitude_rate(sign, value))
}

/// Calculates the barometric altitude rate based on the given sign and value.
fn barometric_altitude_rate(sign: u32, value: u32) -> i32 {
    let rate = (value as i32) << 5;
    match sign {
        0 => rate,
        _ => rate - 16384,
    }
}

/// Calculates the internal vertical velocity based on the given ADS-B message.
/// Returns `None` if the status is 0, otherwise returns the internal vertical velocity.
pub(crate) fn internal_vertical_velocity_6_0(message: &[u32]) -> Option<i32> {
    status_flag_and_range_value(message, 78, 79, 80, 88)
        .filter(|&f| f.0 == 1 && f.2 != 0)
        .map(|(_, sign, value)| internal_vertical_velocity(sign, value))
}

/// Calculates the internal vertical velocity based on the given sign and value.
fn internal_vertical_velocity(sign: u32, value: u32) -> i32 {
    let rate = (value << 5) as i32;
    match sign {
        0 => rate,
        _ => rate - 16384,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barometric_altitude_rate() {
        assert_eq!(barometric_altitude_rate(0, 0b1_1111_1111), 16352);
        assert_eq!(barometric_altitude_rate(1, 0b1_1111_1111), -32);
        assert_eq!(barometric_altitude_rate(0, 0b0_0000_0001), 32);
        assert_eq!(barometric_altitude_rate(1, 0b0_0000_0001), -16352);
    }

    #[test]
    fn test_internal_vertical_velocity() {
        assert_eq!(internal_vertical_velocity(0, 0b1_1111_1111), 16352);
        assert_eq!(internal_vertical_velocity(1, 0b1_1111_1111), -32);
        assert_eq!(internal_vertical_velocity(0, 0b0_0000_0001), 32);
        assert_eq!(internal_vertical_velocity(1, 0b0_0000_0001), -16352);
    }
}

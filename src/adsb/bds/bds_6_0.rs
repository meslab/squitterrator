use super::goodflags;
use crate::adsb::*;

/// Checks if the given ADS-B message is a BDS 6,0 message and returns the extracted data if it is valid.
///
/// # Arguments
///
/// * `message` - The ADS-B message as a slice of `u32` values.
///
/// # Returns
///
/// An `Option` containing a tuple with the extracted data if the message is valid, or `None` otherwise. The tuple contains the following values:
///
/// * `magnetic_heading` - The magnetic heading in degrees.
/// * `indicated_airspeed` - The indicated airspeed in knots.
/// * `mach_number` - The Mach number.
/// * `barometric_altitude_rate` - The barometric altitude rate in feet per minute.
/// * `internal_vertical_velocity` - The internal vertical velocity in feet per minute.
///
pub fn is_bds_6_0(message: &[u32]) -> Option<(u32, u32, f64, i32, i32)> {
    if goodflags(message, 33, 34, 44)
        && goodflags(message, 45, 46, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        let (
            magnetic_heading,
            indicated_airspeed,
            mach_number,
            barometric_altitude_rate,
            internal_vertical_velocity,
        ) = (
            magnetic_heading_6_0(message),
            indicated_airspeed_6_0(message),
            mach_number_6_0(message),
            barometric_altitude_rate_6_0(message),
            internal_vertical_velocity_6_0(message),
        );
        if magnetic_heading.is_some_and(|x| (0..=360).contains(&x))
            && indicated_airspeed.is_some_and(|x| (0..=1023).contains(&x))
            && mach_number.is_some_and(|x| (0.0..=4.092).contains(&x))
            && barometric_altitude_rate.is_some_and(|x| (-16384..=16384).contains(&x))
            && internal_vertical_velocity.is_some_and(|x| (-16384..=16384).contains(&x))
        {
            Some((
                magnetic_heading.unwrap(),
                indicated_airspeed.unwrap(),
                mach_number.unwrap(),
                barometric_altitude_rate.unwrap(),
                internal_vertical_velocity.unwrap(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}

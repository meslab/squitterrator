use super::goodflags;
use crate::adsb::*;
pub struct HeadingAndSpeed {
    pub magnetic_heading: Option<u32>,
    pub indicated_airspeed: Option<u32>,
    pub mach_number: Option<f64>,
    pub barometric_altitude_rate: Option<i32>,
    pub internal_vertical_velocity: Option<i32>,
}

impl HeadingAndSpeed {
    pub fn new() -> Self {
        HeadingAndSpeed {
            magnetic_heading: None,
            indicated_airspeed: None,
            mach_number: None,
            barometric_altitude_rate: None,
            internal_vertical_velocity: None,
        }
    }

    pub fn from_data(
        magnetic_heading: Option<u32>,
        indicated_airspeed: Option<u32>,
        mach_number: Option<f64>,
        barometric_altitude_rate: Option<i32>,
        internal_vertical_velocity: Option<i32>,
    ) -> Self {
        HeadingAndSpeed {
            magnetic_heading,
            indicated_airspeed,
            mach_number,
            barometric_altitude_rate,
            internal_vertical_velocity,
        }
    }
}

impl Default for HeadingAndSpeed {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_6_0(message: &[u32]) -> Option<HeadingAndSpeed> {
    if goodflags(message, 33, 34, 44)
        && goodflags(message, 45, 46, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        let heading = HeadingAndSpeed::from_data(
            magnetic_heading_6_0(message),
            indicated_airspeed_6_0(message),
            mach_number_6_0(message),
            barometric_altitude_rate_6_0(message),
            internal_vertical_velocity_6_0(message),
        );
        if heading
            .magnetic_heading
            .is_some_and(|x| (0..=360).contains(&x))
            && heading
                .indicated_airspeed
                .is_some_and(|x| (0..=1023).contains(&x))
            && heading
                .mach_number
                .is_some_and(|x| (0.0..=1.0).contains(&x))
            && heading
                .barometric_altitude_rate
                .is_some_and(|x| (-6000..=6000).contains(&x))
            && heading
                .internal_vertical_velocity
                .is_some_and(|x| (-6000..=6000).contains(&x))
        {
            Some(heading)
        } else {
            None
        }
    } else {
        None
    }
}

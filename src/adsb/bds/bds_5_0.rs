use super::goodflags;
use crate::adsb::*;

pub fn is_bds_5_0(message: &[u32]) -> Option<(i32, u32, i32, u32, u32)> {
    if goodflags(message, 33, 34, 43)
        && goodflags(message, 44, 45, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        let (roll_angle, track_angle, track_angle_rate, ground_speed, true_airspeed) = (
            roll_angle_5_0(message),
            track_angle_5_0(message),
            track_angle_rate_5_0(message),
            ground_speed_5_0(message),
            true_airspeed_5_0(message),
        );
        if roll_angle.is_some_and(|x| (-90..=90).contains(&x))
            && track_angle.is_some_and(|x| (0..=360).contains(&x))
            && track_angle_rate.is_some_and(|x| (-16..=16).contains(&x))
            && ground_speed.is_some_and(|x| (0..=2046).contains(&x))
            && true_airspeed.is_some_and(|x| (0..=2046).contains(&x))
            && ground_speed.unwrap().abs_diff(true_airspeed.unwrap()) < 200
        {
            Some((
                roll_angle.unwrap(),
                track_angle.unwrap(),
                track_angle_rate.unwrap(),
                ground_speed.unwrap(),
                true_airspeed.unwrap(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}

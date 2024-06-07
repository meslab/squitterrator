use super::goodflags;
use crate::adsb::*;

pub struct Track {
    pub roll_angle: Option<i32>,
    pub track_angle: Option<u32>,
    pub track_angle_rate: Option<i32>,
    pub ground_speed: Option<u32>,
    pub true_airspeed: Option<u32>,
}

impl Track {
    pub fn new() -> Self {
        Track {
            roll_angle: None,
            track_angle: None,
            track_angle_rate: None,
            ground_speed: None,
            true_airspeed: None,
        }
    }

    pub fn from_data(
        roll_angle: Option<i32>,
        track_angle: Option<u32>,
        track_angle_rate: Option<i32>,
        ground_speed: Option<u32>,
        true_airspeed: Option<u32>,
    ) -> Self {
        Track {
            roll_angle,
            track_angle,
            track_angle_rate,
            ground_speed,
            true_airspeed,
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_5_0(message: &[u32]) -> Option<Track> {
    if goodflags(message, 33, 34, 43)
        && goodflags(message, 44, 45, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        let track = Track::from_data(
            roll_angle_5_0(message).filter(|x| (-90..=90).contains(x)),
            track_angle_5_0(message).filter(|x| (0..=360).contains(x)),
            track_angle_rate_5_0(message).filter(|x| (0..=2046).contains(x)),
            ground_speed_5_0(message).filter(|x| (0..=2046).contains(x)),
            true_airspeed_5_0(message).filter(|x| (0..=2046).contains(x)),
        );
        if track.ground_speed.is_some()
            && track.true_airspeed.is_some()
            && track.roll_angle.is_some()
            && track.track_angle.is_some()
            && track.track_angle_rate.is_some()
            && track
                .ground_speed
                .unwrap()
                .abs_diff(track.true_airspeed.unwrap())
                < 200
        {
            Some(track)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn ground_track(message: &[u32]) -> Option<f64> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 45, 46, 52) {
        match status {
            1 => Some(value as f64 * 360.0 / 128.0),
            _ => None,
        }
    } else {
        None
    }
}

pub fn track_and_groundspeed(message: &[u32], is_supersonic: bool) -> (Option<f64>, Option<f64>) {
    let sp_west = match crate::adsb::flag_and_range_value(message, 46, 47, 56) {
        Some((dir_west, speed_west)) => match dir_west {
            1 => -(speed_west as f64 - 1.0),
            _ => speed_west as f64 - 1.0,
        },
        _ => 0.0,
    };

    let sp_south = match crate::adsb::flag_and_range_value(message, 57, 58, 67) {
        Some((dir_south, speed_south)) => match dir_south {
            1 => -(speed_south as f64 - 1.0),
            _ => speed_south as f64 - 1.0,
        },
        _ => 0.0,
    };

    let supersonic = |x| if is_supersonic { x * 4.0 } else { x };
    let groundspeed = supersonic((sp_west.powi(2) + sp_south.powi(2)).sqrt());
    let track = (sp_west.atan2(sp_south).to_degrees() + 360.0) % 360.0;
    (Some(track), Some(groundspeed))
}

#[cfg(test)]
mod tests {
    use crate::adsb;

    use super::*;

    #[test]
    fn test_track_and_groundspeed() {
        if let Some(message) = adsb::message("8DC06A75990D0628B0040C8AA788") {
            if let (Some(track), Some(groundspeed)) = track_and_groundspeed(&message, false) {
                assert_eq!(groundspeed, 416.0492759277439);
                assert_eq!(track, 38.853374340353355);
            };
        };
    }
    #[test]
    fn test_track_and_groundspeed_new() {
        if let Some(message) = adsb::message("8DC06A75990D0628B0040C8AA788") {
            if let (Some(track), Some(groundspeed)) = track_and_groundspeed(&message, false) {
                assert_eq!(track, 38.853374340353355);
                assert_eq!(groundspeed, 416.0492759277439);
            };
        };
    }
}

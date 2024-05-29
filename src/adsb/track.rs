pub fn ground_track(message: &[u32]) -> Option<f64> {
    match (message[11] >> 3) & 1 {
        1 => Some((((message[11] & 0x7) << 4) | (message[12] & 0xF)) as f64 * 360.0 / 128.0),
        _ => None,
    }
}

pub fn track_and_groundspeed(message: &[u32], is_supersonic: bool) -> (Option<f64>, Option<f64>) {
    let dir_west = (message[11] >> 2) & 1;
    let mut sp_west =
        ((message[11] & 3) << 8 | (message[12] & 0xF) << 4 | message[13] & 0xF) as f64;

    sp_west -= 1.0;

    if dir_west > 0 {
        sp_west *= -1.0;
    }

    let dir_south = (message[14] >> 3) & 1;
    let mut sp_south =
        ((message[14] & 7) << 7 | (message[15] & 0xF) << 3 | (message[16] >> 1) & 7) as f64;

    sp_south -= 1.0;

    if dir_south > 0 {
        sp_south *= -1.0;
    }

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
                assert_eq!(track, 321.14662565964665);
                assert_eq!(groundspeed, 416.0492759277439);
            };
        };
    }
}

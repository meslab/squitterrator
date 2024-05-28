pub fn track(message: &[u32]) -> Option<f64> {
    let track = (((message[11] & 0x7) << 4) | message[12] & 0xF) as f64 * 2.8125;
    if (message[11] & 0x8) >> 3 != 0 {
        Some(360.0 - track)
    } else {
        Some(track)
    }
}

pub fn track_and_groundspeed(message: &[u32]) -> (Option<f64>, Option<f64>) {
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

    let groundspeed = (sp_west.powi(2) + sp_south.powi(2)).sqrt();
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
            if let (Some(track), Some(groundspeed)) = track_and_groundspeed(&message) {
                assert_eq!(track, 321.14662565964665);
                assert_eq!(groundspeed, 416.0492759277439);
            };
        };
    }
}

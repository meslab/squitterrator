use crate::adsb::{ais, ma_code, me_code};
use log::debug;

pub struct DecodedMessage {
    pub alt: u32,
    pub ais: String,
    pub vsign: u32,
    pub vrate: i32,
    pub cpr_form: u32,
    pub cpr_lat: u32,
    pub cpr_long: u32,
    pub sp_west: i32,
    pub sp_south: i32,
    pub grspeed: f64,
    pub airspeed: u32,
    pub heading: f64,
    pub turn: u32,
    pub track: f64,
}

impl DecodedMessage {
    pub fn new() -> Self {
        DecodedMessage {
            alt: 0,
            ais: "".to_string(),
            vsign: 0,
            vrate: 0,
            cpr_form: 2,
            cpr_lat: 0,
            cpr_long: 0,
            sp_west: 0,
            sp_south: 0,
            grspeed: 0.0,
            airspeed: 0,
            heading: 0.0,
            turn: 0,
            track: 0.0,
        }
    }
}

impl Default for DecodedMessage {
    fn default() -> Self {
        Self::new()
    }
}

pub fn mode_e_decoded_message(message: &[u32], df: u32) -> Option<DecodedMessage> {
    if message.len() < 28 {
        return None;
    }
    let mut decoded_message = DecodedMessage::new();
    let (message_type, message_subtype) = ((message[8] << 1) | (message[9] >> 3), message[5] & 7);

    debug!("T: {} ST: {}", message_type, message_subtype);

    decoded_message.vsign = (message[17] & 8) >> 3;
    decoded_message.vrate =
        (((message[17] & 7) << 6) | (message[18] << 2) | (message[19] >> 2)) as i32;
    if decoded_message.vrate > 0 {
        decoded_message.vrate = (decoded_message.vrate - 1) * 64;
    }
    decoded_message.turn = message[19] & 3;

    match message_type {
        1..=4 => {
            decoded_message.ais = ais(message)?;
            Some(decoded_message)
        }
        0 | 9..=18 | 20..=21 => {
            if message_type == 20 || message_type == 21 {
                decoded_message.ais = ais(message)?;
            }
            decoded_message.alt = alt(message, df);
            decoded_message.cpr_form = (message[13] & 4) >> 2;
            if decoded_message.cpr_form < 2 {
                decoded_message.cpr_lat = ((message[13] & 3) << 15)
                    | (message[14] << 11)
                    | (message[15] << 7)
                    | (message[16] << 3)
                    | (message[17] >> 1);
                decoded_message.cpr_long = ((message[17] & 1) << 16)
                    | (message[18] << 12)
                    | (message[19] << 8)
                    | (message[20] << 4)
                    | message[21];
            }
            Some(decoded_message)
        }
        5..=8 => {
            decoded_message.track = (((message[11] >> 3) << 4) | message[12]) as f64 * 2.8125;
            if message[11] >> 3 != 0 {
                decoded_message.track = 360.0 - decoded_message.track;
            }
            decoded_message.cpr_form = (message[13] & 4) >> 2;
            decoded_message.cpr_lat = ((message[13] & 3) << 15)
                | (message[14] << 11)
                | (message[15] << 7)
                | (message[16] << 3)
                | (message[17] >> 1);
            decoded_message.cpr_long = ((message[17] & 1) << 16)
                | (message[18] << 12)
                | (message[19] << 8)
                | (message[20] << 4)
                | message[21];
            Some(decoded_message)
        }
        19 => {
            match message_subtype {
                1 | 2 => {
                    let dir_west = (message[11] & 4) >> 2;
                    decoded_message.sp_west =
                        (((message[11] & 3) << 8) | (message[12] << 4) | message[13]) as i32;
                    if message_subtype == 2 && decoded_message.sp_west > 0 {
                        decoded_message.sp_west = ((decoded_message.sp_west - 1) * 4) + 1;
                    }
                    decoded_message.sp_west -= 1;
                    if dir_west > 0 {
                        decoded_message.sp_west *= -1;
                    }

                    let dir_south = (message[14] & 8) >> 3;
                    decoded_message.sp_south =
                        (((message[14] & 7) << 7) | (message[15] << 3) | (message[16] >> 1)) as i32;
                    if message_subtype == 2 && decoded_message.sp_south > 0 {
                        decoded_message.sp_south = ((decoded_message.sp_south - 1) * 4) + 1;
                    }
                    decoded_message.sp_south -= 1;
                    if dir_south > 0 {
                        decoded_message.sp_south *= -1;
                    }

                    decoded_message.grspeed =
                        ((decoded_message.sp_west.pow(2) + decoded_message.sp_south.pow(2)) as f64)
                            .sqrt();
                    decoded_message.heading = (decoded_message.sp_west as f64)
                        .atan2(decoded_message.sp_south as f64)
                        * 180.0
                        / std::f64::consts::PI;
                    if decoded_message.heading < 0.0 {
                        decoded_message.heading += 360.0;
                    }
                }
                3 | 4 => {
                    // let head_stat = (message[11] & 4) >> 2;
                    match ((message[11] & 3) << 8) | (message[12] << 4) | message[13] {
                        0 => decoded_message.heading = 0.0,
                        heading => {
                            decoded_message.heading =
                                heading as f64 * 360.0 / 1024.0 / std::f64::consts::PI;
                        }
                    }
                    // let airspeed_type = (message[14] & 8) >> 3;
                    decoded_message.airspeed =
                        ((message[14] & 7) << 7) | (message[15] << 3) | (message[16] >> 1);
                    if message_subtype == 4 && decoded_message.airspeed > 0 {
                        decoded_message.airspeed = ((decoded_message.airspeed - 1) * 4) + 1;
                    }
                }
                _ => {}
            }
            Some(decoded_message)
        }
        _ => None,
    }
}

fn alt(message: &[u32], df: u32) -> u32 {
    let code = match df {
        17 => me_code(message),
        _ => ma_code(message),
    };

    match code {
        Some(code) => match code & 0b10 {
            0 => match code & 1 {
                0 => {
                    let (high, low) = graytobin(message);
                    high * 500 + low * 100 + 1200
                }
                _ => (((code >> 7) << 4) | ((code >> 2) & 0b1111)) as u32 * 25 + 1000,
            },
            _ => ((((code >> 7) << 4) & 0b11111110000 | (code >> 2) & 0b1111) as f32 * 0.31) as u32,
        },
        None => 0,
    }
}

fn graytobin(message: &[u32]) -> (u32, u32) {
    if let Some(code) = ma_code(message) {
        let n = ((code >> 4) & 1) << 10
            | ((code >> 2) & 1) << 9
            | ((code >> 12) & 1) << 8
            | ((code >> 10) & 1) << 7
            | ((code >> 8) & 1) << 6
            | ((code >> 7) & 1) << 5
            | ((code >> 5) & 1) << 4
            | ((code >> 3) & 1) << 3
            | ((code >> 13) & 1) << 2
            | ((code >> 11) & 1) << 1
            | ((code >> 13) & 1);
        let mut mask = 0x80;
        let mut cp = false;
        let mut result = 0;
        for _ in 1..=16 {
            if (n & mask) != 0 {
                cp = !cp;
            }
            if cp {
                result |= mask;
            }
            mask >>= 1;
        }

        let sub = n & 7;
        let high = result >> 3;
        let low = match high % 2 {
            0 => match sub {
                4 => 4,
                6 => 3,
                3 => 1,
                2 => 2,
                _ => 0,
            },
            _ => match sub {
                1 => 4,
                3 => 3,
                6 => 1,
                2 => 2,
                _ => 0,
            },
        };
        (high as u32, low as u32)
    } else {
        (0, 0)
    }
}

//fn get_position(cpr: &[&[u32]]) -> (f64, f64) {
//    let div = 1 << 17;
//    let adl0 = 360.0 / 60.0;
//    let ald1 = 360.0 / 59.0;
//
//    (0.0, 0.0)
//}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb;

    #[test]
    fn test_graytobin() {
        if let Some(message) = adsb::message("A8281200200464B3CF7820CD194C") {
            let (high, low) = graytobin(&message);
            assert_eq!(high, 31);
            assert_eq!(low, 0);
        }
    }

    #[test]
    fn test_alt() {
        if let Some(message) = adsb::message("A8281200200464B3CF7820CD194C") {
            let df = adsb::df(&message);
            let result = alt(&message, df);
            assert_eq!(result, 16700);
        }
    }
}

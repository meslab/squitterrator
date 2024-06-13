use super::Df17;
use crate::decoder;

impl Df17 {
    pub fn from_message(message: &[u32]) -> Self {
        let mut dl = Df17::new();
        if let Some(df) = decoder::df(message) {
            dl.icao = decoder::icao(message, df);
            dl.capability = decoder::ca(message);
            dl.message_type = decoder::message_type(message);
            dl.adsb_version = decoder::version(message);
            match dl.message_type.0 {
                1..=4 => {
                    dl.ais = decoder::ais(message);
                    dl.category = Some(dl.message_type);
                }
                5..=18 => {
                    dl.cpr = decoder::cpr(message);
                    match dl.message_type.0 {
                        5..=8 => {
                            dl.ground_movement = decoder::ground_movement(message);
                            dl.track = decoder::ground_track(message);
                            dl.track_source = Some('\u{2070}');
                            dl.altitude_source = Some('\u{2070}');
                        }
                        9..=18 => {
                            dl.altitude = decoder::altitude(message, df);
                            dl.surveillance_status = Some(decoder::surveillance_status(message));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
        dl
    }
}

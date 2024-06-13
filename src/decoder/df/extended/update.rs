use super::Ext;
use crate::decoder;

impl Ext {
    pub fn from_message(message: &[u32]) -> Self {
        let mut dl = Ext::new();
        if let Some(df) = decoder::df(message) {
            dl.icao = decoder::icao(message, df);
            dl.capability = decoder::ca(message);
            dl.message_type = decoder::message_type(message);
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
                19 => {
                    dl.vrate = decoder::vertical_rate(message);
                    dl.altitude_delta = decoder::altitude_delta(message);
                    match dl.message_type.1 {
                        1 => {
                            (dl.track, dl.grspeed) = decoder::track_and_groundspeed(message, false);
                            dl.track_source = Some('\u{2081}');
                        }
                        2 => {
                            (dl.track, dl.grspeed) = decoder::track_and_groundspeed(message, true);
                            dl.track_source = Some('\u{2082}');
                        }
                        3 | 4 => {
                            dl.heading = decoder::heading(message);
                            dl.heading_source = Some('\u{2083}');
                        }
                        _ => {}
                    }
                }
                20..=22 => {
                    dl.altitude_gnss = decoder::altitude_gnss(message);
                    dl.surveillance_status = Some(decoder::surveillance_status(message));
                }
                31 => {
                    dl.adsb_version = decoder::version(message);
                }
                _ => {}
            }
        };
        dl
    }
}

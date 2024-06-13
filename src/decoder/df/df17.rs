use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df17 {
    pub icao: Option<u32>,
    pub capability: u32,
    pub message_type: (u32, u32),
    pub ais: Option<String>,
    pub category: Option<(u32, u32)>,
    pub cpr: Option<(u32, u32, u32)>,
    pub ground_movement: Option<f64>,
    pub altitude: Option<u32>,
    pub altitude_source: Option<char>,
    pub surveillance_status: Option<char>,
    pub adsb_version: Option<u32>,
}

impl Default for Df17 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df17 {
    pub fn new() -> Self {
        Df17 {
            icao: None,
            capability: 0,
            message_type: (0, 0),
            ais: None,
            category: None,
            cpr: None,
            ground_movement: None,
            altitude: None,
            altitude_source: None,
            surveillance_status: None,
            adsb_version: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        let mut dl = Df17::new();
        if let Some(df) = super::df(message) {
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

impl Display for Df17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DF17")?;
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        write!(f, ",{}", self.capability)?;
        write!(f, ",{}.{}", self.message_type.0, self.message_type.1)?;
        if let Some(v) = &self.ais {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.surveillance_status {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.adsb_version {
            writeln!(f, ",{:X}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

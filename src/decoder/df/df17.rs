use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df17 {
    pub icao: Option<u32>,
    pub capability: u32,
    pub message_type: (u32, u32),
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
            adsb_version: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df17 {
                icao: decoder::icao(message, df),
                capability: decoder::ca(message),
                message_type: decoder::message_type(message),
                adsb_version: decoder::version(message),
            }
        } else {
            Df17::new()
        }
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
        if let Some(v) = self.adsb_version {
            writeln!(f, ",{:X}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Srt {
    pub icao: Option<u32>,
    pub squawk: Option<u32>,
    pub altitude: Option<u32>,
    pub capability: Option<u32>,
}

impl Default for Srt {
    fn default() -> Self {
        Self::new()
    }
}

impl Srt {
    pub fn new() -> Self {
        Srt {
            icao: None,
            squawk: None,
            altitude: None,
            capability: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = decoder::df(message) {
            match df {
                4 => Srt {
                    icao: decoder::icao(message, df),
                    squawk: None,
                    altitude: decoder::altitude(message, df),
                    capability: None,
                },
                5 => Srt {
                    icao: decoder::icao(message, df),
                    squawk: decoder::squawk(message),
                    altitude: None,
                    capability: None,
                },
                11 => Srt {
                    icao: decoder::icao(message, df),
                    squawk: None,
                    altitude: None,
                    capability: Some(decoder::ca(message)),
                },
                _ => Srt::new(),
            }
        } else {
            Srt::new()
        }
    }
}

impl Display for Srt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SRT")?;
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.squawk {
            writeln!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.capability {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.altitude {
            writeln!(f, ",{}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df4 {
    pub icao: Option<u32>,
    pub altitude: Option<u32>,
}

impl Default for Df4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df4 {
    pub fn new() -> Self {
        Df4 {
            icao: None,
            altitude: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df4 {
                icao: decoder::icao(message, df),
                altitude: decoder::altitude(message, df),
            }
        } else {
            Df4::new()
        }
    }
}

impl Display for Df4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DF4 ")?;
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
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

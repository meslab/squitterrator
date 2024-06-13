use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Mds {
    pub df: Option<u32>,
    pub icao: Option<u32>,
    pub altitude: Option<u32>,
}

impl Default for Mds {
    fn default() -> Self {
        Self::new()
    }
}

impl Mds {
    pub fn new() -> Self {
        Mds {
            df: None,
            icao: None,
            altitude: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = decoder::df(message) {
            Mds {
                df: Some(df),
                icao: decoder::icao(message, df),
                altitude: decoder::altitude(message, df),
            }
        } else {
            Mds::new()
        }
    }
}

impl Display for Mds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.df {
            write!(f, "DF{:02}", v)?
        } else {
            write!(f, "")?
        }
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

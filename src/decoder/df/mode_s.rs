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

impl decoder::Downlink for Mds {
    fn from_message(message: &[u32]) -> Result<Self, String> {
        if let Some(df) = decoder::df(message) {
            Ok(Mds {
                df: Some(df),
                icao: decoder::icao(message, df),
                altitude: decoder::altitude(message, df),
            })
        } else {
            Err("cannot extract df".to_string())
        }
    }

    fn update(&mut self, message: &[u32]) {
        if let Some(df) = decoder::df(message) {
            self.df = Some(df);
            self.icao = decoder::icao(message, df);
        }
    }
}

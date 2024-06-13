use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df11 {
    pub icao: Option<u32>,
    pub capability: u32,
}

impl Default for Df11 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df11 {
    pub fn new() -> Self {
        Df11 {
            icao: None,
            capability: 0,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = decoder::df(message) {
            Df11 {
                icao: decoder::icao(message, df),
                capability: decoder::ca(message),
            }
        } else {
            Df11::new()
        }
    }
}

impl Display for Df11 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DF11")?;
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        writeln!(f, ",{}", self.capability)
    }
}

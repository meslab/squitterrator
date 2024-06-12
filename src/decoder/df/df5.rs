use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df5 {
    pub icao: Option<u32>,
    pub squawk: Option<u32>,
}

impl Default for Df5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df5 {
    pub fn new() -> Self {
        Df5 {
            icao: None,
            squawk: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df5 {
                icao: decoder::icao(message, df),
                squawk: decoder::squawk(message),
            }
        } else {
            Df5::new()
        }
    }
}

impl Display for Df5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.icao {
            write!(f, "{:X},", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.squawk {
            write!(f, "{},", v)?
        } else {
            write!(f, ",")?
        }
        write!(f, "")
    }
}

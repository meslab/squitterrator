use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df21 {
    pub icao: Option<u32>,
    pub squawk: Option<u32>,
}

impl Default for Df21 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df21 {
    pub fn new() -> Self {
        Df21 {
            icao: None,
            squawk: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df21 {
                icao: decoder::icao(message, df),
                squawk: decoder::squawk(message),
            }
        } else {
            Df21::new()
        }
    }
}

impl Display for Df21 {
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

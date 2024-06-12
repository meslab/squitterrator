use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df0 {
    pub icao: Option<u32>,
}

impl Default for Df0 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df0 {
    pub fn new() -> Self {
        Df0 { icao: None }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df0 {
                icao: decoder::icao(message, df),
            }
        } else {
            Df0::new()
        }
    }
}

impl Display for Df0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.icao {
            write!(f, "{:X}", v)?
        } else {
            write!(f, "")?
        }
        write!(f, "")
    }
}

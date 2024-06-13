use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Df17 {
    pub icao: Option<u32>,
    pub capability: u32,
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
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df17 {
                icao: decoder::icao(message, df),
                capability: decoder::ca(message),
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
        writeln!(f, "")
    }
}

use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Mds {
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
            icao: None,
            altitude: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = decoder::df(message) {
            Mds {
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
        write!(f, "MDS")?;
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

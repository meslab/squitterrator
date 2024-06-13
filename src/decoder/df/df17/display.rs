use std::fmt::{self, Display};
use super::Df17;

impl Display for Df17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DF17")?;
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        write!(f, ",{}", self.capability)?;
        write!(f, ",{}.{}", self.message_type.0, self.message_type.1)?;
        if let Some(v) = &self.ais {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.category {
            write!(f, ",{}{}", v.0, v.1)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.cpr {
            write!(f, ",{},{},{}", v.0, v.1, v.2)?
        } else {
            write!(f, ",,,")?
        }
        if let Some(v) = &self.surveillance_status {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.adsb_version {
            writeln!(f, ",{:X}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

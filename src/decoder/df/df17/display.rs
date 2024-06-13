use super::Df17;
use std::fmt::{self, Display};

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
        if let Some(v) = &self.ground_movement {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.track {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.track_source {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.heading {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.heading_source {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.altitude {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.altitude_source {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.altitude_delta {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.altitude_gnss {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.vrate {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = &self.vrate_source {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
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

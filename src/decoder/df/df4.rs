use crate::decoder;

pub struct Df4 {
    pub icao: Option<u32>,
    pub altitude: Option<u32>,
}

impl Default for Df4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df4 {
    pub fn new() -> Self {
        Df4 {
            icao: None,
            altitude: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df4 {
                icao: decoder::icao(message, df),
                altitude: None,
            }
        } else {
            Df4::new()
        }
    }
}

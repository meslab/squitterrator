use crate::decoder;

pub struct Df0 {
    pub icao: Option<u32>,
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

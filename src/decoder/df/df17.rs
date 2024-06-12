use crate::decoder;

pub struct Df17 {
    pub icao: Option<u32>,
}

impl Df17 {
    pub fn new() -> Self {
        Df17 { icao: None }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df17 {
                icao: decoder::icao(message, df),
            }
        } else {
            Df17::new()
        }
    }
}

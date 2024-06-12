use crate::decoder;

pub struct Df5 {
    pub icao: Option<u32>,
}

impl Df5 {
    pub fn new() -> Self {
        Df5 { icao: None }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df5 {
                icao: decoder::icao(message, df),
            }
        } else {
            Df5::new()
        }
    }
}

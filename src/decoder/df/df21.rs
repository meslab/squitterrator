use crate::decoder;

pub struct Df21 {
    pub icao: Option<u32>,
}

impl Df21 {
    pub fn new() -> Self {
        Df21 { icao: None }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df21 {
                icao: decoder::icao(message, df),
            }
        } else {
            Df21::new()
        }
    }
}

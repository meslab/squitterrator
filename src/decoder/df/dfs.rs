use core::fmt;

use log::error;

use super::*;

#[derive(Debug)]
pub enum DF {
    DF0(Df0),
    DF4(Df4),
    DF5(Df5),
    DF11(Df11),
    DF17(Df17),
    DF20(Df20),
    DF21(Df21),
}

pub fn get_downlink(message: &[u32]) -> Option<DF> {
    df(message).map(|df| match df {
        4 => DF::DF4(Df4::from_message(message)),
        5 => DF::DF5(Df5::from_message(message)),
        11 => DF::DF11(Df11::from_message(message)),
        17 => DF::DF17(Df17::from_message(message)),
        20 => DF::DF20(Df20::from_message(message)),
        21 => DF::DF21(Df21::from_message(message)),
        _ => {
            error!("Cannot create DF:{}", df);
            DF::DF0(Df0::new())
        }
    })
}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DF::DF4(v) => write!(f, "{}", v),
            DF::DF5(v) => write!(f, "{}", v),
            DF::DF11(v) => write!(f, "{}", v),
            DF::DF17(v) => write!(f, "{}", v),
            DF::DF20(v) => write!(f, "{}", v),
            DF::DF21(v) => write!(f, "{}", v),
            _ => Ok(()),
        }
    }
}

use log::error;

use super::*;

pub enum DF {
    DF0(Df0),
    DF4(Df4),
    DF5(Df5),
    DF11(Df11),
    DF17(Df17),
    DF20(Df20),
    DF21(Df21),
}

pub fn new_df(df: u32, icao: u32) -> DF {
    match df {
        4 => DF::DF4(Df4::new(icao)),
        5 => DF::DF5(Df5::new(icao)),
        11 => DF::DF11(Df11::new(icao)),
        17 => DF::DF17(Df17::new(icao)),
        20 => DF::DF20(Df20::new(icao)),
        21 => DF::DF21(Df21::new(icao)),
        _ => {
            error!("Cannot create DF:{}", df);
            DF::DF0(Df0::new(icao))
        }
    }
}

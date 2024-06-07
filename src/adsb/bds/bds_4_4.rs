use crate::adsb;

pub struct Meteo {
    pub temp: f64,
    pub wind: (u32, u32),
    pub humidity: u32,
    pub turbulence: u32,
    pub pressure: u32,
}

impl Meteo {
    pub fn new() -> Self {
        Meteo {
            temp: 0.0,
            wind: (0, 0),
            humidity: 0,
            turbulence: 0,
            pressure: 0,
        }
    }

    pub fn from_data(
        temp: f64,
        wind: (u32, u32),
        humidity: u32,
        turbulence: u32,
        pressure: u32,
    ) -> Self {
        Meteo {
            temp,
            wind,
            humidity,
            turbulence,
            pressure,
        }
    }
}

impl Default for Meteo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_4_4(message: &[u32]) -> Option<Meteo> {
    if let Some((_, fom)) = crate::adsb::flag_and_range_value(message, 33, 33, 36) {
        if fom > 0
            && adsb::goodflags(message, 37, 38, 55)
            && adsb::goodflags(message, 37, 57, 66)
            && adsb::goodflags(message, 67, 68, 78)
            && adsb::goodflags(message, 79, 80, 81)
            && adsb::goodflags(message, 82, 83, 88)
        {
            let (temp, wind, humidity, turbulence, pressure) = (
                adsb::temperature_4_4(message),
                adsb::wind_4_4(message),
                adsb::humidity_4_4(message),
                adsb::turbulence_4_4(message),
                adsb::pressure_4_4(message),
            );
            match (temp, wind, humidity, turbulence, pressure) {
                (Some(temp), Some(wind), Some(humidity), Some(turbulence), Some(pressure)) => {
                    Some(Meteo::from_data(temp, wind, humidity, turbulence, pressure))
                }
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

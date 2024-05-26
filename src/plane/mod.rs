use crate::adsb;

pub struct Plane {
    pub icao: u32,
    pub ais: Option<String>,
    pub alt: u32,
    pub squawk: Option<u32>,
    pub vsign: u32,
    pub vrate: i32,
    pub cpr_lat: [u32; 2],
    pub cpr_long: [u32; 2],
    pub lat: f64,
    pub lon: f64,
    pub sp_west: i32,
    pub sp_south: i32,
    pub grspeed: f64,
    pub airspeed: u32,
    pub heading: f64,
    pub turn: u32,
    pub track: f64,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            icao: 0,
            ais: None,
            alt: 0,
            squawk: None,
            vsign: 0,
            vrate: 0,
            cpr_lat: [0, 0],
            cpr_long: [0, 0],
            lat: 0.0,
            lon: 0.0,
            sp_west: 0,
            sp_south: 0,
            grspeed: 0.0,
            airspeed: 0,
            heading: 0.0,
            turn: 0,
            track: 0.0,
        }
    }

    pub fn from_message(message: &[u32], df: u32, icao: u32) -> Self {
        let mut plane = Plane::new();
        plane.icao = icao;
        plane.squawk = match df {
            5 | 21 => adsb::squawk(message),
            _ => None,
        };
        if let Some(dm) = adsb::mode_e_decoded_message(message, df) {
            plane.alt = dm.alt;
            plane.ais = dm.ais;
            plane.vsign = dm.vsign;
            plane.vrate = dm.vrate;
            match dm.cpr_form {
                0 | 1 => {
                    plane.cpr_lat[dm.cpr_form as usize] = dm.cpr_lat;
                    plane.cpr_long[dm.cpr_form as usize] = dm.cpr_long;
                    // let (lat, lon) = adsb::cpr(&decoded_message, icao);
                    // plane.lat = lat;
                    // plane.lon = lon;
                }
                _ => {}
            }
            plane.sp_west = dm.sp_west;
            plane.sp_south = dm.sp_south;
            plane.grspeed = dm.grspeed;
            plane.airspeed = dm.airspeed;
            plane.heading = dm.heading;
            plane.turn = dm.turn;
            plane.track = dm.track;
        }
        plane
    }

    pub fn update(&mut self, message: &[u32], df: u32) {
        if let Some(dm) = adsb::mode_e_decoded_message(message, df) {
            self.alt = dm.alt;
            self.ais = dm.ais;
            self.vsign = dm.vsign;
            self.vrate = dm.vrate;
            match dm.cpr_form {
                0 | 1 => {
                    self.cpr_lat[dm.cpr_form as usize] = dm.cpr_lat;
                    self.cpr_long[dm.cpr_form as usize] = dm.cpr_long;
                    // let (lat, lon) = adsb::cpr(&decoded_message, icao);
                    // self.lat = lat;
                    // self.lon = lon;
                }
                _ => {}
            }
            self.sp_west = dm.sp_west;
            self.sp_south = dm.sp_south;
            self.grspeed = dm.grspeed;
            self.airspeed = dm.airspeed;
            self.heading = dm.heading;
            self.turn = dm.turn;
            self.track = dm.track;
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

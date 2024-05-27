use crate::adsb;
use std::{fmt, fmt::Display};

pub struct Plane {
    pub icao: u32,
    pub ais: Option<String>,
    pub alt: Option<u32>,
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
            alt: None,
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
        plane.update(message, df);
        plane
    }

    pub fn update(&mut self, message: &[u32], df: u32) {
        if df == 5 || df == 21 {
            if let Some(squawk) = adsb::squawk(message) {
                self.squawk = Some(squawk);
            }
        }
        if df == 17 || df == 18 {
            let (message_type, _message_subtype) = adsb::message_type(message);
            match message_type {
                1..=4 => {
                    if let Some(result) = adsb::ais(message) {
                        self.ais = Some(result);
                    }
                }
                5..=8 => {}
                _ => {}
            }
        }
        if df == 20 || df == 21 {
            let (bds1, bds2) = adsb::bds(message);
            if bds1 == 2 && bds2 == 0 {
                self.ais = adsb::ais(message);
            }
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ICAO: {:06X}", self.icao)?;
        if let Some(alt) = self.alt {
            write!(f, " Alt: {}", alt)?;
        } else {
            write!(f, " {:10}", "")?;
        }
        if let Some(squawk) = self.squawk {
            write!(f, " Squawk: {:04}", squawk)?;
        } else {
            write!(f, " {:12}", "")?;
        }
        if let Some(ais) = &self.ais {
            write!(f, " AIS: {:8}", ais)?;
        } else {
            write!(f, " {:13}", "")?;
        }
        write!(f, "")

        //write!(f, "ICAO: {:06X} Alt: {:?} Squawk: {:?} Lat: {:.6} Lon: {:.6} Heading: {:.1} Track: {:.1} Airspeed: {} Groundspeed: {:.1} Vrate: {}",
        //    self.icao, self.alt, self.squawk, self.lat, self.lon, self.heading, self.track, self.airspeed, self.grspeed, self.vrate)
    }
}

use crate::adsb;
use chrono::{DateTime, Utc};
use std::{fmt, fmt::Display};

pub struct Plane {
    pub icao: u32,
    pub reg: &'static str,
    pub ais: Option<String>,
    pub alt: Option<u32>,
    pub squawk: Option<u32>,
    pub vsign: u32,
    pub vrate: i32,
    pub cpr_lat: [u32; 2],
    pub cpr_lon: [u32; 2],
    pub lat: f64,
    pub lon: f64,
    pub sp_west: i32,
    pub sp_south: i32,
    pub grspeed: f64,
    pub airspeed: u32,
    pub heading: f64,
    pub turn: u32,
    pub track: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            icao: 0,
            reg: "",
            ais: None,
            alt: None,
            squawk: None,
            vsign: 0,
            vrate: 0,
            cpr_lat: [0, 0],
            cpr_lon: [0, 0],
            lat: 0.0,
            lon: 0.0,
            sp_west: 0,
            sp_south: 0,
            grspeed: 0.0,
            airspeed: 0,
            heading: 0.0,
            turn: 0,
            track: None,
            timestamp: Utc::now(),
        }
    }

    pub fn from_message(message: &[u32], df: u32, icao: u32) -> Self {
        let mut plane = Plane::new();
        plane.icao = icao;
        (_, plane.reg) = crate::country::icao_to_country(icao);
        plane.update(message, df);
        plane
    }

    pub fn update(&mut self, message: &[u32], df: u32) {
        self.timestamp = Utc::now();
        if df == 4 {
            if let Some(alt) = adsb::alt(message, df) {
                self.alt = Some(alt);
            }
        }
        if df == 5 || df == 21 {
            if let Some(squawk) = adsb::squawk(message) {
                self.squawk = Some(squawk);
            }
        }
        if df == 17 || df == 18 {
            let (message_type, _message_subtype) = adsb::message_type(message);
            match message_type {
                1..=4 => {
                    self.ais = adsb::ais(message);
                }
                9..=18 => {
                    self.track = adsb::track(message);
                    self.alt = adsb::alt(message, df);
                    let (cpr_form, cpr_lat, cpr_lon) = adsb::cpr(message);
                    match cpr_form {
                        0 | 1 => {
                            self.cpr_lat[cpr_form as usize] = cpr_lat;
                            self.cpr_lon[cpr_form as usize] = cpr_lon;
                        }
                        _ => {}
                    }
                    if self.cpr_lat[0] != 0
                        && self.cpr_lat[1] != 0
                        && self.cpr_lon[0] != 0
                        && self.cpr_lon[1] != 0
                    {
                        if let Some((lat, lon)) =
                            adsb::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form)
                        {
                            self.lat = lat;
                            self.lon = lon;
                        }
                    }
                }
                _ => {}
            }
        }
        if df == 20 || df == 21 {}
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
        write!(f, " Reg: {:2}", self.reg)?;
        if let Some(alt) = self.alt {
            write!(f, " Alt: {:>5}", alt)?;
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
        if self.lat != 0.0 && self.lon != 0.0 {
            write!(f, " Lat: {:10.6} Lon: {:11.6}", self.lat, self.lon)?;
        } else {
            write!(f, " {:15} {:16}", "", "")?;
        }
        if let Some(track) = self.track {
            write!(f, " Track: {:>3.0}", track)?;
        } else {
            write!(f, " {:15}", "")?;
        }
        write!(f, "")
    }
}

pub trait SimpleDisplay {
    fn simple_display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl SimpleDisplay for Plane {
    fn simple_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:06X}", self.icao)?;
        write!(f, " {:2}", self.reg)?;
        if let Some(alt) = self.alt {
            write!(f, " {:>5}", alt)?;
        } else {
            write!(f, " {:5}", "")?;
        }
        if let Some(squawk) = self.squawk {
            write!(f, " {:04}", squawk)?;
        } else {
            write!(f, " {:4}", "")?;
        }
        if let Some(ais) = &self.ais {
            write!(f, " {:8}", ais)?;
        } else {
            write!(f, " {:8}", "")?;
        }
        if self.lat != 0.0 && self.lon != 0.0 {
            write!(f, " {:10.6} {:11.6}", self.lat, self.lon)?;
        } else {
            write!(f, " {:10} {:11}", "", "")?;
        }
        if let Some(track) = self.track {
            write!(f, " {:>3.0}", track)?;
        } else {
            write!(f, " {:3}", "")?;
        }
        write!(
            f,
            " {:>3}",
            Utc::now()
                .signed_duration_since(self.timestamp)
                .num_seconds()
        )
    }
}

pub struct SimpleDisplayWrapper<'a, T: SimpleDisplay>(&'a T);

impl<'a, T: SimpleDisplay> fmt::Display for SimpleDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.simple_display(f)
    }
}

pub fn format_simple_display<T: SimpleDisplay>(item: &T) -> String {
    format!("{}", SimpleDisplayWrapper(item))
}

use crate::adsb;
use chrono::{DateTime, Utc};
use log::info;
use std::{fmt, fmt::Display};

pub struct Plane {
    pub icao: u32,
    pub category: (u32, u32),
    pub reg: &'static str,
    pub ais: Option<String>,
    pub alt: Option<u32>,
    pub alt_gnss: Option<u32>,
    pub squawk: Option<u32>,
    pub surveillance_status: char,
    pub vrate: Option<i32>,
    pub cpr_lat: [u32; 2],
    pub cpr_lon: [u32; 2],
    pub cpr_time: [DateTime<Utc>; 2],
    pub lat: f64,
    pub lon: f64,
    pub sp_west: i32,
    pub sp_south: i32,
    pub grspeed: Option<f64>,
    pub ground_movement: Option<f64>,
    pub airspeed: u32,
    pub turn: u32,
    pub track: Option<f64>,
    pub heading: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub position_timestamp: Option<DateTime<Utc>>,
    pub last_type_code: u32,
    pub last_df: u32,
    pub adsb_version: Option<u32>,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            icao: 0,
            category: (0, 0),
            reg: "",
            ais: None,
            alt: None,
            alt_gnss: None,
            squawk: None,
            surveillance_status: ' ',
            vrate: None,
            cpr_lat: [0, 0],
            cpr_lon: [0, 0],
            cpr_time: [Utc::now(), Utc::now()],
            lat: 0.0,
            lon: 0.0,
            sp_west: 0,
            sp_south: 0,
            grspeed: None,
            airspeed: 0,
            ground_movement: None,
            turn: 0,
            track: None,
            heading: None,
            timestamp: Utc::now(),
            position_timestamp: None,
            last_type_code: 0,
            last_df: 0,
            adsb_version: None,
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
        self.last_df = df;

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
            let (message_type, message_subtype) = adsb::message_type(message);
            self.last_type_code = message_type;
            info!("DF:{}, TC:{}, ST:{}", df, message_type, message_subtype);
            match message_type {
                1..=4 => {
                    self.ais = adsb::ais(message);
                    self.category = (message_type, message_subtype);
                }
                5..=18 => {
                    self.alt = adsb::alt(message, df);
                    let (cpr_form, cpr_lat, cpr_lon) = adsb::cpr(message);
                    match cpr_form {
                        0 | 1 => {
                            self.cpr_lat[cpr_form as usize] = cpr_lat;
                            self.cpr_lon[cpr_form as usize] = cpr_lon;
                            self.cpr_time[cpr_form as usize] = Utc::now();
                        }
                        _ => {}
                    }
                    if let 5..=8 = message_type {
                        self.ground_movement = adsb::ground_movement(message);
                    }
                    if let 9..=18 = message_type {
                        self.surveillance_status = adsb::surveillance_status(message);
                    }
                    if self.cpr_lat[0] != 0
                        && self.cpr_lat[1] != 0
                        && self.cpr_lon[0] != 0
                        && self.cpr_lon[1] != 0
                        && self.cpr_time[0]
                            .signed_duration_since(self.cpr_time[1])
                            .num_seconds()
                            .abs()
                            < 10
                    {
                        if let Some((lat, lon)) = match message_type {
                            5..=8 => {
                                self.track = adsb::ground_track(message);
                                adsb::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 4)
                            }
                            9..=18 => adsb::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 1),
                            _ => None,
                        } {
                            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                                self.lat = lat;
                                self.lon = lon;
                                self.position_timestamp = Some(Utc::now());
                            }
                        }
                    }
                }
                19 => {
                    self.vrate = adsb::vertical_rate(message);
                    match message_subtype {
                        1 => {
                            (self.track, self.grspeed) =
                                adsb::track_and_groundspeed(message, false);
                        }
                        2 => {
                            // 4 knots units for supersonic
                            (self.track, self.grspeed) = adsb::track_and_groundspeed(message, true);
                        }
                        _ => {}
                    }
                }
                20..=22 => {
                    self.alt_gnss = adsb::alt_gnss(message);
                    self.surveillance_status = adsb::surveillance_status(message);
                }
                31 => {
                    self.adsb_version = adsb::version(message);
                }
                _ => {}
            }
        }
        if df == 20 || df == 21 {
            let bds = adsb::bds(message);
            if bds == (2, 0) {
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
        if let Some(grspeed) = self.grspeed {
            write!(f, " GSpd: {:>4.0}", grspeed)?;
        } else {
            write!(f, " {:10}", "")?;
        }
        if let Some(track) = self.track {
            write!(f, " Track: {:>3.0}", track)?;
        } else {
            write!(f, " {:15}", "")?;
        }
        if let Some(heading) = self.heading {
            write!(f, " Heading: {:>3.0}", heading)?;
        } else {
            write!(f, " {:15}", "")?;
        }
        write!(f, "")
    }
}

pub trait SimpleDisplay {
    fn simple_display(&self, f: &mut fmt::Formatter, wide: bool) -> fmt::Result;
}

impl SimpleDisplay for Plane {
    fn simple_display(&self, f: &mut fmt::Formatter, wide: bool) -> fmt::Result {
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
            write!(f, " {:9.5} {:11.5}", self.lat, self.lon)?;
        } else {
            write!(f, " {:9} {:11}", "", "")?;
        }
        if let Some(grspeed) = self.grspeed {
            write!(f, " {:>4.0}", grspeed)?;
        } else {
            write!(f, " {:4}", "")?;
        }
        if let Some(track) = self.track {
            write!(f, " {:>3.0}", track)?;
        } else {
            write!(f, " {:3}", "")?;
        }
        if let Some(vrate) = self.vrate {
            write!(f, " {:>5}", vrate)?;
        } else {
            write!(f, " {:5}", "")?;
        }
        if wide {
            write!(f, " {}{}", self.category.0, self.category.1)?;
            if self.last_df != 0 {
                write!(f, " {:>2}", self.last_df)?;
            } else {
                write!(f, " {:2}", "")?;
            }
            if self.last_type_code != 0 {
                write!(f, " {:>2}", self.last_type_code)?;
            } else {
                write!(f, " {:2}", "")?;
            }
            if let Some(version) = self.adsb_version {
                write!(f, " {:1}", version)?;
            } else {
                write!(f, " {:1}", "")?;
            }
            write!(f, " {}", self.surveillance_status)?;
            if let Some(position_timestamp) = self.position_timestamp {
                write!(
                    f,
                    " {:>3}",
                    Utc::now()
                        .signed_duration_since(position_timestamp)
                        .num_seconds()
                )?;
            } else {
                write!(f, " {:3}", "")?;
            }
        }
        write!(
            f,
            " {:>2}",
            Utc::now()
                .signed_duration_since(self.timestamp)
                .num_seconds()
        )
    }
}

pub struct SimpleDisplayWrapper<'a, T: SimpleDisplay>(&'a T, bool);

impl<'a, T: SimpleDisplay> fmt::Display for SimpleDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.simple_display(f, self.1)
    }
}

pub fn format_simple_display<T: SimpleDisplay>(item: &T, wide: bool) -> String {
    format!("{}", SimpleDisplayWrapper(item, wide))
}

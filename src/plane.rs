use crate::adsb::{self, is_bds_4_4, Capability};
use chrono::{DateTime, Utc};
use log::{debug, error};
use std::fmt::{self, Display};

pub struct Plane {
    pub icao: u32,
    pub capability: (u32, Capability),
    pub category: (u32, u32),
    pub reg: &'static str,
    pub ais: Option<String>,
    pub altitude: Option<u32>,
    pub altitude_gnss: Option<u32>,
    pub altitude_source: char,
    pub squawk: Option<u32>,
    pub surveillance_status: char,
    pub threat_encounter: Option<char>,
    pub vrate: Option<i32>,
    pub vrate_source: char,
    pub cpr_lat: [u32; 2],
    pub cpr_lon: [u32; 2],
    pub cpr_time: [DateTime<Utc>; 2],
    pub lat: f64,
    pub lon: f64,
    pub grspeed: Option<u32>,
    pub true_airspeed: Option<u32>,
    pub indicated_airspeed: Option<u32>,
    pub mach_number: Option<f64>,
    pub ground_movement: Option<f64>,
    pub turn: u32,
    pub track: Option<u32>,
    pub track_source: char,
    pub heading: Option<u32>,
    pub heading_source: char,
    pub roll_angle: Option<i32>,
    pub track_angle_rate: Option<i32>,
    pub bds_5_0_timestamp: Option<DateTime<Utc>>,
    pub temperature: Option<f64>,
    pub wind: Option<(u32, u32)>,
    pub turbulence: Option<u32>,
    pub humidity: Option<u32>,
    pub pressure: Option<u32>,
    pub timestamp: DateTime<Utc>,
    pub position_timestamp: Option<DateTime<Utc>>,
    pub track_timestamp: Option<DateTime<Utc>>,
    pub heading_timestamp: Option<DateTime<Utc>>,
    pub last_type_code: u32,
    pub last_df: u32,
    pub adsb_version: Option<u32>,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            icao: 0,
            capability: (0, Capability::default()),
            category: (0, 0),
            reg: "",
            ais: None,
            altitude: None,
            altitude_gnss: None,
            altitude_source: ' ',
            squawk: None,
            surveillance_status: ' ',
            threat_encounter: None,
            vrate: None,
            vrate_source: '_',
            cpr_lat: [0, 0],
            cpr_lon: [0, 0],
            cpr_time: [Utc::now(), Utc::now()],
            lat: 0.0,
            lon: 0.0,
            grspeed: None,
            true_airspeed: None,
            indicated_airspeed: None,
            mach_number: None,
            ground_movement: None,
            turn: 0,
            track: None,
            track_source: ' ',
            heading: None,
            heading_source: ' ',
            roll_angle: None,
            track_angle_rate: None,
            bds_5_0_timestamp: None,
            temperature: None,
            wind: None,
            turbulence: None,
            humidity: None,
            pressure: None,
            timestamp: Utc::now(),
            position_timestamp: None,
            track_timestamp: None,
            heading_timestamp: None,
            last_type_code: 0,
            last_df: 0,
            adsb_version: None,
        }
    }

    pub fn from_message(message: &[u32], df: u32, icao: u32, strict: bool) -> Self {
        let mut plane = Plane::new();
        plane.icao = icao;
        (_, plane.reg) = crate::country::icao_to_country(icao);
        plane.update(message, df, strict);
        plane
    }

    pub fn update(&mut self, message: &[u32], df: u32, strict: bool) {
        self.timestamp = Utc::now();
        self.last_df = df;

        if df == 4 {
            if let Some(altitude) = adsb::altitude(message, df) {
                if altitude > 1000000 {
                    error!(
                        "DF:{} ALT:{} ERR: {} ICAO:{}, M:{:?}",
                        df,
                        self.altitude.unwrap_or(0),
                        altitude,
                        self.icao,
                        message
                    );
                } else {
                    self.altitude = Some(altitude);
                }
            }
            self.altitude_source = ' ';
        }
        if df == 5 || df == 21 {
            self.squawk = adsb::squawk(message);
        }

        if df == 11 || df == 17 {
            self.capability.0 = adsb::ca(message);
        }
        if df == 17 || df == 18 {
            let (message_type, message_subtype) = adsb::message_type(message);
            self.last_type_code = message_type;
            debug!("DF:{}, TC:{}, ST:{}", df, message_type, message_subtype);
            match message_type {
                1..=4 => {
                    self.ais = adsb::ais(message);
                    self.category = (message_type, message_subtype);
                }
                5..=18 => {
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
                        self.altitude = None;
                        self.altitude_source = '\u{2070}';
                    }
                    if let 9..=18 = message_type {
                        self.altitude = adsb::altitude(message, df);
                        self.altitude_source = ' ';
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
                                self.track_source = ' ';
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
                    self.vrate_source = ' ';
                    if let Some(altitude) = self.altitude {
                        if let Some(altitude_delta) = adsb::altitude_delta(message) {
                            self.altitude_gnss = Some((altitude as i32 + altitude_delta) as u32);
                        }
                    }
                    match message_subtype {
                        1 => {
                            (self.track, self.grspeed) =
                                adsb::track_and_groundspeed(message, false);
                            self.track_source = '\u{2081}';
                        }
                        2 => {
                            // 4 knots units for supersonic
                            (self.track, self.grspeed) = adsb::track_and_groundspeed(message, true);
                            self.track_source = '\u{2082}';
                        }
                        3 | 4 => {
                            self.track = adsb::heading(message);
                            self.track_source = '\u{2083}';
                            self.altitude_source = '"';
                        }
                        _ => {}
                    }
                }
                20..=22 => {
                    self.altitude_gnss = adsb::altitude_gnss(message);
                    self.surveillance_status = adsb::surveillance_status(message);
                }
                31 => {
                    self.adsb_version = adsb::version(message);
                }
                _ => {}
            }
        }
        if (!strict || (self.capability.0 > 3)) && (df == 20 || df == 21) {
            let mut bds = adsb::bds(message);
            if bds == (2, 0) {
                self.ais = adsb::ais(message);
            }
            if bds == (3, 0) {
                self.threat_encounter = adsb::threat_encounter(message);
            }
            if bds == (0, 0) {
                if let Some(result) = adsb::is_bds_1_7(message) {
                    self.capability.1 = result;
                    bds = (1, 7);
                    debug!(
                        "DF:{}, BDS:{}.{}, C:{:b} 4:{} 4.4:{} 5:{} 6:{}",
                        df,
                        bds.0,
                        bds.1,
                        self.capability.1.flags,
                        self.capability.1.bds40,
                        self.capability.1.bds40,
                        self.capability.1.bds50,
                        self.capability.1.bds60
                    );
                }
            }
            if bds == (0, 0) && (!strict || self.capability.1.bds50) {
                if let Some(result) = adsb::is_bds_5_0(message) {
                    self.roll_angle = result.roll_angle;
                    self.track = result.track_angle;
                    self.track_angle_rate = result.track_angle_rate;
                    self.grspeed = result.ground_speed;
                    self.true_airspeed = result.true_airspeed;
                    self.bds_5_0_timestamp = Some(self.timestamp);
                    self.track_source = '\u{2085}';
                    self.track_timestamp = Some(Utc::now());
                    bds = (5, 0);
                    debug!("DF:{}, BDS:{}.{}", df, bds.0, bds.1);
                }
            }
            if bds == (0, 0) && (!strict || self.capability.1.bds60) {
                if let Some(result) = adsb::is_bds_6_0(message) {
                    self.heading = result.magnetic_heading;
                    self.indicated_airspeed = result.indicated_airspeed;
                    self.mach_number = result.mach_number;
                    self.vrate = result.internal_vertical_velocity;
                    self.vrate_source = '\u{2086}';
                    self.heading_source = '\u{2086}';
                    self.heading_timestamp = Some(Utc::now());
                    bds = (6, 0);
                    debug!("DF:{}, BDS:{}.{}", df, bds.0, bds.1);
                }
            }
            if bds == (0, 0) {
                if let Some(meteo) = is_bds_4_4(message) {
                    self.temperature = meteo.temp;
                    self.wind = meteo.wind;
                    self.humidity = meteo.humidity;
                    self.turbulence = meteo.turbulence;
                    self.pressure = meteo.pressure;
                    bds = (4, 4);
                }
            }
            if bds == (4, 5) {
                if let Some(temp) = adsb::temperature_4_5(message) {
                    if temp > 45.0 {
                        error!("DF:{}, BDS:{}.{}, T:{}", df, bds.0, bds.1, temp);
                    } else {
                        self.temperature = Some(temp);
                    }
                }
            }
            debug!("DF:{} BDS:{}.{}", df, bds.0, bds.1);
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
        if let Some(altitude) = self.altitude {
            write!(f, " Alt: {:>5}", altitude)?;
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
            write!(f, " GSpd: {:>3.0}", grspeed)?;
        } else {
            write!(f, " {:19}", "")?;
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
    fn simple_display(
        &self,
        f: &mut fmt::Formatter,
        weather: bool,
        angles: bool,
        speed: bool,
        extra: bool,
    ) -> fmt::Result;
}

impl SimpleDisplay for Plane {
    fn simple_display(
        &self,
        f: &mut fmt::Formatter,
        weather: bool,
        angles: bool,
        speed: bool,
        extra: bool,
    ) -> fmt::Result {
        write!(f, "{:06X}", self.icao)?;
        write!(f, " {:2}", self.reg)?;
        write!(f, "{}", self.altitude_source)?;
        if let Some(squawk) = self.squawk {
            write!(f, "{:04}", squawk)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(threat_encounter) = self.threat_encounter {
            write!(f, "{}", threat_encounter)?;
        } else {
            write!(f, " ")?;
        }
        if let Some(w) = adsb::icao_wtc(&self.category) {
            write!(f, "{}", w)?;
        } else {
            write!(f, " ")?;
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
        if let Some(altitude) = self.altitude {
            write!(f, " {:>5}", altitude)?;
        } else {
            write!(f, " {:5}", "")?;
        }
        if let Some(vrate) = self.vrate {
            write!(f, " {:>5}", vrate)?;
            write!(f, "{:1}", self.vrate_source)?;
        } else {
            write!(f, " {:6}", "")?;
        }
        if let Some(track) = self.track {
            write!(f, "{:>3.0}", track)?;
            write!(f, "{:}", self.track_source)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(heading) = self.heading {
            write!(f, "{:>3.0}", heading)?;
            write!(f, "{:}", self.heading_source)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(grspeed) = self.grspeed {
            write!(f, "{:>3.0}", grspeed)?;
        } else {
            write!(f, "{:3}", "")?;
        }
        if speed {
            if let Some(tas) = self.true_airspeed {
                write!(f, " {:>3}", tas)?;
            } else {
                write!(f, " {:3}", "")?;
            }
            if let Some(ias) = self.indicated_airspeed {
                write!(f, " {:>3}", ias)?;
            } else {
                write!(f, " {:3}", "")?;
            }
            if let Some(mn) = self.mach_number {
                write!(f, " {:>4.2}", mn)?;
            } else {
                write!(f, " {:4}", "")?;
            }
        }
        if angles {
            if let Some(roll_angle) = self.roll_angle {
                write!(f, " {:>3}", roll_angle)?;
            } else {
                write!(f, " {:3}", "")?;
            }
            if let Some(track_angle_rate) = self.track_angle_rate {
                write!(f, " {:>3}", track_angle_rate)?;
            } else {
                write!(f, " {:3}", "")?;
            }
            if let Some(altitude_gnss) = self.altitude_gnss {
                write!(f, " {:>5}", altitude_gnss)?;
            } else {
                write!(f, " {:5}", "")?;
            }
        }
        if weather {
            if let Some(temperature) = self.temperature {
                write!(f, " {:>5.1}", temperature)?;
            } else {
                write!(f, " {:5}", "")?;
            }
            if let Some(wind) = self.wind {
                write!(f, " {:>3}", wind.0)?;
                write!(f, " {:>3}", wind.1)?;
            } else {
                write!(f, " {:7}", "")?;
            }
            if let Some(humidity) = self.humidity {
                write!(f, " {:>3}", humidity)?;
            } else {
                write!(f, " {:3}", "")?;
            }
            if let Some(pressure) = self.pressure {
                write!(f, " {:>4}", pressure)?;
            } else {
                write!(f, " {:4}", "")?;
            }
            if let Some(turbulence) = self.turbulence {
                write!(f, " {:>2}", turbulence)?;
            } else {
                write!(f, " {:2}", "")?;
            }
        }
        if extra {
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
                    " {:x}",
                    Utc::now()
                        .signed_duration_since(position_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, "  ")?;
            }
            if let Some(track_timestamp) = self.track_timestamp {
                write!(
                    f,
                    "{:x}",
                    Utc::now()
                        .signed_duration_since(track_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, " ")?;
            }
            if let Some(heading_timestamp) = self.heading_timestamp {
                write!(
                    f,
                    "{:x}",
                    Utc::now()
                        .signed_duration_since(heading_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, " ")?;
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

pub struct SimpleDisplayWrapper<'a, T: SimpleDisplay>(&'a T, bool, bool, bool, bool);

impl<'a, T: SimpleDisplay> fmt::Display for SimpleDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.simple_display(f, self.1, self.2, self.3, self.4)
    }
}

pub fn format_simple_display<T: SimpleDisplay>(
    item: &T,
    weather: bool,
    angles: bool,
    speed: bool,
    extra: bool,
) -> String {
    format!(
        "{}",
        SimpleDisplayWrapper(item, weather, angles, speed, extra)
    )
}

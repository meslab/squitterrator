use super::Plane;
use crate::decoder::{Ext, Mds, Srt, DF};
use chrono::Utc;

impl Plane {
    pub fn from_downlink(&mut self, df: DF, relaxed: bool) {
        self.timestamp = Utc::now();
        match df {
            DF::SRT(dl) => self.from_srt(dl, relaxed),
            DF::EXT(dl) => self.from_ext(dl, relaxed),
            DF::MDS(dl) => self.from_mds(dl, relaxed),
        }
    }

    pub fn from_srt(&mut self, dl: Srt, _relaxed: bool) {
        if let Some(v) = dl.df {
            self.last_df = v;
        }
        if let Some(v) = dl.icao {
            self.icao = v;
        }
        if dl.squawk.is_some() {
            self.squawk = dl.squawk;
        }
        if let Some(v) = dl.capability {
            self.capability.0 = v;
        }
        if dl.altitude.is_some() {
            self.altitude = dl.altitude;
        }
    }

    pub fn from_ext(&mut self, _dl: Ext, _relaxed: bool) {}
    pub fn from_mds(&mut self, _dl: Mds, _relaxed: bool) {}

    /*
    self.last_df = df;

    if df == 4 || df == 20 {
        self.altitude = decoder::altitude(message, df);
        self.altitude_source = ' ';
    }
    if df == 5 || df == 21 {
        self.squawk = decoder::squawk(message);
    }

    if df == 11 || df == 17 {
        self.capability.0 = decoder::ca(message);
    }
    if df == 17 || df == 18 {
        let (message_type, message_subtype) = decoder::message_type(message);
        self.last_type_code = message_type;
        debug!("DF:{}, TC:{}, ST:{}", df, message_type, message_subtype);
        match message_type {
            1..=4 => {
                self.ais = decoder::ais(message);
                self.category = (message_type, message_subtype);
            }
            5..=18 => {
                if let 5..=8 = message_type {
                    self.ground_movement = decoder::ground_movement(message);
                    self.altitude = None;
                    self.altitude_source = '\u{2070}';
                    self.track = decoder::ground_track(message);
                    self.track_source = ' ';
                }
                if let 9..=18 = message_type {
                    self.altitude = decoder::altitude(message, df);
                    self.altitude_source = ' ';
                    self.surveillance_status = decoder::surveillance_status(message);
                }
                if let Some((cpr_form, cpr_lat, cpr_lon)) = decoder::cpr(message) {
                    match cpr_form {
                        0 | 1 => {
                            self.cpr_lat[cpr_form as usize] = cpr_lat;
                            self.cpr_lon[cpr_form as usize] = cpr_lon;
                            self.cpr_time[cpr_form as usize] = Utc::now();
                        }
                        _ => {}
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
                                decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 4)
                            }
                            9..=18 => {
                                decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 1)
                            }
                            _ => None,
                        } {
                            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)
                            {
                                self.lat = lat;
                                self.lon = lon;
                                self.position_timestamp = Some(Utc::now());
                            }
                        }
                    }
                }
            }
            19 => {
                self.vrate = decoder::vertical_rate(message);
                self.vrate_source = ' ';
                if let Some(altitude) = self.altitude {
                    if let Some(altitude_delta) = decoder::altitude_delta(message) {
                        self.altitude_gnss = Some((altitude as i32 + altitude_delta) as u32);
                    }
                }
                match message_subtype {
                    1 => {
                        (self.track, self.grspeed) =
                            decoder::track_and_groundspeed(message, false);
                        self.track_source = '\u{2081}';
                    }
                    2 => {
                        // 4 knots units for supersonic
                        (self.track, self.grspeed) =
                            decoder::track_and_groundspeed(message, true);
                        self.track_source = '\u{2082}';
                    }
                    3 | 4 => {
                        self.heading = decoder::heading(message);
                        self.heading_source = '\u{2083}';
                        self.altitude_source = '"';
                    }
                    _ => {}
                }
            }
            20..=22 => {
                self.altitude_gnss = decoder::altitude_gnss(message);
                self.surveillance_status = decoder::surveillance_status(message);
            }
            31 => {
                self.adsb_version = decoder::version(message);
            }
            _ => {}
        }
    }
    if (relaxed || (self.capability.0 > 3)) && (df == 20 || df == 21) {
        let mut bds = decoder::bds(message);
        if bds == (2, 0) {
            self.ais = decoder::ais(message);
        }
        if bds == (3, 0) {
            self.threat_encounter = decoder::threat_encounter(message);
        }
        if bds == (0, 0) {
            if let Some(result) = decoder::is_bds_1_7(message) {
                self.capability.1 = result;
                bds = (1, 7);
                debug!("Relaxed:{}", relaxed);
                debug!(
                    "DF:{}, BDS:{}.{}, C:{:b} 4:{} 4.4:{} 5:{} 6:{}",
                    df,
                    bds.0,
                    bds.1,
                    self.capability.1.flags,
                    self.capability.1.bds40,
                    self.capability.1.bds44,
                    self.capability.1.bds50,
                    self.capability.1.bds60
                );
            }
        }
        if bds == (0, 0) && (relaxed || self.capability.1.bds40) {
            if let Some(value) = decoder::is_bds_4_0(message) {
                self.selected_altitude =
                    value.mcp_selected_altitude.or(value.fms_selected_altitude);
                self.target_altitude_source = match value.target_altitude_source {
                    Some(v) => match v {
                        1 => '\u{2081}',
                        2 => '\u{2082}',
                        3 => '\u{2083}',
                        _ => ' ',
                    },
                    _ => ' ',
                };
                self.barometric_pressure_setting = value.barometric_pressure_setting;
                bds = (4, 0);
                debug!(
                    "DF:{}, BDS:{}.{} S:{}",
                    df,
                    bds.0,
                    bds.1,
                    value.target_altitude_source.unwrap_or(0)
                );
            }
        }
        if bds == (0, 0) && (relaxed || self.capability.1.bds50) {
            if let Some(result) = decoder::is_bds_5_0(message) {
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
        if bds == (0, 0) && (relaxed || self.capability.1.bds60) {
            if let Some(result) = decoder::is_bds_6_0(message) {
                self.heading = result.magnetic_heading;
                self.indicated_airspeed = result.indicated_airspeed;
                self.mach_number = result.mach_number;
                self.vrate = match result.barometric_altitude_rate.is_some() {
                    true => {
                        self.vrate_source = '\u{2086}';
                        result.barometric_altitude_rate
                    }
                    _ => {
                        self.vrate_source = '\u{2071}';
                        result.internal_vertical_velocity
                    }
                };
                self.heading_source = '\u{2086}';
                self.heading_timestamp = Some(Utc::now());
                bds = (6, 0);
                debug!("DF:{}, BDS:{}.{}", df, bds.0, bds.1);
            }
        }
        if bds == (0, 0) {
            if let Some(meteo) = decoder::is_bds_4_4(message) {
                self.temperature = meteo.temp;
                if meteo.wind.is_some() {
                    self.wind = meteo.wind;
                }
                self.humidity = meteo.humidity;
                self.turbulence = meteo.turbulence;
                self.pressure = meteo.pressure;
                bds = (4, 4);
                debug!("DF:{} B:4.4 FOM:{:b}", df, message[8] & 0xF);
            }
        }
        if bds == (4, 5) {
            self.temperature = decoder::is_bds_4_5(message);
        }
        debug!("DF:{} BDS:{}.{}", df, bds.0, bds.1);
    }*/
}

use super::Plane;
use crate::decoder;
use chrono::Utc;
use log::debug;

impl Plane {
    pub fn update(&mut self, message: &[u32], df: u32, relaxed: bool) {
        self.timestamp = Utc::now();
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
                    self.update_from_ext_1_4(message, message_type, message_subtype);
                }
                5..=8 => {
                    self.update_from_ext_5_8(message, message_type);
                }
                9..=18 => {
                    self.update_from_ext_9_18(message, message_type, df);
                }
                19 => {
                    self.update_from_ext_19(message, message_subtype);
                }
                20..=22 => {
                    self.update_from_ext_20_22(message);
                }
                31 => {
                    self.update_from_ext_31(message);
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
                    self.track_timestamp = Some(self.timestamp);
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
                    self.heading_timestamp = Some(self.timestamp);
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
        }
    }
}

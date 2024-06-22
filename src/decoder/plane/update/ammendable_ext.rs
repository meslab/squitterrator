use super::Plane;
use crate::decoder::{Ammendable, Ext};

impl Ammendable<Ext> for Plane {
    fn ammend(&mut self, dl: &Ext) {
        if dl.icao.is_some() {
            self.last_type_code = dl.message_type.0;
            match dl.message_type.0 {
                1..=4 => {
                    self.ammend_from_ext_1_4(dl);
                }
                5..=8 => {
                    self.ammend_from_ext_5_8(dl);
                }
                9..=18 => {
                    self.ammend_from_ext_9_18(dl);
                }
                19 => {
                    self.ammend_from_ext_19(dl);
                }
                20..=22 => {
                    self.ammend_from_ext_20_22(dl);
                }
                31 => {
                    self.ammend_from_ext_31(dl);
                }
                _ => {}
            }
        }
    }
}

impl Plane {
    fn ammend_from_ext_1_4(&mut self, dl: &Ext) {
        if dl.ais.is_some() {
            self.ais.clone_from(&dl.ais);
            self.category = dl.message_type;
        }
    }

    fn ammend_from_ext_5_8(&mut self, dl: &Ext) {
        self.ground_movement = dl.ground_movement;
        self.altitude = dl.altitude;
        self.altitude_source = '\u{2070}';
        self.track = dl.track;
        self.track_source = dl.track_source.unwrap_or(' ');
        self.ammend_cpr(dl);
    }

    fn ammend_from_ext_9_18(&mut self, dl: &Ext) {
        self.altitude = dl.altitude;
        self.altitude_source = ' ';
        self.surveillance_status = dl.surveillance_status.unwrap_or(' ');
        self.ammend_cpr(dl);
    }

    fn ammend_from_ext_19(&mut self, dl: &Ext) {
        self.vrate = dl.vrate;
        self.vrate_source = ' ';
        if let Some(altitude_delta) = dl.altitude_delta {
            if let Some(altitude) = self.altitude {
                self.altitude_gnss = Some((altitude as i32 + altitude_delta) as u32);
            }
        }
        match dl.message_type.1 {
            1 => {
                (self.track, self.grspeed) = (self.track, self.grspeed);
                self.track_source = '\u{2081}';
            }
            2 => {
                (self.track, self.grspeed) = (self.track, self.grspeed);
                self.track_source = '\u{2082}';
            }
            3 | 4 => {
                self.heading = dl.heading;
                self.heading_source = '\u{2083}';
                self.altitude_source = '"';
            }
            _ => {}
        }
    }

    fn ammend_from_ext_20_22(&mut self, dl: &Ext) {
        self.altitude_gnss = dl.altitude_gnss;
        self.surveillance_status = dl.surveillance_status.unwrap_or(' ');
    }

    fn ammend_from_ext_31(&mut self, dl: &Ext) {
        self.adsb_version = dl.adsb_version;
    }

    fn ammend_cpr(&mut self, dl: &Ext) {
        if let Some((cpr_form, cpr_lat, cpr_lon)) = dl.cpr {
            self.cpr_lat[cpr_form as usize] = cpr_lat;
            self.cpr_lon[cpr_form as usize] = cpr_lon;
            self.cpr_time[cpr_form as usize] = self.timestamp;

            self.update_position(dl.message_type.0, cpr_form);
        }
    }
}

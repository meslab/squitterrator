use crate::decoder::{Ext, Mds, Srt, DF};

use super::Plane;
use chrono::Utc;
mod update_from_bcast;
mod update_from_ext;
mod update_from_mode_s;
mod update_position;

impl Plane {
    pub fn update(&mut self, message: &[u32], df: u32, relaxed: bool) {
        self.timestamp = Utc::now();
        self.last_df = df;

        self.update_from_bcast(message, df);

        if df == 17 || df == 18 {
            self.update_from_ext(message, df);
        }

        if (relaxed || (self.capability.0 > 3)) && (df == 20 || df == 21) {
            self.update_from_mode_s(message, df, relaxed);
        }
    }
}

pub trait Ammendable<T> {
    fn ammend(&mut self, dl: &T);
}

impl Ammendable<Srt> for Plane {
    fn ammend(&mut self, dl: &Srt) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

impl Ammendable<Ext> for Plane {
    fn ammend(&mut self, dl: &Ext) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

impl Ammendable<Mds> for Plane {
    fn ammend(&mut self, dl: &Mds) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

impl Ammendable<DF> for Plane {
    fn ammend(&mut self, dl: &DF) {
        match dl {
            DF::SRT(v) => self.icao = v.icao.unwrap(),
            DF::EXT(v) => self.icao = v.icao.unwrap(),
            DF::MDS(v) => self.icao = v.icao.unwrap(),
        }
    }
}

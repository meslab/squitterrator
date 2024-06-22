use crate::decoder::{plane::ammend::Ammendable, Plane, Srt};

impl Ammendable<Srt> for Plane {
    fn ammend(&mut self, dl: &Srt) {
        if dl.icao.is_some() {
            if dl.df == Some(5) && dl.altitude.is_some() {
                self.altitude = dl.altitude;
                self.altitude_source = ' ';
            }
            if dl.df == Some(5) && dl.squawk.is_some() {
                self.squawk = dl.squawk;
            }
            if dl.df == Some(11) {
                if let Some(v) = dl.capability {
                    self.capability.0 = v;
                }
            }
        }
    }
}

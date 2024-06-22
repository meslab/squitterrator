mod ammendable_ext;
mod ammendable_mds;
mod ammendable_srt;

use crate::decoder::{Plane, DF};

pub trait Ammendable<T> {
    fn ammend(&mut self, dl: &T);
}

impl Ammendable<DF> for Plane {
    fn ammend(&mut self, dl: &DF) {
        match dl {
            DF::SRT(v) => self.ammend(v),
            DF::EXT(v) => self.ammend(v),
            DF::MDS(v) => self.ammend(v),
        }
    }
}

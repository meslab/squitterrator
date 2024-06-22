mod amendable_ext;
mod amendable_mds;
mod amendable_srt;

use crate::decoder::{Plane, DF};

pub trait Amendable<T> {
    fn amend(&mut self, dl: &T);
}

impl Amendable<DF> for Plane {
    fn amend(&mut self, dl: &DF) {
        match dl {
            DF::SRT(v) => self.amend(v),
            DF::EXT(v) => self.amend(v),
            DF::MDS(v) => self.amend(v),
        }
    }
}

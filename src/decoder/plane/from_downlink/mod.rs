mod from_ext;
mod from_mds;
mod from_srt;

use crate::decoder::{Plane, DF};

pub trait UpdateFromDownlink<T> {
    fn amend(&mut self, dl: &T);
}

impl UpdateFromDownlink<DF> for Plane {
    fn amend(&mut self, dl: &DF) {
        match dl {
            DF::SRT(v) => self.amend(v),
            DF::EXT(v) => self.amend(v),
            DF::MDS(v) => self.amend(v),
        }
    }
}

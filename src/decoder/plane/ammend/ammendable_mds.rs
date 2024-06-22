use crate::decoder::{plane::ammend::Ammendable, Mds, Plane};

impl Ammendable<Mds> for Plane {
    fn ammend(&mut self, dl: &Mds) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

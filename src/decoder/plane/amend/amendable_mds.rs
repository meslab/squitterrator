use crate::decoder::{plane::amend::Amendable, Mds, Plane};

impl Amendable<Mds> for Plane {
    fn amend(&mut self, dl: &Mds) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

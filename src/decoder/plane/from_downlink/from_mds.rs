use crate::decoder::{plane::from_downlink::UpdateFromDownlink, Mds, Plane};

impl UpdateFromDownlink<Mds> for Plane {
    fn update_from_downlink(&mut self, dl: &Mds) {
        if let Some(v) = dl.icao {
            self.icao = v;
        }
    }
}

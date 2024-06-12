pub struct Df20 {
    pub icao: u32,
    pub altitude: Option<u32>,
}

impl Df20 {
    pub fn new(icao: u32) -> Self {
        Df20 {
            icao,
            altitude: None,
        }
    }
}

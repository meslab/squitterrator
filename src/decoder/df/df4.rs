pub struct Df4 {
    pub icao: u32,
    pub altitude: Option<u32>,
}

impl Df4 {
    pub fn new(icao: u32) -> Self {
        Df4 {
            icao,
            altitude: None,
        }
    }
}

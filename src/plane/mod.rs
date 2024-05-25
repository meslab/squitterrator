pub struct Plane {
    pub ais: String,
    pub alt: u32,
    pub squawk: u32,
    pub vsign: u32,
    pub vrate: i32,
    pub cpr_form: u32,
    pub cpr_lat: [u32; 2],
    pub cpr_long: [u32; 2],
    pub lat: f64,
    pub lon: f64,
    pub sp_west: i32,
    pub sp_south: i32,
    pub grspeed: f64,
    pub airspeed: u32,
    pub heading: f64,
    pub turn: u32,
    pub track: f64,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            ais: "".to_string(),
            alt: 0,
            squawk: 0,
            vsign: 0,
            vrate: 0,
            cpr_form: 2,
            cpr_lat: [0, 0],
            cpr_long: [0, 0],
            lat: 0.0,
            lon: 0.0,
            sp_west: 0,
            sp_south: 0,
            grspeed: 0.0,
            airspeed: 0,
            heading: 0.0,
            turn: 0,
            track: 0.0,
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

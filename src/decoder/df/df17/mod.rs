mod display;
mod update;

#[derive(Debug)]
pub struct Df17 {
    pub icao: Option<u32>,
    pub capability: u32,
    pub message_type: (u32, u32),
    pub ais: Option<String>,
    pub category: Option<(u32, u32)>,
    pub cpr: Option<(u32, u32, u32)>,
    pub ground_movement: Option<f64>,
    pub track: Option<u32>,
    pub track_source: Option<char>,
    pub altitude: Option<u32>,
    pub altitude_source: Option<char>,
    pub surveillance_status: Option<char>,
    pub adsb_version: Option<u32>,
}

impl Default for Df17 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df17 {
    pub fn new() -> Self {
        Df17 {
            icao: None,
            capability: 0,
            message_type: (0, 0),
            ais: None,
            category: None,
            cpr: None,
            ground_movement: None,
            track: None,
            track_source: None,
            altitude: None,
            altitude_source: None,
            surveillance_status: None,
            adsb_version: None,
        }
    }
}

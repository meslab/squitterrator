use crate::adsb::{ais, icao, mode_e_decoded_message};
use crate::country::icao_to_country;
use log::debug;

pub fn generate_ais(message: &[u32], squitter: &str) {
    if let Some(result) = ais(message) {
        println!("(\"{}\", \"{}\"),", squitter, result);
    }
}

pub fn squitter_decode(message: &[u32], df: u32) {
    if let Some(r) = mode_e_decoded_message(message, df) {
        println!(
            "DF:{:>2}, Alt:{:>5}, AIS:{:8}, Vs:{}, Vr:{:>5}, F:{}, Lat:{:>6}, Lon:{:>6}, W:{}, S:{}, Gs:{}, As:{}, H:{}, Tu:{}, Tr:{}",
            df,
            r.alt.unwrap_or(0),
            r.ais.unwrap_or("".to_string()),
            r.vsign,
            r.vrate,
            r.cpr_form,
            r.cpr_lat,
            r.cpr_long,
            r.sp_west,
            r.sp_south,
            r.grspeed,
            r.airspeed,
            r.heading,
            r.turn,
            r.track
        )
    }
}

pub fn icao_decode(message: &[u32], df: u32, squitter: &str) {
    if let Some(icao_address) = icao(message, df) {
        debug!("Squitter: {}, M: {:?}", squitter, message);
        let (country, cshrt) = icao_to_country(icao_address);
        println!(
            "Squitter: {:28}, ICAO: {:06X}, DF: {:2}, {}: {}",
            squitter, icao_address, df, cshrt, country,
        );
    }
}

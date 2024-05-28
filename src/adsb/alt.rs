use crate::adsb::{graytobin, ma_code, me_code};

pub fn alt(message: &[u32], df: u32) -> Option<u32> {
    let code = match df {
        17 => me_code(message),
        _ => ma_code(message),
    };

    match code {
        Some(code) => match code & 0b10 {
            0 => match code & 1 {
                0 => {
                    let (high, low) = graytobin(message);
                    Some(high * 500 + low * 100 + 1200)
                }
                _ => Some((((code >> 7) << 4) | ((code >> 2) & 0b1111)) as u32 * 25 + 1000),
            },
            _ => Some(
                ((((code >> 7) << 4) & 0b11111110000 | (code >> 2) & 0b1111) as f32 * 0.31) as u32,
            ),
        },
        None => None,
    }
}

pub fn alt_gnss(message: &[u32]) -> Option<u32> {
    Some(((message[12] & 0xF) << 8 | (message[13] & 0xF) << 4) | (message[14] & 0xF))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb;

    #[test]
    fn test_alt() {
        if let Some(message) = adsb::message("A8281200200464B3CF7820CD194C") {
            let df = adsb::df(&message);
            let result = alt(&message, df);
            assert_eq!(result, Some(16700));
        }
    }
}

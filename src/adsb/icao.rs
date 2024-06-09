use crate::adsb::get_crc;

/// Calculates the ICAO address based on the given ADS-B message and downlink format (DF).
///
/// # Arguments
///
/// * `message` - The ADS-B message as an array of 32-bit unsigned integers.
/// * `df` - The downlink format (DF) of the ADS-B message.
///
/// # Returns
///
/// The calculated ICAO address as a 32-bit unsigned integer.
///
/// # Examples
///
/// ```
/// use squitterator::adsb::{message, df, icao};
/// if let Some(message) = message("A0001838300000000000007ADA59") {
///     if let Some(df) = df(&message) {
///         if let Some(icao_address) = icao(&message, df) {
///             assert_eq!(icao_address, 7453696);
///         }
///     }
/// }
/// ```

pub fn icao(message: &[u32], df: u32) -> Option<u32> {
    let crc = get_crc(message, df);
    let icao = match df {
        0 | 4 | 5 | 16 | 20 | 21 => {
            let len = (message.len() * 4) as u32;
            if let Some(result) = crate::adsb::flag_and_range_value(message, 0, len - 23, len) {
                result.1 ^ crc
            } else {
                0
            }
        }
        _ => {
            if let Some((_, value)) = crate::adsb::flag_and_range_value(message, 0, 9, 32) {
                value
            } else {
                0
            }
        }
    };
    match icao {
        0 => None,
        _ => Some(icao),
    }
}

/// Calculates the Wake Turbulence Category (WTC) based on the given VDL Mode 2 Code (VC).
/// The WTC is used to determine the separation minima between aircraft.
///
/// # Arguments
///
/// * `vc` - The VDL Mode 2 Code (VC) as a tuple of two 32-bit unsigned integers.
///
/// # Returns
///
/// The calculated Wake Turbulence Category (WTC) as a character.
///
pub fn icao_wtc(vc: &(u32, u32)) -> Option<char> {
    match vc {
        (4, 1) => Some('L'),
        (4, 2) => Some('S'),
        (4, 3) => Some('M'),
        (4, 4) => Some('H'),
        (4, 5) => Some('J'),
        (4, 7) => Some('R'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::adsb::{df, icao, message};

    #[test]
    fn test_icao() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 4219421),
            ("A0001838300000000000007ADA59", 7453696),
            ("A800161110010080E6000073D501", 7453696),
            ("A800120110010080F600001AFEDD", 4921598),
            ("8D71BC009901DC93C0070788AE4B", 7453696),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 7453696),
            ("8DA7F6429B053D0000000060D7AE", 11007554),
            ("8D4B18FE68BF033F523BF5BAAAEB", 4921598),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = message(squitter) {
                let df = df(&message).unwrap();
                if let Some(result) = icao(&message, df) {
                    assert_eq!(result, *value, "Squitter: {}", squitter);
                }
            }
        }
    }

    #[test]
    fn test_icao_wtc() {
        let vcs = [
            ((4, 1), 'L'),
            ((4, 2), 'S'),
            ((4, 3), 'M'),
            ((4, 4), 'H'),
            ((4, 5), 'J'),
            ((4, 7), 'R'),
        ];

        for (vc, value) in vcs.iter() {
            if let Some(result) = crate::adsb::icao_wtc(vc) {
                assert_eq!(result, *value, "VC: {:?}", vc);
            }
        }
    }

    #[test]
    fn test_icao_wtc_none() {
        let vcs = [(4, 0), (4, 6)];

        for vc in vcs.iter() {
            assert_eq!(crate::adsb::icao_wtc(vc), None, "VC: {:?}", vc);
        }
    }
}

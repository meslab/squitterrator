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
///     let df = df(&message);
///     if let Some(icao_address) = icao(&message, df) {
///         assert_eq!(icao_address, 7453696);
///     }
/// }
/// ```

pub fn icao(message: &[u32], df: u32) -> Option<u32> {
    let crc = get_crc(message, df);
    let icao = match df {
        0 | 4 | 5 | 16 | 20 | 21 => {
            let len = message.len();
            (((message[len - 6]) << 20)
                | ((message[len - 5]) << 16)
                | ((message[len - 4]) << 12)
                | ((message[len - 3]) << 8)
                | ((message[len - 2]) << 4)
                | message[len - 1])
                ^ crc
        }
        _ => {
            message[2] << 20
                | message[3] << 16
                | message[4] << 12
                | message[5] << 8
                | message[6] << 4
                | message[7]
        }
    };
    match icao {
        0 => None,
        _ => Some(icao),
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
                let df = df(&message);
                if let Some(result) = icao(&message, df) {
                    assert_eq!(result, *value, "Squitter: {}", squitter);
                }
            }
        }
    }
}

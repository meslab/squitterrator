use log::debug;

/// Calculates the Mode A (MA) code from the given ADS-B message.
///
/// The MA code is a 12-bit code used in Mode A/C transponders to represent the aircraft's
/// identity or altitude. This function extracts the MA code from the ADS-B message and returns
/// it as an `Option<u16>`. If the extraction is successful, the MA code is returned as `Some(code)`,
/// otherwise `None` is returned.
///
/// # Arguments
///
/// * `message` - The ADS-B message as a slice of `u32` values.
///
/// # Examples
///
/// ```
/// use squitterator::adsb::message;
/// use squitterator::adsb::ma_code;
/// let squitter = "8D40621D58C382D690C8AC2863A7";
/// if let Some(message) = message(squitter) {
///     let result = ma_code(&message);
///     assert_eq!(result, Some(1141));
/// }
/// ```
pub fn ma_code(message: &[u32]) -> Option<u16> {
    let mut result = 0u16;

    let flags = [
        message[4] & 1,
        (message[5] >> 3) & 1,
        (message[5] >> 2) & 1,
        (message[5] >> 1) & 1,
        message[5] & 1,
        (message[6] >> 3) & 1,
        (message[6] >> 1) & 1,
        message[6] & 1,
        (message[7] >> 3) & 1,
        (message[7] >> 2) & 1,
        (message[7] >> 1) & 1,
        message[7] & 1,
        (message[6] >> 2) & 1,
        message[6] & 1,
    ];

    for (i, bit) in flags.iter().enumerate() {
        result |= (*bit as u16) << (13 - i);
    }

    debug!("MA code: {:016b}", result);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb::message;

    #[test]
    fn test_ma_code() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 1141),
            ("A0001838300000000000007ADA59", 12513),
            ("A800161110010080E6000073D501", 11333),
            ("A800120110010080F600001AFEDD", 9220),
            ("8D71BC009901DC93C0070788AE4B", 14336),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 14336),
            ("8DA7F6429B053D0000000060D7AE", 11274),
            ("8D4B18FE68BF033F523BF5BAAAEB", 12795),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = message(squitter) {
                let result = ma_code(&message);
                assert_eq!(result, Some(*value));
            }
        }
    }

    #[test]
    fn test_ma_code_invalid_hex() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            let result = ma_code(&message);
            assert_eq!(result, Some(1141));
        }
    }
}

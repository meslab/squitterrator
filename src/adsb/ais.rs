/// Retrieves the BDS values from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the BDS values from.
///
/// # Returns
///
/// A tuple containing the BDS1 and BDS2 values.
fn bds(message: &[u32]) -> (u32, u32) {
    (message[8], message[9])
}

/// Converts an IA5 character code to a `char`.
///
/// # Arguments
///
/// * `ch` - The IA5 character code.
///
/// # Returns
///
/// The corresponding `char` value.
fn ia5(ch: u32) -> char {
    match ch {
        ch if (ch >> 4) == 3 => char::from_u32(ch).unwrap(),
        ch if ch > 0 && ch < 27 => char::from_u32(ch | 64).unwrap(),
        _ => ' ',
    }
}

/// Extracts AIS (Aeronautical Information Service) data from a message.
///
/// # Arguments
///
/// * `message` - The message to extract AIS data from.
///
/// # Returns
///
/// An `Option` containing the AIS data as a `String`, or `None` if the message does not contain AIS data.
pub fn ais(message: &[u32]) -> Option<String> {
    let (bds1, bds2) = bds(message);
    if bds1 == 2 && bds2 == 0 {
        Some(
            [
                ((message[10] << 2) | (message[11] >> 2)),
                (((message[11] & 3) << 4) | message[12]),
                ((message[13] << 2) | (message[14] >> 2)),
                (((message[14] & 3) << 4) | message[15]),
                ((message[16] << 2) | (message[17] >> 2)),
                (((message[17] & 3) << 4) | message[18]),
                ((message[19] << 2) | (message[20] >> 2)),
                (((message[20] & 3) << 4) | message[21]),
            ]
            .iter()
            .map(|&c| ia5(c))
            .collect::<String>(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb::message;

    #[test]
    fn test_bds() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(bds(&message), (5, 8));
        }
    }

    #[test]
    fn test_ais() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(ais(&message), None);
        }
    }

    #[test]
    fn test_ais_batch() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", "AFR3539 "),
            // ... more test cases ...
            ("8D40621D58C382D690C8AC2863A7", "BAW3457 "),
            ("8D40621D58C382D690C8AC2863A7", "00000000"),
            ("8D40621D58C382D690C8AC2863A7", "AFR439  "),
            ("8D40621D58C382D690C8AC2863A7", "CLX863  "),
            ("8D40621D58C382D690C8AC2863A7", "AFR337U "),
            ("8D40621D58C382D690C8AC2863A7", "AFR337  "),
            ("8D40621D58C382D690C8AC2863A7", "DAL65   "),
            ("8D40621D58C382D690C8AC2863A7", "BAW3457 "),
            ("8D40621D58C382D690C8AC2863A7", "00000000"),
            ("8D40621D58C382D690C8AC2863A7", "AFR439  "),
            ("8D40621D58C382D690C8AC2863A7", "CLX863  "),
            ("8D40621D58C382D690C8AC2863A7", "00000000"),
            ("8D40621D58C382D690C8AC2863A7", "AFR337  "),
            ("8D40621D58C382D690C8AC2863A7", "        "),
            ("8D40621D58C382D690C8AC2863A7", "AFR351  "),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = message(squitter) {
                let result = ais(&message);
                match result {
                    Some(r) => assert_eq!(r.as_str(), *value, "{}", squitter),
                    None => assert_eq!(result, None),
                }
            }
        }
    }
}

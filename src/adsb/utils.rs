use crate::adsb;
use log::{debug, warn};

/// Converts a squitter string into a vector of u32 values.
///
/// # Arguments
///
/// * `squitter` - The squitter string to convert.
///
/// # Returns
///
/// * `Option<Vec<u32>>` - An Option vector of u32 values representing the converted squitter string.
pub fn message(squitter: &str) -> Option<Vec<u32>> {
    match adsb::clean_squitter(squitter) {
        Some(cleaned_squitter) => match cleaned_squitter.len() {
            14 | 28 => {
                let message = cleaned_squitter
                    .chars()
                    .map(|c| u32::from_str_radix(&c.to_string(), 16).unwrap())
                    .collect::<Vec<u32>>();
                debug!("Message: {:?}", message);
                let r = adsb::reminder(&message);
                match r {
                    0 => Some(message),
                    _ => {
                        warn!("{}, R:{}", cleaned_squitter, r);
                        None
                    }
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// Retrieves the message type and subtype from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the message type and subtype from.
///
/// # Returns
///
/// A tuple containing the message type and subtype.
///
/// # Examples
///
/// ```
/// use squitterator::adsb::{message, message_type};
/// let squitter = "8D40621D58C382D690C8AC2863A7";
/// if let Some(message) = message(squitter) {
///    let (message_type, message_subtype) = message_type(&message);
///    assert_eq!(message_type, 11);
///    assert_eq!(message_subtype, 0);
/// }
/// ```
pub fn message_type(message: &[u32]) -> (u32, u32) {
    ((message[8] << 1) | (message[9] >> 3), message[9] & 7)
}

/// Retrieves the IC (Interrogator Code) value from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the IC value from.
///
/// # Returns
///
/// The IC value.
pub fn ic(message: &[u32]) -> u32 {
    (message[2] << 1) | (message[3] >> 3) & 0b11111
}

/// Retrieves the CA (Capability) value from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the CA value from.
///
/// # Returns
///     
/// The CA value.
pub fn ca(message: &[u32]) -> u32 {
    message[1] & 0b0111
}

/// Calculates the positive modulo of two values.
///
/// # Arguments
///
/// * `x` - The dividend.
/// * `y` - The divisor.
///
/// # Returns
///
/// The positive modulo of the two values.
///
/// # Examples
///
/// ```
/// use squitterator::adsb::pmod;
/// let x = -5;
/// let y = 3;
/// let result = pmod(x, y);
/// assert_eq!(result, 1);
/// ```
pub fn pmod(x: i32, y: i32) -> i32 {
    let mut res = x % y;
    if res < 0 {
        res += y;
    }
    res
}

pub fn graytobin(message: &[u32]) -> (u32, u32) {
    if let Some(code) = adsb::ma_code(message) {
        let n = ((code >> 4) & 1) << 10
            | ((code >> 2) & 1) << 9
            | ((code >> 12) & 1) << 8
            | ((code >> 10) & 1) << 7
            | ((code >> 8) & 1) << 6
            | ((code >> 7) & 1) << 5
            | ((code >> 5) & 1) << 4
            | ((code >> 3) & 1) << 3
            | ((code >> 13) & 1) << 2
            | ((code >> 11) & 1) << 1
            | ((code >> 13) & 1);
        let mut mask = 0x80;
        let mut cp = false;
        let mut result = 0;
        for _ in 1..=16 {
            if (n & mask) != 0 {
                cp = !cp;
            }
            if cp {
                result |= mask;
            }
            mask >>= 1;
        }

        let sub = n & 7;
        let high = result >> 3;
        let low = match high % 2 {
            0 => match sub {
                4 => 4,
                6 => 3,
                3 => 1,
                2 => 2,
                _ => 0,
            },
            _ => match sub {
                1 => 4,
                3 => 3,
                6 => 1,
                2 => 2,
                _ => 0,
            },
        };
        (high as u32, low as u32)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        match message(squitter) {
            Some(message) => assert_eq!(
                message,
                vec![
                    8, 13, 4, 0, 6, 2, 1, 13, 5, 8, 12, 3, 8, 2, 13, 6, 9, 0, 12, 8, 10, 12, 2, 8,
                    6, 3, 10, 7
                ]
            ),
            None => panic!("Failed to convert squitter to message"),
        }
    }

    #[test]
    fn test_message_short() {
        let squitter = "02E197B00179C3";
        match message(squitter) {
            Some(message) => assert_eq!(message, vec![0, 2, 14, 1, 9, 7, 11, 0, 0, 1, 7, 9, 12, 3]),
            None => panic!("Failed to convert squitter to message"),
        }
    }

    #[test]
    fn test_ic() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(ic(&message), 8);
        }
    }

    #[test]
    fn test_ca() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(ca(&message), 5);
        }
    }

    #[test]
    fn test_pmod() {
        let x = -5;
        let y = 3;
        let result = pmod(x, y);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_message_type() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            let (message_type, message_subtype) = message_type(&message);
            assert_eq!(message_type, 11);
            assert_eq!(message_subtype, 0);
        }
    }

    #[test]
    fn test_graytobin() {
        if let Some(message) = adsb::message("A8281200200464B3CF7820CD194C") {
            let (high, low) = graytobin(&message);
            assert_eq!(high, 31);
            assert_eq!(low, 0);
        }
    }
}

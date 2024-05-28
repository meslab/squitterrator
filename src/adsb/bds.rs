/// Retrieves the BDS values from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the BDS values from.
///
/// # Returns
///
/// A tuple containing the BDS1 and BDS2 values.
pub fn bds(message: &[u32]) -> (u32, u32) {
    (message[8], message[9])
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
}

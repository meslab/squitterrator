pub fn threat_encounter(message: &[u32]) -> Option<char> {
    let multiple_threats = message[14] & 1 == 1;
    let single_threat = (message[10] >> 3) & 1 == 1;
    if multiple_threats {
        Some('!')
    } else if single_threat {
        Some('\'')
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb;
    #[test]
    fn test_threat_encounter() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = adsb::message(squitter) {
            assert_eq!(threat_encounter(&message), Some('!'));
        }
    }
}

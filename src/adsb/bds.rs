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
    if let (1, 0) = (message[8] & 0xF, message[9] & 0xF) {
        if message[10] & 0x7 == 0 && message[11] & 0xC == 0 {
            return (1, 0);
        }
    };
    if let (2, 0) = (message[8] & 0xF, message[9] & 0xF) {
        return (2, 0);
    };

    if let (3, 0) = (message[8] & 0xF, message[9] & 0xF) {
        if message[7] & 0b1100 != 0b1100
            && ((message[3] & 1) << 6 | (message[4] & 0xF) << 2 | (message[5] & 0b1100) >> 2) < 48
        {
            return (3, 0);
        }
    };

    if message[15..19].iter().all(|&x| x == 0) {
        return (1, 7);
    }
    if goodflags(message, 33, 34, 45)
        && goodflags(message, 46, 47, 58)
        && goodflags(message, 59, 60, 71)
        && !goodflags(message, 33, 72, 79)
        && !goodflags(message, 33, 84, 85)
    {
        return (4, 0);
    };

    if goodflags(message, 33, 34, 43)
        && goodflags(message, 44, 45, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        return (5, 0);
    };

    if goodflags(message, 33, 34, 44)
        && goodflags(message, 45, 46, 55)
        && goodflags(message, 56, 57, 66)
        && goodflags(message, 67, 68, 77)
        && goodflags(message, 78, 79, 88)
    {
        return (6, 0);
    };

    if let Some(temp) = crate::adsb::temperature(message) {
        if (-80.0..=60.0).contains(&temp)
            && goodflags(message, 37, 38, 55)
            && goodflags(message, 67, 68, 78)
            && goodflags(message, 67, 68, 78)
            && goodflags(message, 79, 80, 81)
            && goodflags(message, 82, 83, 88)
        {
            return (4, 4);
        };
    };
    if goodflags(message, 33, 34, 35)
        && goodflags(message, 36, 37, 38)
        && goodflags(message, 39, 40, 41)
        && goodflags(message, 42, 43, 44)
        && goodflags(message, 45, 46, 47)
        && goodflags(message, 48, 49, 58)
        && goodflags(message, 59, 60, 60)
        && goodflags(message, 71, 72, 83)
        && !goodflags(message, 33, 84, 88)
    {
        return (4, 5);
    };
    (0, 0)
}

pub fn goodflags(message: &[u32], flag: u32, sb: u32, eb: u32) -> bool {
    match flag_and_range_value(message, flag, sb, eb) {
        Some((flag, result)) => match flag {
            0 => false,
            _ => !matches!(result, 0),
        },
        None => false,
    }
}

pub fn flag_and_range_value(message: &[u32], flag: u32, sb: u32, eb: u32) -> Option<(u32, u32)> {
    let (flag_ibyte, flag_ibit) = bit_location(flag);
    let (sb_ibyte, sb_ibit) = bit_location(sb);
    let (eb_ibyte, eb_ibit) = bit_location(eb);

    if eb_ibyte < sb_ibyte || (eb_ibyte == sb_ibyte && eb_ibit < sb_ibit) {
        return None;
    }
    let result = match eb_ibyte - sb_ibyte {
        0 => (message[sb_ibyte] & (0xF >> sb_ibit)) >> (3 - eb_ibit),
        1 => {
            (message[sb_ibyte] & (0xF >> sb_ibit)) << (eb_ibit + 1)
                | (message[eb_ibyte] >> (3 - eb_ibit))
        }
        _ => {
            message[sb_ibyte + 1..eb_ibyte]
                .iter()
                .fold(message[sb_ibyte] & (0xF >> sb_ibit), |a, x| {
                    a << 4 | x & 0xF
                })
                << (eb_ibit + 1)
                | (message[eb_ibyte] >> (3 - eb_ibit))
        }
    };

    let flag = (message[flag_ibyte] >> (3 - flag_ibit)) & 1;
    match flag {
        0 => Some((flag, result)),
        _ => match result {
            0 => Some((flag, result)),
            _ => Some((flag, result)),
        },
    }
}

fn bit_location(position: u32) -> (usize, usize) {
    let ibyte: usize = ((position - 1) / 4).try_into().unwrap();
    let ibit: usize = ((position - 1) % 4).try_into().unwrap();
    (ibyte, ibit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb::message;

    #[test]
    fn test_bds() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(bds(&message), (0, 0));
        }
    }

    #[test]
    fn test_badflags() {
        let messages = [
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                36,
                (1, 8),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 16),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 17),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                34,
                37,
                (1, 1),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                38,
                (1, 32),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                39,
                (1, 64),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                40,
                (1, 128),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0],
                32,
                34,
                46,
                (1, 0b100_0000_0000_00),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
                33,
                34,
                41,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
                34,
                35,
                43,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
                35,
                36,
                44,
                (1, 0),
            ),
        ];
        for (message, f, s, e, result) in messages {
            assert_eq!(
                flag_and_range_value(&message, f, s, e),
                Some(result),
                "F:{} S:{} E:{} L:{}",
                f,
                s,
                e,
                e - s + 1
            );
        }
    }

    #[test]
    fn test_bit_location() {
        assert_eq!(bit_location(33), (8, 0));
        assert_eq!(bit_location(34), (8, 1));
        assert_eq!(bit_location(35), (8, 2));
        assert_eq!(bit_location(36), (8, 3));
        assert_eq!(bit_location(37), (9, 0));
        assert_eq!(bit_location(38), (9, 1));
        assert_eq!(bit_location(39), (9, 2));
        assert_eq!(bit_location(40), (9, 3));
        assert_eq!(bit_location(1), (0, 0));
        assert_eq!(bit_location(57), (14, 0));
        assert_eq!(bit_location(88), (21, 3));
    }
}

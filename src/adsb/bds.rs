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
    if let Some(temp) = crate::adsb::temperature(message) {
        if (-80.0..=60.0).contains(&temp) {
            return (4, 4);
        };
    };
    (0, 0)
}

pub fn badflags(
    message: &[u32],
    flag: u32,
    sb: u32,
    eb: u32,
) -> (bool, u32, u32, usize, usize, usize, usize) {
    let (flag_ibyte, flag_ibit) = bit_location(flag);
    let (sb_ibyte, sb_ibit) = bit_location(sb);
    let (eb_ibyte, eb_ibit) = bit_location(eb);

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

    let flag = message[flag_ibyte] >> (3 - flag_ibit);
    match flag {
        0 => (true, flag, result, sb_ibyte, sb_ibit, eb_ibyte, eb_ibit),
        _ => match result {
            0 => (true, flag, result, sb_ibyte, sb_ibit, eb_ibyte, eb_ibit),
            _ => (false, flag, result, sb_ibyte, sb_ibit, eb_ibyte, eb_ibit),
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
                (true, 1, 0, 8, 0, 9, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                36,
                (false, 1, 8, 8, 0, 8, 3),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                37,
                (false, 1, 16, 8, 0, 9, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                33,
                37,
                (false, 1, 17, 8, 0, 9, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                34,
                37,
                (false, 1, 1, 8, 1, 9, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                38,
                (false, 1, 32, 8, 0, 9, 1),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                39,
                (false, 1, 64, 8, 0, 9, 2),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                40,
                (false, 1, 128, 8, 0, 9, 3),
            ),
            //(
            //    [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
            //    32,
            //    33,
            //    37,
            //    (false, 1, 16, 8, 0, 0, 0),
            //),
            //(
            //    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            //    32,
            //    33,
            //    42,
            //    (true, 1, 0, 8, 0, 10, 1),
            //),
            //(
            //    [0, 0, 0, 0, 0, 0, 0, 1, 0, 8, 0, 0, 0, 0],
            //    32,
            //    33,
            //    42,
            //    (false, 1, 32, 8, 0, 0, 0),
            //),
            //(
            //    [0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0],
            //    32,
            //    33,
            //    42,
            //    (false, 1, 4, 8, 0, 0, 0),
            //),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
                33,
                34,
                41,
                (true, 1, 0, 8, 1, 10, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
                34,
                35,
                43,
                (true, 1, 0, 8, 2, 10, 2),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
                35,
                36,
                44,
                (true, 1, 0, 8, 3, 10, 3),
            ),
        ];
        for (message, f, s, e, result) in messages {
            assert_eq!(
                badflags(&message, f, s, e),
                result,
                "F:{} S:{} E:{} L:{}",
                f,
                s,
                e,
                e - s + 1
            );
        }
    }
}

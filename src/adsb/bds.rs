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

pub fn badflags(message: &[u32], flag: u32, sb: u32, eb: u32) -> (bool, u32, u32, usize, usize) {
    let flag_ibit: usize = match flag % 4 {
        0 => 3,
        r => (r + 1) as usize,
    };
    let flag_ibyte: usize = ((flag - 1) / 4).try_into().unwrap();
    let sb_ibit: usize = match sb % 4 {
        0 => 3,
        r => (r + 1) as usize,
    };
    let sb_ibyte: usize = ((sb - 1) / 4).try_into().unwrap();
    let eb_ibit: usize = match eb % 4 {
        0 => 3,
        r => (r + 1) as usize,
    };
    let eb_ibyte: usize = ((eb - 1) / 4).try_into().unwrap();

    let mut result = message[sb_ibyte] & (0xF >> (4 - sb_ibit));
    for ibytes in (sb_ibyte + 1)..eb_ibyte {
        result = (result << 4) | (message[ibytes] & 0xF);
    }
    result = (result << eb_ibit) | (message[eb_ibyte] & (0xF >> (4 - eb_ibit)));

    let flag = message[flag_ibyte] >> (4 - flag_ibit);
    match flag {
        0 => (true, flag, result, flag_ibyte, flag_ibit),
        _ => match result {
            0 => (true, flag, result, flag_ibyte, flag_ibit),
            _ => (false, flag, result, flag_ibyte, flag_ibit),
        },
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
            assert_eq!(bds(&message), (0, 0));
        }
    }

    #[test]
    fn test_badflags_true() {
        let messages = [
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                32,
                36,
                42,
                (true, 0, 0, 7, 3),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                33,
                36,
                42,
                (true, 0, 0, 8, 2),
            ),
        ];
        for (message, f, s, e, result) in messages {
            assert_eq!(
                badflags(&message, f, s, e),
                result,
                "F:{} S:{} E:{}",
                f,
                s,
                e
            );
        }
        //let message = [0, 0, 0, 0, 0, 0, 0, 0, 0b10, 0, 0, 0, 0, 0];
        //assert_eq!(
        //    badflags(&message, 35, 36, 42),
        //    (true, 1, 0, 8, 2),
        //    "only flag is 1"
        //);
        //
        //let message = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        //assert_eq!(
        //    badflags(&message, 34, 36, 42),
        //    (true, 0, 0, 8, 2),
        //    "all zeros"
        //);
        //let message = [0, 0, 0, 0, 0, 0, 0, 0, 0b10, 0, 0, 0, 0, 0];
        //assert_eq!(
        //    badflags(&message, 33, 36, 42),
        //    (true, 1, 0, 8, 2),
        //    "only flag is 1"
        //);
    }

    //#[test]
    //fn test_badflags_false() {
    //    let message = [0, 0, 0, 0, 0, 0, 0, 0, 0b11, 2, 0, 0, 0, 0];
    //    assert_eq!(badflags(&message, 35, 36, 42), (false, 1, 1, 8, 2));
    //    //let message = [0, 0, 0, 0, 0, 0, 0, 2, 0xf, 0, 0, 0, 0, 0];
    //    //assert_eq!(badflags(&message, 35, 36, 42), false);
    //}
}

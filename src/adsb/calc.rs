pub(crate) fn flag_and_range_value(
    message: &[u32],
    flag: u32,
    sb: u32,
    eb: u32,
) -> Option<(u32, u32)> {
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

    let flag = match flag {
        0 => 0,
        _ => {
            let (flag_ibyte, flag_ibit) = bit_location(flag);
            (message[flag_ibyte] >> (3 - flag_ibit)) & 1
        }
    };
    Some((flag, result))
}

fn bit_location(position: u32) -> (usize, usize) {
    let ibyte: usize = ((position - 1) / 4).try_into().unwrap();
    let ibit: usize = ((position - 1) % 4).try_into().unwrap();
    (ibyte, ibit)
}

#[cfg(test)]
mod tests {
    use super::*;

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

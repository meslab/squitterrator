pub fn vertical_rate(message: &[u32]) -> Option<i32> {
    let is_negative = (message[17] >> 3) & 1;
    let absolute_rate =
        (((message[17] & 0b111) << 6 | (message[18] & 0xF) << 2 | (message[19] & 0b1100) >> 2) - 1)
            * 64;
    match absolute_rate {
        0 => None,
        _ => match is_negative {
            1 => Some(absolute_rate as i32 * (-1)),
            _ => Some(absolute_rate as i32),
        },
    }
}

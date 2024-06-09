pub fn altitude_delta(message: &[u32]) -> Option<i32> {
    if let Some((is_negative, absolute_delta)) =
        crate::adsb::flag_and_range_value(message, 81, 82, 88)
    {
        match absolute_delta {
            0 => None,
            _ => match is_negative {
                1 => Some(-(absolute_delta as i32) * 25),
                _ => Some(absolute_delta as i32 * 25),
            },
        }
    } else {
        None
    }
}

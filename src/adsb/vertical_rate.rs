pub fn vertical_rate(message: &[u32]) -> Option<i32> {
    if let Some((is_negative, value)) = crate::adsb::flag_and_range_value(message, 69, 70, 78) {
        let absolute_rate = ((value - 1) << 6) as i32;
        match absolute_rate {
            0 => None,
            _ => match is_negative {
                1 => Some(-(absolute_rate)),
                _ => Some(absolute_rate),
            },
        }
    } else {
        None
    }
}

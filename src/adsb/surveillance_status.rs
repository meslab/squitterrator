pub fn surveillance_status(message: &[u32]) -> char {
    match (message[9] & 7) >> 1 {
        0 => 'N',
        1 => 'P',
        2 => 'T',
        3 => 'S',
        _ => ' ',
    }
}

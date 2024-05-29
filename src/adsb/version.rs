pub fn version(message: &[u32]) -> Option<u32> {
    Some((message[18] & 0b1110) >> 1)
}

pub(crate) fn altitude_gnss(message: &[u32]) -> Option<u32> {
    crate::decoder::range_value(message, 49, 60)
}

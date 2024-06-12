pub(crate) fn version(message: &[u32]) -> Option<u32> {
    crate::decoder::range_value(message, 73, 75)
}

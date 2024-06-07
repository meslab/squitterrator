use crate::adsb;

pub fn is_bds_1_7(message: &[u32]) -> Option<(u32, u32)> {
    if ((message[8] >> 3 & 1) ^ (message[8] >> 2 & 1)) == 1
        && message[14..21].iter().all(|&x| x == 0)
    {
        adsb::flag_and_range_value(message, 33, 33, 56)
    } else {
        None
    }
}

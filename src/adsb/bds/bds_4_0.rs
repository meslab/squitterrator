use crate::adsb;

pub fn is_bds_4_0(message: &[u32]) -> Option<(u32, u32)> {
    if adsb::goodflags(message, 33, 34, 45)
        && adsb::goodflags(message, 46, 47, 58)
        && adsb::goodflags(message, 59, 60, 71)
        && !adsb::goodflags(message, 33, 72, 79)
        && !adsb::goodflags(message, 33, 84, 85)
    {
        adsb::flag_and_range_value(message, 33, 33, 56)
    } else {
        None
    }
}

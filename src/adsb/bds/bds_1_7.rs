use crate::adsb;

pub fn is_bds_1_7(message: &[u32]) -> Option<(u32, u32)> {
    if let Some((bds20, reserved)) = adsb::flag_and_range_value(message, 39, 61, 88) {
        if bds20 == 1 && reserved == 0 {
            adsb::flag_and_range_value(message, 33, 33, 56)
        } else {
            None
        }
    } else {
        None
    }
}

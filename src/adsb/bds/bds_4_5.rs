use crate::adsb;

pub(crate) fn is_bds_4_5(message: &[u32]) -> Option<f64> {
    if adsb::goodflags(message, 33, 34, 35)
        && adsb::goodflags(message, 36, 37, 38)
        && adsb::goodflags(message, 39, 40, 41)
        && adsb::goodflags(message, 42, 43, 44)
        && adsb::goodflags(message, 45, 46, 47)
        && adsb::goodflags(message, 48, 49, 58)
        && adsb::goodflags(message, 59, 60, 60)
        && adsb::goodflags(message, 71, 72, 83)
        && !adsb::goodflags(message, 33, 84, 88)
    {
        adsb::temperature_4_5(message).filter(|&temp| temp <= 45.0)
    } else {
        None
    }
}
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_is_bds_4_5()
//}

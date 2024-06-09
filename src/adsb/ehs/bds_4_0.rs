pub(crate) fn mcp_selected_altitude(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 33, 34, 45) {
        if status == 1 {
            Some(value * 16)
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn fms_selected_altitude(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 46, 47, 58) {
        if status == 1 {
            Some(value * 16)
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn barometric_pressure_setting(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 59, 60, 71) {
        if status == 1 {
            Some(value / 10 - 800)
        } else {
            Some(value / 10)
        }
    } else {
        None
    }
}

pub(crate) fn target_altitude_source(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = crate::adsb::flag_and_range_value(message, 86, 87, 88) {
        if status == 1 {
            Some(value)
        } else {
            None
        }
    } else {
        None
    }
}

use log::debug;

use crate::adsb;

#[derive(Debug)]
pub struct SelectedVerticalIntention {
    pub mcp_selected_altitude: Option<u32>,
    pub fms_selected_altitude: Option<u32>,
    pub barometric_pressure_setting: Option<u32>,
    pub target_altitude_source: Option<u32>,
}

impl SelectedVerticalIntention {
    pub fn new() -> Self {
        SelectedVerticalIntention {
            mcp_selected_altitude: None,
            fms_selected_altitude: None,
            barometric_pressure_setting: None,
            target_altitude_source: None,
        }
    }

    pub fn from_data(
        mcp_selected_altitude: Option<u32>,
        fms_selected_altitude: Option<u32>,
        barometric_pressure_setting: Option<u32>,
        target_altitude_source: Option<u32>,
    ) -> Self {
        SelectedVerticalIntention {
            mcp_selected_altitude,
            fms_selected_altitude,
            barometric_pressure_setting,
            target_altitude_source,
        }
    }
}

impl Default for SelectedVerticalIntention {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_4_0(message: &[u32]) -> Option<SelectedVerticalIntention> {
    if adsb::goodflags(message, 33, 34, 45)
        && adsb::goodflags(message, 46, 47, 58)
        && adsb::goodflags(message, 59, 60, 71)
        && !adsb::goodflags(message, 33, 72, 79)
        && !adsb::goodflags(message, 33, 84, 85)
    {
        let intent = SelectedVerticalIntention::from_data(
            adsb::mcp_selected_altitude(message).filter(|x| (0..=65530).contains(x)),
            adsb::fms_selected_altitude(message).filter(|x| (0..=65530).contains(x)),
            adsb::barometric_pressure_setting(message).filter(|x| (0..=410).contains(x)),
            adsb::target_altitude_source(message).filter(|x| (0..=3).contains(x)),
        );
        debug!("BDS:4.0 {:?}", intent);
        if intent.mcp_selected_altitude.is_some() || intent.fms_selected_altitude.is_some() {
            Some(intent)
        } else {
            None
        }
    } else {
        None
    }
}

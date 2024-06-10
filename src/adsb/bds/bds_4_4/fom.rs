#[derive(Debug)]
pub(super) enum FOM {
    NoData,
    Wind,
    Temp,
    Turb,
    Hum,
    WindTemp,
    WindTurb,
    WindHum,
    TempTurb,
    TempHum,
    TurbHum,
    WindTempTurb,
    WindTempHum,
    WindTurbHum,
    TempTurbHum,
    WindTempTurbHum,
}

pub(super) fn fom(message: &[u32]) -> Option<FOM> {
    crate::adsb::range_value(message, 33, 36).map(|x| match x {
        1 => FOM::Wind,
        2 => FOM::Temp,
        3 => FOM::Turb,
        4 => FOM::Hum,
        5 => FOM::WindTemp,
        6 => FOM::WindTurb,
        7 => FOM::WindHum,
        8 => FOM::TempTurb,
        9 => FOM::TempHum,
        10 => FOM::TurbHum,
        11 => FOM::WindTempTurb,
        12 => FOM::WindTempHum,
        13 => FOM::WindTurbHum,
        14 => FOM::TempTurbHum,
        15 => FOM::WindTempTurbHum,
        _ => FOM::NoData,
    })
}

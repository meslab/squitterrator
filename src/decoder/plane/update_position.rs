use super::Plane;
use crate::decoder;

/// Updates the position of the plane based on the received message type and CPR format.
///
/// # Arguments
///
/// * `message_type` - The type of the received message.
/// * `cpr_form` - The CPR format.
///
/// # Remarks
///
/// This method updates the position of the plane if the following conditions are met:
/// - The CPR latitude and longitude values are not equal to zero.
/// - The time difference between the CPR timestamps is less than 10 seconds.
///
/// If the conditions are met and the calculated latitude and longitude values are within the valid range,
/// the plane's latitude, longitude, and position timestamp are updated.
///
impl Plane {
    pub(super) fn update_position(&mut self, message_type: u32, cpr_form: u32) {
        if self.cpr_lat[0] != 0
            && self.cpr_lat[1] != 0
            && self.cpr_lon[0] != 0
            && self.cpr_lon[1] != 0
            && self.cpr_time[0]
                .signed_duration_since(self.cpr_time[1])
                .num_seconds()
                .abs()
                < 10
        {
            if let Some((lat, lon)) = match message_type {
                5..=8 => decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 4),
                9..=18 => decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 1),
                _ => None,
            } {
                if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                    self.lat = lat;
                    self.lon = lon;
                    self.position_timestamp = Some(self.timestamp);
                }
            }
        }
    }
}

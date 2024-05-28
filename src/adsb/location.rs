use crate::adsb::pmod;

pub fn cpr(message: &[u32]) -> (u32, u32, u32) {
    let cpr_form = (message[13] >> 2) & 1;
    let cpr_lat = ((message[13] & 3) << 15)
        | ((message[14] & 0xF) << 11)
        | ((message[15] & 0xF) << 7)
        | ((message[16] & 0xF) << 3)
        | ((message[17] & 0xE) >> 1);
    let cpr_long = ((message[17] & 1) << 16)
        | ((message[18] & 0xF) << 12)
        | ((message[19] & 0xF) << 8)
        | ((message[20] & 0xF) << 4)
        | message[21] & 0xF;
    (cpr_form, cpr_lat, cpr_long)
}

pub fn cpr_location(
    cpr_lat: &[u32; 2],
    cpr_lon: &[u32; 2],
    cpr_form: u32,
    coeff: i32,
) -> Option<(f64, f64)> {
    let div = (1 << 17) as f64;
    let adl0 = 6.0; // 360 / 60
    let adl1 = 360.0 / 59.0;

    let j = ((59.0 * cpr_lat[0] as f64 - 60.0 * cpr_lat[1] as f64) / div + 0.5).floor();
    let rlat = [
        fixed_lat(adl0 * ((j % 60.0) + (cpr_lat[0] as f64 / div))),
        fixed_lat(adl1 * ((j % 59.0) + (cpr_lat[1] as f64 / div))),
    ];

    let nl = [nl(rlat[0]), nl(rlat[1])];
    match nl[0] == nl[1] {
        true => {
            let (ni, nlt, lngt) = match cpr_form {
                1 => (
                    *[nl[1] / coeff - 1, 1].iter().max().unwrap(), // coeff = 4 for ground location
                    nl[1] / coeff, // coeff = 1 for airborne location
                    cpr_lon[1],
                ),
                _ => (
                    *[nl[0] / coeff, 1].iter().max().unwrap(), // coeff = 4 for ground location
                    nl[0] / coeff,                             // coeff = 1 for airborne location
                    cpr_lon[0],
                ),
            };
            let dlngt = 360.0 / ni as f64;
            let m = (((cpr_lon[0] as f64 * (nlt - 1) as f64 - cpr_lon[1] as f64 * nlt as f64)
                / div)
                + 0.5)
                .floor();
            let lon = dlngt * (pmod(m as i32, ni) as f64 + lngt as f64 / div) as f64;

            Some((rlat[cpr_form as usize], signed_lon(lon)))
        }
        false => None,
    }
}

fn signed_lon(lon: f64) -> f64 {
    match lon {
        _ if lon > 180.0 => lon - 360.0,
        _ if lon < -180.0 => lon + 360.0,
        _ => lon,
    }
}

fn fixed_lat(lat: f64) -> f64 {
    match lat {
        _ if lat > 90.0 => lat - 360.0,
        _ if lat < -90.0 => lat + 360.0,
        _ => lat,
    }
}

/// Calculate the latitude .
fn nl(lat: f64) -> i32 {
    let lat = lat.abs();
    let boundaries = [
        (10.47047130, 59),
        (14.82817437, 58),
        (18.18626357, 57),
        (21.02939493, 56),
        (23.54504487, 55),
        (25.82924707, 54),
        (27.93898710, 53),
        (29.91135686, 52),
        (31.77209708, 51),
        (33.53993436, 50),
        (35.22899598, 49),
        (36.85025108, 48),
        (38.41241892, 47),
        (39.92256684, 46),
        (41.38651832, 45),
        (42.80914012, 44),
        (44.19454951, 43),
        (45.54626723, 42),
        (46.86733252, 41),
        (48.16039128, 40),
        (49.42776439, 39),
        (50.67150166, 38),
        (51.89342469, 37),
        (53.09516153, 36),
        (54.27817472, 35),
        (55.44378444, 34),
        (56.59318756, 33),
        (57.72747354, 32),
        (58.84763776, 31),
        (59.95459277, 30),
        (61.04917774, 29),
        (62.13216659, 28),
        (63.20427479, 27),
        (64.26616523, 26),
        (65.31845310, 25),
        (66.36171008, 24),
        (67.39646774, 23),
        (68.42322022, 22),
        (69.44242631, 21),
        (70.45451075, 20),
        (71.45986473, 19),
        (72.45884545, 18),
        (73.45177442, 17),
        (74.43893416, 16),
        (75.42056257, 15),
        (76.39684391, 14),
        (77.36789461, 13),
        (78.33374083, 12),
        (79.29428225, 11),
        (80.24923213, 10),
        (81.19801349, 9),
        (82.13956981, 8),
        (83.07199445, 7),
        (83.99173563, 6),
        (84.89166191, 5),
        (85.75541621, 4),
        (86.53536998, 3),
        (87.00000000, 2),
    ];

    for &(boundary, value) in &boundaries {
        if lat < boundary {
            return value;
        }
    }
    1
}
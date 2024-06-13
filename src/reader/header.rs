pub(super) fn print_header(
    weather: bool,
    angles: bool,
    speed: bool,
    altitude: bool,
    extra: bool,
    header: bool,
) {
    let headers_1 = [
        ("ICAO", 6),
        ("RG", 2),
        ("SQWK", 4),
        ("W", 1),
        ("CALLSIGN", 8),
        ("LATITUDE", 9),
        ("LONGITUDE", 11),
        ("ALT B", 5),
    ];
    let headers_2 = [("VRATE", 5), ("TRK", 3), ("HDG", 3), ("GSP", 3)];

    let headers_speed = [("TAS", 3), ("IAS", 3), ("MACH", 4)];
    let headers_angles = [("RLL", 3), ("TAR", 3)];
    let headers_altitude = [("ALT G", 5), ("ALT S", 5), ("BARO", 4)];
    let headers_weather = [
        ("TEMP", 5),
        ("WND", 3),
        ("WDR", 3),
        ("HUM", 3),
        ("PRES", 4),
        ("TB", 2),
    ];

    let extra_headers = [
        ("VX", 2),
        ("DF", 2),
        ("TC", 2),
        ("V", 1),
        ("S", 1),
        ("PTH", 3),
    ];

    let header_line: String = headers_1
        .iter()
        .map(|&(header, width)| format!("{:>width$} ", header, width = width))
        .chain(if altitude {
            headers_altitude
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(
            headers_2
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width)),
        )
        .chain(if speed {
            headers_speed
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if angles {
            headers_angles
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if weather {
            headers_weather
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if extra {
            extra_headers
                .iter()
                .map(|&(header, width)| format!("{:>width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>()
        + "LC\n";

    let separator_line: String = headers_1
        .iter()
        .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
        .chain(if altitude {
            headers_altitude
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(
            headers_2
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width)),
        )
        .chain(if speed {
            headers_speed
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if angles {
            headers_angles
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if weather {
            headers_weather
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .chain(if extra {
            extra_headers
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>()
        + "--\n";

    if header {
        print!("{}{}", header_line, separator_line);
    } else {
        print!("{}", separator_line);
    }
}

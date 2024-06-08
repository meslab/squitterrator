use crate::Args;
use log::{debug, error, warn};
use squitterator::adsb::message;
use squitterator::adsb::{df, icao};
use squitterator::plane::{format_simple_display, Plane};
use std::collections::{BTreeMap, HashMap};
use std::io::{BufRead, Result};

pub fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: &mut HashMap<u32, Plane>,
) -> Result<()> {
    let display_flags = args.display.concat().chars().collect::<Vec<char>>();

    if !args.display.is_empty() {
        clear_screen();
        print_legend(
            display_flags.contains(&'w'),
            display_flags.contains(&'a'),
            display_flags.contains(&'s'),
            display_flags.contains(&'e'),
        );
    }
    let mut df_count = BTreeMap::new();
    let mut timestamp = chrono::Utc::now() + chrono::Duration::seconds(args.update);
    for line in reader.lines() {
        match line {
            Ok(squitter) => {
                debug!("Squitter: {}", squitter);
                if let Some(message) = message(&squitter) {
                    let df = match df(&message) {
                        Some(df) => df,
                        None => {
                            continue;
                        }
                    };
                    if args.count_df {
                        *df_count.entry(df).or_insert(1) += 1;
                    }
                    if let Some(m) = &args.log_messages {
                        if m.contains(&df) {
                            error!("DF:{}, L:{}", df, squitter);
                        }
                    }
                    if let Some(only) = &args.filter {
                        if only.iter().all(|&x| x != df) {
                            continue;
                        }
                    }

                    if !args.display.is_empty() {
                        if let Some(icao) = icao(&message, df) {
                            planes
                                .entry(icao)
                                .and_modify(|p| p.update(&message, df, args.relaxed))
                                .or_insert(Plane::from_message(&message, df, icao, args.relaxed));

                            let now = chrono::Utc::now();
                            if now.signed_duration_since(timestamp).num_seconds() > args.update {
                                clear_screen();
                                print_header(
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                    true,
                                );
                                planes.retain(|_, plane| {
                                    let elapsed =
                                        now.signed_duration_since(plane.timestamp).num_seconds();
                                    if elapsed < 60 {
                                        true
                                    } else {
                                        debug!("Plane {} has been removed from view", plane.icao);
                                        false
                                    }
                                });
                                planes.shrink_to_fit();
                                print_planes(
                                    planes,
                                    args,
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                );
                                debug!("Squirter: {}", squitter);
                                debug!("{}", planes[&icao]);
                                timestamp = now;
                                print_header(
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                    false,
                                );
                                if args.count_df {
                                    let result =
                                        df_count.iter().fold(String::new(), |acc, (df, count)| {
                                            acc + &format!("DF{}:{} ", df, count)
                                        });
                                    println!("{}", result);
                                }
                            }
                        }
                    }
                };
            }
            Err(e) => warn!("Warn: {}", e),
        }
    }
    Ok(())
}

fn clear_screen() {
    print!("{0}[2J{0}[H{0}[3J", 27 as char);
}

fn print_header(
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
    let headers_altitude = [("ALT G", 5)];
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

fn print_legend(weather: bool, angles: bool, speed: bool, extra: bool) {
    let legend = [
        ("ICAO", "ICAO Address"),
        ("RG", "Registraton Country Code"),
        ("ALT B", "Altitude (Barometric)"),
        ("SQWK", "Squawk"),
        ("CALLSIGN", "Callsign"),
        ("LATITUDE", "Latitude"),
        ("LONGITUDE", "Longitude"),
        ("GSP", "Ground Speed"),
        ("TRK", "Track"),
        ("HDG", "Heading"),
        ("VRATE", "Vertical Rate"),
        ("LC", "Last Contact"),
        ("W", "Wake Turbulence Category"),
    ];

    let legend_speed = [
        ("TAS", "True Air Speed"),
        ("IAS", "Indicated Air Speed"),
        ("MACH", "Mach Number"),
    ];
    let legend_angles = [("RLL", "Roll Angle")];
    let legend_weather = [
        ("TEMP", "Static temperature"),
        ("WND", "Wind Speed"),
        ("WDR", "Wind Direction"),
        ("HUM", "Humidity"),
        ("PRES", "Static pressure"),
        ("TB", "Turbulence"),
    ];

    let legend_extra = [
        ("VX", "Wake Vortex ADS-B Category"),
        ("DF", "Downlink Format"),
        ("TC", "Type Code"),
        ("V", "ASD-B Version"),
        ("S", "Surveillance Status"),
        ("PTH", "Position, Track, Heaging age"),
    ];

    let width = (10, 28);
    let legend_line = legend
        .iter()
        .map(|&(header, description)| {
            format!(
                "{:w0$}: {:w1$}\n",
                header,
                description,
                w0 = width.0,
                w1 = width.1
            )
        })
        .chain(if speed {
            legend_speed
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if angles {
            legend_angles
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if weather {
            legend_weather
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if extra {
            legend_extra
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>();

    print!("{}", legend_line);
}

fn print_planes(
    planes: &mut HashMap<u32, Plane>,
    args: &Args,
    weather: bool,
    angles: bool,
    speed: bool,
    altitude: bool,
    extra: bool,
) {
    let mut planes_vector: Vec<(&u32, &Plane)> = planes.iter().collect();
    planes_vector.sort_by_cached_key(|&(k, _)| k);
    for order_by in &args.order_by {
        for c in order_by.chars() {
            match c {
                'a' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.altitude);
                }
                'A' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.altitude);
                    planes_vector.reverse();
                }
                'c' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.category);
                }
                'C' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| {
                        -(((p.category.0 << 1) | p.category.1) as i32)
                    });
                }
                'N' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.lat as i32);
                }
                'S' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| -(p.lat as i32));
                }
                'W' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.lon as i32);
                }
                'E' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| -(p.lon as i32));
                }
                's' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.squawk);
                }
                'V' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| -(p.vrate.unwrap_or(0)));
                }
                'v' => {
                    planes_vector.sort_by_cached_key(|&(_, p)| p.vrate.unwrap_or(0));
                }
                _ => {}
            }
        }
    }

    print!(
        "{}",
        planes_vector.iter().fold(String::new(), |acc, (_, plane)| {
            acc + &format!(
                "{}\n",
                format_simple_display(*plane, weather, angles, speed, altitude, extra)
            )
        })
    );
}

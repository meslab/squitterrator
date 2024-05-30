use crate::Args;
use log::{debug, warn};
use squitterator::adsb::message;
use squitterator::adsb::{df, icao};
use squitterator::plane::{format_simple_display, Plane};
use std::collections::HashMap;
use std::io::{BufRead, Result};

pub fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: &mut HashMap<u32, Plane>,
) -> Result<()> {
    if args.planes {
        clear_screen();
        print_legend(args.wide);
    }
    let mut timestamp = chrono::Utc::now() + chrono::Duration::seconds(args.refresh);
    for line in reader.lines() {
        match line {
            Ok(squitter) => {
                debug!("Squitter: {}", squitter);
                if let Some(message) = message(&squitter) {
                    let df = df(&message);
                    if let Some(only) = &args.only {
                        if only.iter().all(|&x| x != df) {
                            continue;
                        }
                    }

                    let now = chrono::Utc::now();
                    if args.planes {
                        if let Some(icao) = icao(&message, df) {
                            planes
                                .entry(icao)
                                .and_modify(|p| p.update(&message, df))
                                .or_insert(Plane::from_message(&message, df, icao));

                            if now.signed_duration_since(timestamp).num_seconds() > args.refresh {
                                clear_screen();
                                print_header(args.wide);
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

                                // Print the entire result string at once
                                print!(
                                    "{}",
                                    planes.iter().fold(String::new(), |acc, (_, plane)| {
                                        acc + &format!(
                                            "{}\n",
                                            format_simple_display(plane, args.wide)
                                        )
                                    })
                                );

                                debug!("Squirter: {}", squitter);
                                debug!("{}", planes[&icao]);
                                timestamp = now;
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
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}

fn print_header(wide: bool) {
    let headers = [
        ("ICAO", 6),
        ("RG", 2),
        ("ALT", 5),
        ("SQWK", 4),
        ("CALLSIGN", 8),
        ("LATITUDE", 9),
        ("LONGITUDE", 11),
        ("GSPD", 4),
        ("TRK", 3),
        ("VRATE", 5),
    ];

    let extra_headers = [
        ("W", 1),
        ("DF", 2),
        ("TC", 2),
        ("V", 1),
        ("S", 1),
        ("LPC", 3),
    ];

    let header_line: String = headers
        .iter()
        .map(|&(header, width)| format!("{:width$} ", header, width = width))
        .chain(if wide {
            extra_headers
                .iter()
                .map(|&(header, width)| format!("{:width$} ", header, width = width))
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>()
        + "LC\n";

    let separator_line: String = headers
        .iter()
        .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
        .chain(if wide {
            extra_headers
                .iter()
                .map(|&(_, width)| format!("{:-<width$} ", "", width = width))
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>()
        + "--\n";

    // Print the result
    print!("{}{}", header_line, separator_line);
}

fn print_legend(wide: bool) {
    let legend = [
        ("ICAO", "ICAO Address"),
        ("DF", "Downlink Format"),
        ("RG", "Registraton Country Code"),
        ("ALT", "Altitude"),
        ("SQWK", "Squawk"),
        ("CALLSIGN", "Callsign"),
        ("LATITUDE", "Latitude"),
        ("LONGITUDE", "Longitude"),
        ("GSPD", "Ground Speed"),
        ("TRK", "Track"),
        ("VRATE", "Vertical Rate"),
        ("LC", "Last Contact"),
    ];

    let legend_wide = [
        ("W", "Wake Turbulence Category"),
        ("TC", "Type Code"),
        ("V", "ASD-B Version"),
        ("S", "Surveillance Status"),
        ("LPC", "Last Position Contact"),
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
        .chain(if wide {
            legend_wide
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

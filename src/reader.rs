use crate::Args;
use log::{debug, warn};
use squitterator::adsb::message;
use squitterator::adsb::{clean_squitter, df, icao};
use squitterator::plane::Plane;
use squitterator::process::generate_ais;
use squitterator::process::icao_decode;
use squitterator::process::squitter_decode;
use std::collections::HashMap;
use std::io::{BufRead, Result};

pub fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: &mut HashMap<u32, Plane>,
) -> Result<()> {
    for line in reader.lines() {
        match line {
            Ok(squitter) => {
                debug!("Squitter: {}", squitter);
                if let Some(message) = message(&squitter) {
                    let df = df(&message);
                    if args.ais {
                        generate_ais(&message, &squitter);
                    }

                    if args.decode {
                        squitter_decode(&message, df);
                    }

                    if args.icao {
                        icao_decode(&message, df, clean_squitter(&squitter).unwrap().as_str());
                    }
                    if let Some(icao) = icao(&message, df) {
                        planes
                            .entry(icao)
                            .and_modify(|p| p.update(&message, df))
                            .or_insert(Plane::from_message(&message, df, icao));
                        debug!("Total planes in view: {}", planes.len());
                        debug!("{}", planes[&icao]);
                        println!("{}", planes[&icao]);
                    }
                };
            }
            Err(e) => warn!("Warn: {}", e),
        }
    }
    Ok(())
}

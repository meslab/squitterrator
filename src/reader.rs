use log::{debug, warn};
use squitterator::adsb::clean_squitter;
use squitterator::adsb::df;
use squitterator::adsb::message;
use squitterator::process::generate_ais;
use squitterator::process::icao_decode;
use squitterator::process::squitter_decode;
use std::io::{BufRead, Result};
use crate::Args;

pub fn read_lines<R: BufRead>(reader: R, args: &Args) -> Result<()> {
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
                }
            }
            Err(e) => warn!("Warn: {}", e),
        }
    }
    Ok(())
}

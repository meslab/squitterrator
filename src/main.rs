use clap::Parser;
use env_logger::{Builder, Env};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::sync::Mutex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.0.1",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "ADS-B squitter decoder"
)]
pub struct Args {
    #[clap(short, long)]
    ais: bool,

    #[clap(short, long)]
    decode: bool,

    #[clap(short, long)]
    icao: bool,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long, default_value = "log.log")]
    log: String,

    #[clap(
        short,
        long,
        conflicts_with = "tcp",
        default_value = "rec/squitters.txt"
    )]
    source: String,

    #[clap(
        short,
        long,
        conflicts_with = "source",
        required = false,
        default_value = ""
    )]
    tcp: String,

    #[clap(short, long, default_value = None)]
    format: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let log_file = File::create(&args.log).expect("Unable to create log file");
    let log_file = Mutex::new(log_file);

    // Initialize the logger
    Builder::from_env(Env::default().default_filter_or("error"))
        .format(move |buf, record| {
            let mut log_file = log_file.lock().unwrap();
            if args.verbose {
                writeln!(buf, "{} - {}", record.level(), record.args()).unwrap();
            }
            writeln!(log_file, "{} - {}", record.level(), record.args())
        })
        .init();

    match !args.tcp.is_empty() {
        true => {
            let stream = match TcpStream::connect(&args.tcp) {
                Ok(stream) => {
                    println!("Successfully connected to the server");
                    stream
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}", e);
                    return Err(e);
                }
            };
            let reader = BufReader::new(stream);
            read_lines(reader, &args)
        }
        _ => {
            let file = File::open(&args.source)?;
            let reader = BufReader::new(file);
            read_lines(reader, &args)
        }
    }
}

use log::{debug, warn};
use squitterator::adsb::clean_squitter;
use squitterator::adsb::df;
use squitterator::adsb::message;
use squitterator::process::generate_ais;
use squitterator::process::icao_decode;
use squitterator::process::squitter_decode;
use std::io::BufRead;

pub fn read_lines<R: BufRead>(reader: R, args: &Args) -> io::Result<()> {
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

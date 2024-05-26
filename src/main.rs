mod reader;

use crate::reader::read_lines;
use clap::Parser;
use env_logger::{Builder, Env};
use log::info;
use squitterator::plane::Plane;
use std::collections::HashMap;
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
    let mut planes: HashMap<u32, Plane> = HashMap::new();

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
                    info!("Successfully connected to the server {}", &args.tcp);
                    stream
                }
                Err(e) => {
                    eprintln!("Failed to connect: {}", e);
                    return Err(e);
                }
            };
            let reader = BufReader::new(stream);
            read_lines(reader, &args, &mut planes)
        }
        _ => {
            let file = File::open(&args.source)?;
            let reader = BufReader::new(file);
            read_lines(reader, &args, &mut planes)
        }
    }
}

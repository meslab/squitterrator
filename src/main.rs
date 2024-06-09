mod reader;

use crate::reader::read_lines;
use clap::Parser;
use env_logger::{Builder, Env};
use log::{error, info};
use squitterator::adsb::Plane;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.2.2",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "ADS-B squitter decoder"
)]
pub struct Args {
    #[clap(short, long, default_value = None, help = "Process only specific DF messages\n -f 21 - only DF21\n -f 21 -f 4 - DF4 and DF21")]
    filter: Option<Vec<u32>>,

    #[clap(
        short,
        long,
        default_value = "aAews",
        help = "Display plane patameters\na - angles, A - altitude\ns - speed, e - extra info"
    )]
    display: Vec<String>,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long, default_value = "log.log")]
    log: String,

    #[clap(short='M', long, default_value = None)]
    log_messages: Option<Vec<u32>>,

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

    #[clap(short, long, default_value = "3")]
    update: i64,

    #[clap(
        short,
        long,
        default_value = "sA",
        help = "s - squawk, a,A - altitude,\nc,C - category, N, S, E, W - location,\nv,V - vertical rate"
    )]
    order_by: Vec<String>,

    #[clap(short = 'R', long, help = "Relaxed Capabilities check EHS")]
    relaxed: bool,

    #[clap(short, long)]
    count_df: bool,
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
        true => loop {
            let stream = match TcpStream::connect(&args.tcp) {
                Ok(stream) => {
                    info!("Successfully connected to the server {}", &args.tcp);
                    stream
                }
                Err(e) => {
                    error!("Failed to connect: {}", e);
                    continue;
                }
            };
            let reader = BufReader::new(stream);
            if let Err(e) = read_lines(reader, &args, &mut planes) {
                error!("Error during reading: {}", e);
                sleep(Duration::from_secs(5));
                continue;
            }
        },
        _ => {
            let file = File::open(&args.source)?;
            let reader = BufReader::new(file);
            read_lines(reader, &args, &mut planes)
        }
    }
}

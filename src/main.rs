use clap::Parser;
use env_logger::{Builder, Env};
use log::{debug, warn};
use squitterator::adsb::{ais, clean_squitter, df, message, mode_e_decoded_message};
use squitterator::country::icao_to_country;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Mutex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.1.0",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "ADS-B squitter decoder"
)]
struct Args {
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

fn read_lines<R: BufRead>(reader: R, args: &Args) -> io::Result<()> {
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

fn generate_ais(message: &[u32], squitter: &str) {
    if let Some(result) = ais(message) {
        println!("(\"{}\", \"{}\"),", squitter, result);
    }
}

fn squitter_decode(message: &[u32], df: u32) {
    if let Some(r) = mode_e_decoded_message(message, df) {
        println!(
            "DF:{:>2}, Alt:{:>5}, AIS:{:8}, Vs:{}, Vr:{:>5}, F:{}, Lat:{:>6}, Lon:{:>6}, W:{}, S:{}, Gs:{}, As:{}, H:{}, Tu:{}, Tr:{}",
            df,
            r.alt,
            r.ais,
            r.vsign,
            r.vrate,
            r.cpr_form,
            r.cpr_lat,
            r.cpr_long,
            r.sp_west,
            r.sp_south,
            r.grspeed,
            r.airspeed,
            r.heading,
            r.turn,
            r.track
        )
    }
}

fn icao_decode(message: &[u32], df: u32, squitter: &str) {
    if let Some(icao_address) = squitterator::adsb::icao(message, df) {
        debug!("Squitter: {}, M: {:?}", squitter, message);
        let (country, cshrt) = icao_to_country(icao_address);
        println!(
            "Squitter: {:28}, ICAO: {:06X}, DF:{:2}, {}: {}",
            squitter, icao_address, df, cshrt, country
        );
    }
}

mod header;
mod legend;
mod planes;

use header::print_header;
use legend::print_legend;
use planes::print_planes;

use crate::Args;
use squitterator::decoder::{df, icao};
use squitterator::decoder::{message, Plane};

use log::{debug, error, warn};
use std::collections::{BTreeMap, HashMap};
use std::io::{BufRead, Result};

pub(super) fn read_lines<R: BufRead>(
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
                    if args.count_df {
                        *df_count.entry(df).or_insert(1) += 1;
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

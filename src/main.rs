extern crate clap;
use clap::{Arg, Command};
use std::time::Instant;

mod report;
use report::process_csv;

fn main() {
    let start = Instant::now();
    let matches = Command::new("ilmn2ped")
        .version("1.0.0")
        .author("Andrea Talenti <andrea.talenti@ed.ac.uk>")
        .about("Easy conversion from Illumina manifest CSV file to ped/map")
        .arg(
            Arg::new("INPUT")
                .required(true)
                .num_args(1)
                .index(1)
                .help("input CSV file"),
        )
        .arg(
            Arg::new("CODING")
                .short('c')
                .long("coding")
                .required(false)
                .num_args(1)
                .default_value("top")
                .help("Desired allele coding"),
        )
        .arg(
            Arg::new("MAP")
                .short('m')
                .long("map")
                .required(false)
                .num_args(1)
                .help("Map of the variant positions"),
        )
        .arg(
            Arg::new("OUTPUT")
                .required(false)
                .short('o')
                .long("output")
                .num_args(1)
                .help("Output file prefix")
                .default_value("output"),
        )
        .get_matches();

    let filename = matches.get_one::<String>("INPUT").unwrap();
    let coding = matches.get_one::<String>("CODING").unwrap();
    let prefix = matches.get_one::<String>("OUTPUT").unwrap();
    let map = matches.get_one::<String>("MAP");

    // Process the CSV files
    let _ = process_csv(filename, coding, prefix, map);

    let duration = start.elapsed();
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    println!(
        "Report converted: {:02}:{:02}:{:02}",
        hours, minutes, seconds
    );
}

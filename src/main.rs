extern crate clap;
use clap::{Arg, Command};

mod map;
mod report;
use map::load_map;
use report::process_csv;

fn main() {
    let matches = Command::new("ilmn2ped")
        .version("0.1.0")
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

    // Import a map if it is available
    if let Some(map) = map {
        let _variant_map = load_map(map);
    }
    

    let _ = process_csv(filename, coding, prefix);
}

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, stdin};

extern crate clap;
use clap::{Command, Arg};

fn process_csv(file_path: &str, prefix: &str) -> Result<(), Box<dyn std::error::Error>>{
    // Switch to start converting to PED
    let mut start_parsing: bool = false;

    // Input readers, accepting either a file or stdin
    let reader: Box<dyn Read> = if file_path == "-" {
        Box::new(stdin().lock())
    } else {
        let file = File::open(file_path)?
        Box::new(BufReader::new(file))
    };
    let reader = BufReader::new(reader);

    for line in reader.lines(){
        let line = line?;
        if !line.is_empty(){
            let split_line = line.split(' ');
        }
    }

    Ok(())
}


fn main() {
    let matches = Command::new("fastix")
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
            Arg::new("OUTPUT")
                .short('o')
                .long("output")
                .num_args(1)
                .help("Output file prefix"),
        )
        .get_matches();

    let filename = matches.get_one::<String>("INPUT").unwrap();

    let prefix = matches.get_one::<String>("OUTPUT").unwrap();

    process_csv(filename, prefix);

    println!("Hello, world!");
}

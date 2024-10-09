use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, stdin};

extern crate clap;
use clap::{Command, Arg};

fn process_csv(file_path: &str, prefix: &str) -> Result<(), Box<dyn std::error::Error>>{
    // Switch to start converting to PED
    let mut start_parsing: bool = false;
    let mut a1_index: u8;
    let mut a2_index: u8;

    // Input readers, accepting either a file or stdin
    let reader: Box<dyn Read> = if file_path == "-" {
        Box::new(stdin().lock())
    } else {
        Box::new(BufReader::new(File::open(file_path)?))
    };
    let reader = BufReader::new(reader);

    for line in reader.lines(){
        let line = line?;
        if !line.is_empty(){
            if line.contains("[input]") {
                continue;
            } else if line.contains("[data]") {
                start_parsing = true;
            } else if !start_parsing {
                let split_line = line.split('=');
                let split_line: Vec<&str> = split_line.collect();
                let key = split_line[0];
                let val: i64 = split_line[1].parse().unwrap();
                println!("{}: {}", key, val);
            }
        }
    }
    Ok(())
}


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
            Arg::new("OUTPUT")
                .required(false)
                .short('o')
                .long("output")
                .num_args(1)
                .help("Output file prefix")
                .default_value("None"),
        )
        .get_matches();

    let filename = matches.get_one::<String>("INPUT").unwrap();

    let prefix = matches.get_one::<String>("OUTPUT").unwrap();

    process_csv(filename, prefix);

}

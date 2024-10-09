use std::fs::File,
use std::io::{self, BufRead, BufReader, Read, stdin};

extern crate clap;
use clap::{Command, Arg};

fn read_text_file_lines<R: Read>(reader: R) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(reader);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

fn process_csv(fasta_filename: &str, prefix: &str) {
    // Switch to start converting to PED
    let mut start_parsing: bool = false

    // Input readers, accepting either a file or stdin
    let reader: Box<dyn Read> = if let Some(file_path) = std::env::args().nth(1) {
        Box::new(BufReader::new(File::open(file_path)?))
    } else {
        Box::new(stdin().lock())
    };

    for line in read_text_file_lines(fasta_filename){
        if !l.is_empty() {
            println!("{}", l);
        }
    }
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

    process_csv(filename, prefix)

    println!("Hello, world!");
}

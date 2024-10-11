use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Write};

extern crate clap;
use clap::{Arg, Command};

// Validate coding provided
fn get_valid_column_id(coding: &str, allele: &str) -> Result<String, String> {
    let colname = match coding {
        "forward" => format!("Allele{} - Forward", allele),
        "reverse" => format!("Allele{} - Reverse", allele),
        "top" => format!("Allele{} - Top", allele),
        "bottom" => format!("Allele{} - Bottom", allele),
        "ab" => format!("Allele{} - AB", allele),
        _ => String::from("invalid"),
    };
    if colname == "invalid" {
        Err(String::from("Invalid coding provided"))
    } else {
        Ok(colname)
    }
}

fn process_csv(
    file_path: &str,
    coding: &str,
    out_root: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start parsing the data
    let mut start_parsing: bool = false;
    // Sample names and number of expected alleles
    let mut sample_name: Option<String> = None;
    let mut num_alleles: i64 = 0;
    // Vectors of minimal informations
    let mut genotypes: Vec<String> = vec![];
    let mut variants: Vec<String> = vec![];
    // Index of key columns
    let mut a1_index: usize = 4;
    let mut a2_index: usize = 5;
    let mut sample_index: usize = 0;
    let mut snp_index: usize = 1;
    // Create output files
    let mut pedfile = File::create(format!("{out_root}.ped"))?;
    let mut mapfile = File::create(format!("{out_root}.map"))?;

    // Define input file delimiter based on the suffix
    let delimiter = if file_path.ends_with(".csv") {
        ","
    } else {
        "\t"
    };

    // Define the input dataset
    let reader: Box<dyn Read> = if file_path == "-" {
        Box::new(stdin().lock())
    } else {
        Box::new(BufReader::new(File::open(file_path)?))
    };
    let reader = BufReader::new(reader);

    // Process the input file
    for line_result in reader.lines() {
        // Unpack the line
        let line = line_result?;
        if !line.is_empty() {
            let split_line: Vec<&str> = line.split(delimiter).collect();

            if line.contains("[Header]") {
                continue;
            } else if line.contains("[Data]") {
                start_parsing = true;
            } else if !start_parsing {
                let key = split_line[0];
                if key == "Num SNPs" {
                    let num_sites: i64 = split_line[1].parse()?;
                    num_alleles = num_sites * 2;
                    genotypes = Vec::with_capacity(num_alleles as usize);
                    println!("Found {} sites", num_sites);
                }
            } else {
                if line.contains("Sample Name") {
                    let a1_name = get_valid_column_id(coding, "1").unwrap();
                    let a2_name = get_valid_column_id(coding, "2").unwrap();

                    if !split_line.contains(&a1_name.as_str()) {
                        panic!("Column \"{}\" not found", a1_name);
                    }
                    if !split_line.contains(&a2_name.as_str()) {
                        panic!("Column \"{}\" not found", a2_name);
                    }

                    a1_index = split_line
                        .iter()
                        .position(|&value| value == a1_name)
                        .unwrap();
                    a2_index = split_line
                        .iter()
                        .position(|&value| value == a2_name)
                        .unwrap();
                    sample_index = split_line
                        .iter()
                        .position(|&value| value == "Sample Name")
                        .unwrap();
                    snp_index = split_line
                        .iter()
                        .position(|&value| value == "SNP Index")
                        .unwrap();

                    println!("Desired column {} has index {}", a1_name, a1_index);
                    println!("Desired column {} has index {}", a2_name, a2_index);
                } else {
                    let local_sample = split_line[sample_index].to_string();
                    if !variants.contains(&split_line[snp_index].to_string()) {
                        variants.push(split_line[snp_index].to_string())
                    }
                    if sample_name.is_none() {
                        sample_name = Some(local_sample.clone());
                        genotypes = Vec::with_capacity(num_alleles as usize);
                    } else if sample_name.as_ref().unwrap() != &local_sample {
                        let sample = sample_name.take().unwrap();
                        writeln!(pedfile, "{sample} {sample} 0 0 -9 {}", genotypes.join(" "))?;
                        sample_name = Some(local_sample.clone());
                        genotypes = Vec::with_capacity(num_alleles as usize);
                    }

                    let a1: String = split_line[a1_index].to_string().replace("-", "0");
                    let a2: String = split_line[a2_index].to_string().replace("-", "0");
                    let site_idx: i64 = split_line[snp_index].parse().unwrap();
                    let a1_pos: usize = ((site_idx - 1) * 2) as usize;
                    let a2_pos: usize = a1_pos + 1;

                    if a1_pos < genotypes.len() {
                        genotypes[a1_pos] = a1;
                    } else {
                        genotypes.insert(a1_pos, a1);
                    }

                    if a2_pos < genotypes.len() {
                        genotypes[a2_pos] = a2;
                    } else {
                        genotypes.insert(a2_pos, a2);
                    }
                }
            }
        }
    }
    // Save the last line to the output file
    if let Some(sample) = sample_name {
        writeln!(pedfile, "{sample} {sample} 0 0 -9 {}", genotypes.join(" "))?;
    }
    // Save the map file
    for site in variants {
        writeln!(mapfile, "0 {site} 0 0")?;
    }
    Ok(())
}

fn main(){
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
    let coding = matches.get_one::<String>("CODING").unwrap();
    let prefix = matches.get_one::<String>("OUTPUT").unwrap();

    let _ = process_csv(filename, coding, prefix);
}

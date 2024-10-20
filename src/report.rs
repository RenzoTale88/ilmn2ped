mod map;
use map::{load_map, Site};
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Write};

// Define a function to validate the header.
fn validate_header(header: Vec<&str>, coding: &str) -> Result<String, String> {

    let a1_col = get_valid_column_id(coding, "1").unwrap();
    let a2_col = get_valid_column_id(coding, "2").unwrap();
    let has_a1_col: bool = header.contains(&a1_col.as_str());
    let has_a2_col: bool = header.contains(&a2_col.as_str());
    let has_allele_cols: bool = has_a1_col && has_a2_col;
    let has_sample_col: bool = header.contains(&"Sample Name") || header.contains(&"Sample ID");
    let too_many_sample_col: bool =
        header.contains(&"Sample Name") && header.contains(&"Sample ID");
    let has_snp_name: bool = header.contains(&"SNP Name");
    let has_snp_index: bool = header.contains(&"SNP Index");
    if has_allele_cols && has_sample_col && !too_many_sample_col && (has_snp_index || has_snp_name)
    {
        let result = if has_snp_index {
            String::from("SNP Index")
        } else {
            String::from("SNP Name")
        };
        Ok(String::from(result))
    } else if !has_allele_cols {
        Err(String::from(
            "Invalid header: missing \"Allele1\" or \"Allele2\" columns",
        ))
    } else if !too_many_sample_col {
        Err(String::from(
            "Invalid header: found both Sample Name or Sample ID columns",
        ))
    } else if !has_sample_col {
        Err(String::from(
            "Invalid header: missing Sample Name or Sample ID column",
        ))
    } else {
        Err(String::from(
            "Invalid header: missing SNP Index or SNP Name column",
        ))
    }
}

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

pub fn process_csv(
    file_path: &str,
    coding: &str,
    out_root: &str,
    map: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Start parsing the data
    let mut start_parsing: bool = false;
    let mut end_parsing: bool = false;
    let mut parse_header: bool = false;
    // Sample names and number of expected alleles
    // Sample name can be null at the beginning, so that we can
    // initialize it at the first line of the table.
    let mut sample_name: Option<String> = None;
    let mut num_alleles: i64 = 0;
    // Vectors of minimal informations
    let mut genotypes: Vec<String> = vec![];
    let mut variants: Vec<String> = vec![];
    // Hashmap of positions to process, if map is provided
    let mut site_metadata: Option<HashMap<String, Site>> = None;
    // Index of key columns
    let mut a1_index: usize = 4;
    let mut a2_index: usize = 5;
    let mut sample_col_index: usize = 0;
    let mut snp_col_index: usize = 1;
    let mut site_idx: usize = 0;

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
    let mut reader = BufReader::new(reader);
    let mut line: String = String::new();

    // Parse the header first
    while !start_parsing {
        reader.read_line(&mut line)?;
        if !line.is_empty() {
            let split_line: Vec<&str> = line.trim().split(delimiter).collect();
            if parse_header {
                // Parse the header for the right indexes
                let validated = validate_header(split_line.clone(), coding);
                match validated {
                    Ok(snp_col_name) => {
                        // First, we validate the header
                        let a1_name = get_valid_column_id(coding, "1").unwrap();
                        let a2_name = get_valid_column_id(coding, "2").unwrap();
                        // Get allele input columns
                        a1_index = split_line
                        .iter()
                        .position(|&value| value == a1_name)
                        .unwrap();
                        a2_index = split_line
                        .iter()
                        .position(|&value| value == a2_name)
                        .unwrap();
                
                        // Sample column index
                        sample_col_index = split_line
                        .iter()
                        .position(|&value| value.to_lowercase().contains(&"sample"))
                        .unwrap();

                        // SNP column index
                        snp_col_index = split_line
                        .iter()
                        .position(|&value| value.contains(&snp_col_name))
                        .unwrap();
                    },
                    Err(e) => panic!("{e}"),
                };
                println!("Columns for coding {} found.", coding);
                start_parsing = true;
            // Get expected number of variants in the dataset
            } else if split_line.contains(&"[Data]") {
                parse_header = true;
            } else if line.contains("Num SNPs"){
                let num_sites: i64 = split_line[1].parse()?;
                num_alleles = num_sites * 2;
                genotypes = Vec::with_capacity(num_alleles as usize);
                println!("Found {} sites", num_sites);
            }
        }
        line = String::new();
    }
    println!("Start parsing the data");
    // Load map file, if provided
    // if !map.is_none() {
    //     site_metadata = Some(load_map(map.unwrap(), snp_col_name)?);
    // };

    // Print warning
    // Create output files
    let mut pedfile = File::create(format!("{out_root}.ped"))?;
    let mut mapfile = File::create(format!("{out_root}.map"))?;

    // Then process the sites
    while !end_parsing {
    // for line_result in reader.lines() {
        // Unpack the line
        reader.read_line(&mut line)?;
        if !line.is_empty() {
            let split_line: Vec<&str> = line.trim().split(delimiter).collect();
            let local_sample = split_line[sample_col_index].to_string();
            let local_var = split_line[snp_col_index].to_string();
            if !variants.contains(&local_var) {
                variants.push(local_var)
            }
            if sample_name.is_none() {
                sample_name = Some(local_sample.clone());
                genotypes = Vec::with_capacity(num_alleles as usize);
            } else if sample_name.as_ref().unwrap() != &local_sample {
                site_idx = 0;
                let sample = sample_name.take().unwrap();
                writeln!(
                    pedfile,
                    "{sample} {sample} 0 0 0 -9 {}",
                    genotypes.join(" ")
                )?;
                sample_name = Some(local_sample.clone());
                genotypes = Vec::with_capacity(num_alleles as usize);
            }

            let a1: String = split_line[a1_index].to_string().replace("-", "0");
            let a2: String = split_line[a2_index].to_string().replace("-", "0");
            let a1_pos: usize = ((site_idx) * 2) as usize;
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
            site_idx += 1;
            line = String::new();
        } else {
            end_parsing = true;
        }
    }
    // Save the last line to the output file
    if let Some(sample) = sample_name {
        writeln!(
            pedfile,
            "{sample} {sample} 0 0 0 -9 {}",
            genotypes.join(" ")
        )?;
    }
    // Save the map file
    let mut chrom: String = String::from("0");
    let mut pos: i64 = 0;
    for site in variants {
        match site_metadata {
            Some(ref meta) => {
                chrom = meta[&site].chromosome.clone();
                pos = meta[&site].position.clone();
            }
            _ => {}
        };
        writeln!(mapfile, "{chrom}\t{site}\t0\t{pos}")?;
    }
    Ok(())
}

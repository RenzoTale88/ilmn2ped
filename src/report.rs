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
        Ok(result)
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
    let mut parse_header: bool = false;
    let mut snp_column: String = String::new();
    // Sample names and number of expected alleles
    // Sample name can be null at the beginning, so that we can
    // initialize it at the first line of the table.
    let mut sample_name: String = String::new();
    // Vectors of minimal informations
    let mut variants: Vec<String> = vec![];
    // Hashmap of positions to process, if map is provided
    let mut site_metadata: Option<HashMap<String, Site>> = None;
    // Index of key columns
    let mut a1_index: usize = 4;
    let mut a2_index: usize = 5;
    let mut sample_col_index: usize = 0;
    let mut snp_col_index: usize = 1;

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
                            .position(|&value| value.to_lowercase().contains("sample"))
                            .unwrap();

                        // SNP column index
                        snp_col_index = split_line
                            .iter()
                            .position(|&value| value.contains(&snp_col_name))
                            .unwrap();
                        // Set SNP column name
                        snp_column = snp_col_name;
                    }
                    Err(e) => panic!("{e}"),
                };
                println!("Columns for coding {} found.", coding);
                start_parsing = true;
            // Get expected number of variants in the dataset
            } else if split_line.contains(&"[Data]") {
                parse_header = true;
            } else if line.contains("Num SNPs") {
                let num_sites: i64 = split_line[1].parse()?;
                println!("Found {} sites", num_sites);
            }
        }
        line = String::new();
    }
    // Load map file, if provided
    println!("Load mapping information");
    if map.is_some() {
        println!("{snp_column}");
        site_metadata = Some(load_map(map.unwrap(), snp_column)?);
    };

    // Start outputting stuff
    // Create output files
    println!("Writing ped file...");
    let mut pedfile = File::create(format!("{out_root}.ped"))?;
    let mut mapfile = File::create(format!("{out_root}.map"))?;

    // First, we check the first line
    reader.read_line(&mut line)?;
    if !line.is_empty() {
        let split_line: Vec<&str> = line.trim().split(delimiter).collect();
        let local_sample = split_line[sample_col_index].to_string();
        let a1: String = split_line[a1_index].to_string().replace("-", "0");
        let a2: String = split_line[a2_index].to_string().replace("-", "0");
        variants.push(split_line[snp_col_index].to_string());
        sample_name = local_sample.clone();
        write!(pedfile, "{local_sample} {local_sample} 0 0 0 -9 {a1} {a2}")?;
    }

    // Then process the rest of the sites
    for line_result in reader.lines() {
        // Unpack the line
        let line = line_result?;
        if line.len() < a2_index {
            panic!("Line is truncated!")
        }
        let split_line: Vec<&str> = line.trim().split(delimiter).collect();
        let local_sample = split_line[sample_col_index].to_string();
        let a1: String = split_line[a1_index].replace("-", "0");
        let a2: String = split_line[a2_index].replace("-", "0");
        variants.push(split_line[snp_col_index].to_string());
        // Check and print if it is none or a sample name
        if sample_name != local_sample {
            writeln!(pedfile)?;
            write!(pedfile, "{local_sample} {local_sample} 0 0 0 -9")?;
            sample_name = local_sample.clone();
        }
        write!(pedfile, " {a1} {a2}")?;
    }
    writeln!(pedfile)?;

    // Save the map file
    println!("Writing map file...");
    let mut chrom: String = String::from("0");
    let mut name: String;
    let mut pos: i64 = 0;
    let mut processed: Vec<String> = vec![];
    for site in variants {
        if !processed.contains(&site) {
            processed.push(site.clone());
        } else {
            break;
        }
        match site_metadata {
            Some(ref meta) => {
                chrom = meta[&site].chromosome.clone();
                pos = meta[&site].position;
                name = meta[&site].name.clone();
            }
            _ => {
                name = site.clone();
            }
        };
        writeln!(mapfile, "{chrom}\t{name}\t0\t{pos}")?;
    }
    Ok(())
}

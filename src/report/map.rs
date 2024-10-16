use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Structure pointing to a site and its coordinates
pub struct Site {
    pub chromosome: String,
    pub position: i64,
}

// Define a function to validate the header
fn validate_header(header: Vec<&str>) -> Result<bool, &str> {
    if !header.contains(&"Name") {
        Err("Invalid header: missing \"Name\" column")
    } else if !header.contains(&"Chromosome") {
        Err("Invalid header: missing \"Chromosome\" column")
    } else if !header.contains(&"Position") {
        Err("Invalid header: missing \"Position\" column")
    } else {
        Ok(true)
    }
}

// Function to load a map file
pub fn load_map(map: &str, snp_target: String) -> io::Result<HashMap<String, Site>> {
    // Header processed?
    let mut header_processed = false;
    let file = File::open(map);
    let reader = BufReader::new(file?);
    // Create hashmap of sites.
    let mut site_map: HashMap<String, Site> = HashMap::new();
    // Define targets
    let mut key_idx: usize = 0;
    let mut chrom_idx: usize = 0;
    let mut pos_idx: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let split_line: Vec<&str> = line.split('\t').collect();
            if !header_processed {
                // Validate header first.
                let validation = validate_header(split_line.clone());
                match validation {
                    Ok(_) => {
                        // Example lines:
                        // Index	Name	Chromosome	Position	GenTrain Score	SNP	ILMN Strand	Customer Strand NormID
                        //1	snp1	1	10673082	0.7923	[T/C]	BOT	BOT	0
                        key_idx = if snp_target == "SNP Index" {
                            split_line
                                .iter()
                                .position(|&value| value == "Index")
                                .unwrap()
                        } else {
                            split_line
                                .iter()
                                .position(|&value| value == "Name")
                                .unwrap()
                        };
                        chrom_idx = split_line
                            .iter()
                            .position(|&value| value == "Chromosome")
                            .unwrap();
                        pos_idx = split_line
                            .iter()
                            .position(|&value| value == "Position")
                            .unwrap();
                    }
                    Err(s) => panic!("{s}"),
                };
                header_processed = true;
            } else {
                let key = split_line[key_idx].to_string();
                let chrom = split_line[chrom_idx].to_string();
                let pos = split_line[pos_idx].parse();
                // Ensure that pos is an integer
                let pos = match pos {
                    Ok(v) => v,
                    Err(e) => panic!("{e}"),
                };
                let site = Site {
                    chromosome: chrom,
                    position: pos,
                };
                site_map.insert(key, site);
            }
            println!("{}", line);
        }
    }

    Ok(site_map)
}

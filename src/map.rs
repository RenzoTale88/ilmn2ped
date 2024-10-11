use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Write};

// Structure with the basic data.
struct variant {
    chromosome: String,
    position: i64,
    name: String
}

// Function to load a map file
pub fn load_map(map: &str) -> Result<(), ()> {
    let mut header = true;
    let file = File::open(map.to_string());
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let split_line: Vec<&str> = line.split('\t').collect();
            if header {
                header = false
            } else {

            }
        println!("{}", line);
        }
    }

    Ok(())
}

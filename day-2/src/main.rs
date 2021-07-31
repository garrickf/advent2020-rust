use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Part One
fn main() {
    let lines = read_input_from_file().expect("Error reading file.");

    let mut num_valid_pwds = 0;
    for line in lines {
        let v: Vec<&str> = line.split(":").collect();
        let rule = v[0];
        let pwd = v[1].trim();

        let v: Vec<&str> = rule.split(" ").collect();
        let rule_occurs = v[0];
        let rule_char = v[1];

        let v: Vec<&str> = rule_occurs.split("-").collect();
        let min_occurs: u32 = v[0].parse().expect("Error parsing number.");
        let max_occurs: u32 = v[1].parse().expect("Error parsing number.");

        let mut count: u32 = 0;
        for c in pwd.chars() {
            if c.to_string() == rule_char {
                count += 1;
            }
        }

        if count >= min_occurs && count <= max_occurs {
            // Password is valid
            num_valid_pwds += 1;
        }
    }

    println!("Number of valid passwords: {}", num_valid_pwds);
}

fn read_input_from_file() -> Result<Vec<String>, io::Error> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines = Vec::new();
    for line in buffered.lines() {
        let line: String = line?.trim().to_string();
        lines.push(line);
    }

    Ok(lines)
}

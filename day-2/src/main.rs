use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let data = read_input_from_file().expect("Error reading file.");
    part_one(&data);
    part_two(&data);
}

// There is a minimum and maximum number of a rule character allowed in each
// password.
fn part_one(data: &Vec<(u32, u32, String, String)>) {
    let mut num_valid_pwds = 0;
    for (min_occurs, max_occurs, rule_char, pwd) in data {
        let mut count: u32 = 0;
        for c in pwd.chars() {
            if c.to_string() == *rule_char {
                count += 1;
            }
        }
        if count >= *min_occurs && count <= *max_occurs {
            num_valid_pwds += 1;
        }
    }

    println!("Number of valid passwords: {}", num_valid_pwds);
}

// Exactly one of two positions must contain the rule character.
fn part_two(data: &Vec<(u32, u32, String, String)>) {
    let mut num_valid_pwds = 0;
    for (pos_a, pos_b, rule_char, pwd) in data {
        let pwd = pwd.as_bytes();
        let pos_a = *pos_a as usize - 1; // Positions are one-indexed
        let pos_b = *pos_b as usize - 1;
        let rule_char = rule_char.as_bytes()[0];
        if pwd[pos_a] != pwd[pos_b] && (pwd[pos_a] == rule_char || pwd[pos_b] == rule_char) {
            num_valid_pwds += 1;
        }
    }

    println!("Number of valid passwords: {}", num_valid_pwds);
}

fn read_input_from_file() -> Result<Vec<(u32, u32, String, String)>, io::Error> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut data = Vec::new();
    for line in buffered.lines() {
        let line: String = line?.trim().to_string();
        data.push(parse_line(line));
    }

    Ok(data)
}

// Consumes and parses line of form `num1`-`num2` `char`: `password`.
fn parse_line(line: String) -> (u32, u32, String, String) {
    let v: Vec<&str> = line.split(":").collect();
    let rule = v[0];
    let pwd = v[1].trim().to_string();

    let v: Vec<&str> = rule.split(" ").collect();
    let rule_range = v[0];
    let rule_char = v[1].to_string();

    let v: Vec<&str> = rule_range.split("-").collect();
    let rule_num_1: u32 = v[0].parse().expect("Error parsing number.");
    let rule_num_2: u32 = v[1].parse().expect("Error parsing number.");

    return (rule_num_1, rule_num_2, rule_char, pwd);
}

//! Day 6: Custom Customs

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

/// Runs the two parts of the Custom Customs problem.
fn main() {
    let input = read_input_from_file("data/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

/// Runs Part One: count the number of questions to which anyone answered "yes"
fn part_one(input: &Vec<String>) {
    let mut set = HashSet::new();
    let mut num_yes = 0;
    for line in input.iter() {
        for char in line.chars() {
            set.insert(char);
        }

        // trim() removes the "\n", leaving "" between groups
        if line == "" {
            num_yes += set.len();

            set.clear();
        }
    }

    // Account for last group
    num_yes += set.len();

    println!("number of questions anyone answered 'yes': {}", num_yes);
}

/// Runs Part Two: count the number of questions to which everyone answered "yes"
fn part_two(input: &Vec<String>) {
    let mut num_yes = 0;
    let mut line_idx = 0;
    loop {
        if line_idx >= input.len() {
            break;
        }

        // Process a single group
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut num_yes_for_group = 0;
        let mut num_people = 0;
        loop {
            let line = match input.get(line_idx) {
                Some(line) => line,
                None => break,
            };

            for char in line.chars() {
                map.insert(char, 1 + map.get(&char).copied().unwrap_or_default());
            }

            if line == "" {
                line_idx += 1; // End group and move to the next line
                break;
            } else {
                num_people += 1;
            }

            line_idx += 1;
        }

        for (_key, val) in map.iter() {
            if *val == num_people {
                num_yes_for_group += 1;
            }
        }

        num_yes += num_yes_for_group;
    }
    println!("number of questions everyone answered 'yes': {}", num_yes);
}

/// Reads input from file
fn read_input_from_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut input = Vec::new();
    for line in buffered.lines() {
        let trimmed = line?.trim().to_string();
        input.push(trimmed);
    }

    Ok(input)
}

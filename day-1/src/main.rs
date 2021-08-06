use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn read_input_from_file() -> Result<Vec<i32>, Box<dyn Error>> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut nums = Vec::new();
    for line in buffered.lines() {
        let num = line?.trim().parse()?;
        nums.push(num);
    }

    Ok(nums)
}

// Find the product of two numbers that sum to 2020
fn part_one(nums: &Vec<i32>) {
    let mut map = HashMap::new();

    for a in nums {
        let key = 2020 - a;
        map.insert(key, a);
    }

    for b in nums {
        if map.contains_key(b) {
            let a = map[b];
            println!("Found!");
            println!("{} + {} = 2020", a, b);
            println!("{} * {} = {}", a, b, a * b);
            break;
        }
    }
}

// Find the product of three numbers that sum to 2020
fn part_two(nums: &Vec<i32>) {
    let mut map = HashMap::new();
    for (ai, a) in nums.iter().enumerate() {
        for b in &nums[ai..] {
            let pair = (a, b);
            let key = 2020 - a - b;
            map.insert(key, pair);
        }
    }

    for c in nums {
        if map.contains_key(c) {
            let (a, b) = map[c];
            println!("Found!");
            println!("{} + {} + {} = 2020", a, b, c);
            println!("{} * {} * {} = {}", a, b, c, a * b * c);
            break;
        }
    }
}

fn main() {
    let nums = read_input_from_file().unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        process::exit(1);
    });

    println!("Part One\n---");
    part_one(&nums);
    println!("\nPart Two\n---");
    part_two(&nums);
}

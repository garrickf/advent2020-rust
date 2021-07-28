use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_input_from_file() -> Result<Vec<i32>, io::Error> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut nums = Vec::new();
    for line in buffered.lines() {
        let num = line?
            .trim()
            .parse()
            .expect("Could not parse numbers from file.");
        nums.push(num);
    }

    Ok(nums)
}

fn main() {
    let nums = read_input_from_file().expect("Error reading file.");

    let mut map = HashMap::new();
    for (ai, a) in nums.iter().enumerate() {
        for b in &nums[ai..] {
            let pair = (a, b);
            let key = 2020 - a - b;
            map.insert(key, pair);
        }
    }

    for c in &nums {
        if map.contains_key(c) {
            let (a, b) = map[c];
            println!("Found {}, {}, {}; product: {}", a, b, c, a * b * c);
            break;
        }
    }
}

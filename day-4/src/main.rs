use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = read_input_from_file().expect("Error reading file.");
    part_one(&data);
}

fn part_one(data: &Vec<PassportData>) {
    let mut num_valid = 0;
    for passport in data {
        if is_valid(passport) {
            num_valid += 1;
        }
    }

    println!("Valid passports: {}", num_valid);
}

#[derive(Default, Debug)]
struct PassportData {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expire_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn read_input_from_file() -> Result<Vec<PassportData>, Box<dyn Error>> {
    let path = "data/input.txt";
    let file = File::open(path)?;
    let mut buffered = BufReader::new(file);

    let mut data = Vec::new();
    loop {
        let mut passport: PassportData = Default::default();
        loop {
            let mut line = String::new();
            let len = buffered.read_line(&mut line)?;
            if len == 0 {
                data.push(passport);
                return Ok(data);
            }

            line = line.trim().to_string();
            if line == "" {
                break;
            }

            for attribute in line.split(" ") {
                let mut iter = attribute.split(":");
                let key = iter.next().unwrap();
                let value = iter.next().unwrap();

                match key {
                    "byr" => {
                        passport.birth_year = Some(value.to_string());
                    }
                    "iyr" => {
                        passport.issue_year = Some(value.to_string());
                    }
                    "eyr" => {
                        passport.expire_year = Some(value.to_string());
                    }
                    "hgt" => {
                        passport.height = Some(value.to_string());
                    }
                    "hcl" => {
                        passport.hair_color = Some(value.to_string());
                    }
                    "ecl" => {
                        passport.eye_color = Some(value.to_string());
                    }
                    "pid" => {
                        passport.passport_id = Some(value.to_string());
                    }
                    "cid" => {
                        passport.country_id = Some(value.to_string());
                    }
                    _ => {}
                }
            }
        }

        data.push(passport);
    }
}

fn is_valid(passport: &PassportData) -> bool {
    // It's okay if the country id (cid) is missing
    return passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expire_year.is_some()
        && passport.height.is_some()
        && passport.hair_color.is_some()
        && passport.eye_color.is_some()
        && passport.passport_id.is_some();
}

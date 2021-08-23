// A little heavyweight, I also do a lot of parse work to pull out the properties,
// which I don't really need for the problem.
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut data = read_input_from_file().expect("Error reading file.");
    data = part_one(data);
    part_two(data);
}

fn part_one(mut data: Vec<RawPassportData>) -> Vec<RawPassportData> {
    data = data
        .into_iter()
        .filter(|x| has_required_fields(x))
        .collect();

    println!("Valid passports: {}", data.len());
    data
}

fn part_two(data: Vec<RawPassportData>) {
    let data: Vec<PassportData> = data.into_iter().filter_map(|x| parse(x)).collect();

    println!("Valid passports: {}", data.len());
}

#[derive(Default, Debug)]
struct RawPassportData {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expire_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Unit {
    Cm,
    In,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Cm
    }
}

#[derive(Default, Debug)]
struct PassportData {
    birth_year: i32,
    issue_year: i32,
    expire_year: i32,
    height: i32,
    height_unit: Unit,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

fn read_input_from_file() -> Result<Vec<RawPassportData>, Box<dyn Error>> {
    let path = "data/input.txt";
    let file = File::open(path)?;
    let buffered = BufReader::new(file);

    let mut data = Vec::new();
    let mut passport: RawPassportData = Default::default();
    for line in buffered.lines() {
        let line = line?.trim().to_string();
        if line == "" {
            data.push(passport);
            passport = Default::default();
        } else {
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
    }
    data.push(passport);
    Ok(data)
}

fn has_required_fields(passport: &RawPassportData) -> bool {
    // It's okay if the country id (cid) is missing
    return passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expire_year.is_some()
        && passport.height.is_some()
        && passport.hair_color.is_some()
        && passport.eye_color.is_some()
        && passport.passport_id.is_some();
}

#[derive(Debug)]
struct PassportParseError;

impl fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing passport data.")
    }
}

impl Error for PassportParseError {}

// Attempts parse. Returns None if parse fails or data invalid
fn try_parse(raw: RawPassportData) -> Result<PassportData, Box<dyn Error>> {
    let mut parsed: PassportData = Default::default();
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r#"^([\d]+)(in|cm)$"#).unwrap(); // raw string
        static ref HEX_COLOR_RE: Regex = Regex::new(r#"^#[a-f\d]{6}$"#).unwrap();
        static ref EYE_COLOR_RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PASSPORT_ID_RE: Regex = Regex::new(r#"^[\d]{9}$"#).unwrap();
    }

    parsed.birth_year = raw.birth_year.unwrap().parse()?;
    parsed.issue_year = raw.issue_year.unwrap().parse()?;
    parsed.expire_year = raw.expire_year.unwrap().parse()?;

    let height = raw.height.unwrap();
    let hair_color = raw.hair_color.unwrap();
    let eye_color = raw.eye_color.unwrap();
    let passport_id = raw.passport_id.unwrap();

    if !HEIGHT_RE.is_match(&height)
        || !HEX_COLOR_RE.is_match(&hair_color)
        || !EYE_COLOR_RE.is_match(&eye_color)
        || !PASSPORT_ID_RE.is_match(&passport_id)
    {
        return Err(Box::new(PassportParseError));
    }

    let caps = HEIGHT_RE.captures(&height).unwrap();
    parsed.height = caps[1].parse()?;
    parsed.height_unit = match &caps[2] {
        "in" => Unit::In,
        "cm" => Unit::Cm,
        _ => panic!("Not possible"),
    };

    parsed.hair_color = hair_color;
    parsed.eye_color = eye_color;
    parsed.passport_id = passport_id;
    parsed.country_id = raw.country_id;
    Ok(parsed)
}

fn parse(raw: RawPassportData) -> Option<PassportData> {
    match try_parse(raw) {
        Ok(x) => {
            if x.birth_year < 1920
                || x.birth_year > 2002
                || x.issue_year < 2010
                || x.issue_year > 2020
                || x.expire_year < 2020
                || x.expire_year > 2030
                || x.height_unit == Unit::Cm && (x.height < 150 || x.height > 193)
                || x.height_unit == Unit::In && (x.height < 59 || x.height > 76)
            {
                None
            } else {
                Some(x)
            }
        }
        Err(_) => None,
    }
}

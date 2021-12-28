// Day 5: Binary Boarding

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Small testing lines
    let test = BoardingPosition::from_boarding_pass("FBFBBFFRLR".to_owned()).unwrap();
    println!("row {}, col {}, id {}", test.row, test.col, test.get_id());

    let mut plane = [[false; 8]; 128];

    // Part One
    let boarding_passes = read_input_from_file().unwrap();
    let mut highest_id = -1;
    for boarding_pass in boarding_passes {
        let pos = BoardingPosition::from_boarding_pass(boarding_pass).unwrap();
        if pos.get_id() > highest_id {
            highest_id = pos.get_id();
        }
        plane[pos.row as usize][pos.col as usize] = true;
    }

    println!("highest id: {}", highest_id);

    // Part Two: Find missing seat
    let mut seen_start = false;
    for row in 0..128 {
        for col in 0..8 {
            if plane[row][col] {
                seen_start = true;
            } else if seen_start {
                println!("missing row {}, col {}, id {}", row, col, row * 8 + col);
                return;
            }
        }
    }
}

fn read_input_from_file() -> Result<Vec<String>, Box<dyn Error>> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut input = Vec::new();
    for line in buffered.lines() {
        let trimmed = line?.trim().to_string();
        input.push(trimmed);
    }

    Ok(input)
}

// The BoardingPassError class is used for errors related to parsing or using a
// boarding pass.
#[derive(Debug)]
struct BoardingPassError {
    msg: String,
}

impl fmt::Display for BoardingPassError {
    // NOTE: lifetime elision, see https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// The BoardingPosition class is a row and column on the plane.
struct BoardingPosition {
    row: i32,
    col: i32,
}

impl BoardingPosition {
    fn is_valid(&self) -> bool {
        // Helper function that checks if the boarding position is in bounds.
        return self.row >= 0 && self.row < 128 && self.col >= 0 && self.col < 8;
    }

    fn get_id(&self) -> i32 {
        // Returns the id of the boarding position.
        return self.row * 8 + self.col;
    }

    fn from_boarding_pass(code: String) -> Result<Self, BoardingPassError> {
        // Associated function that constructs a BoardingPosition from a boarding
        // pass string.
        let (mut row_lo, mut row_hi) = (0, 127);
        let (mut col_lo, mut col_hi) = (0, 7);
        for c in code.chars() {
            // NOTE: the below maths only work because 127 and 7 repeatedly truncate
            // to odd numbers when dividing by two. Thus we truncate using int
            // division and always add one to get the amount to adjust a bound
            // by.
            //
            // If a distance were ever to be even (as can happen with other sizes),
            // we only want adjust the new bound by that number / 2.
            match c {
                'F' => {
                    // Take the lower half of rows
                    row_hi -= 1 + (row_hi - row_lo) / 2;
                }
                'B' => {
                    // Take the upper half of rows
                    row_lo += 1 + (row_hi - row_lo) / 2;
                }
                'L' => {
                    // Take the left half of cols
                    col_hi -= 1 + (col_hi - col_lo) / 2;
                }
                'R' => {
                    // Take the right half of cols
                    col_lo += 1 + (col_hi - col_lo) / 2;
                }
                _ => {
                    return Err(BoardingPassError {
                        msg: format!("Incorrectly formatted boarding pass '{}'; must consist only of F, B, L, R", code)
                    });
                }
            }
        }

        if row_lo != row_hi && col_lo != col_hi {
            println!("{} {} {} {}", row_lo, row_hi, col_lo, col_hi);
            return Err(BoardingPassError {
                msg: "Couldn't reduce position to a single row and single col".to_owned(),
            });
        }

        let pos = BoardingPosition {
            row: row_lo,
            col: col_lo,
        };

        if pos.is_valid() {
            Ok(pos)
        } else {
            Err(BoardingPassError {
                msg: format!("Invalid boarding position of row {}, col {}; must be in range row [0, 127] and col [0, 7]", pos.row, pos.col)
            })
        }
    }

    fn new(row: i32, col: i32) -> Result<Self, BoardingPassError> {
        let pos = BoardingPosition { row, col };

        if pos.is_valid() {
            Ok(pos)
        } else {
            Err(BoardingPassError {
                msg: format!("Invalid boarding position of row {}, col {}; must be in range row [0, 127] and col [0, 7]", pos.row, pos.col)
            })
        }
    }
}

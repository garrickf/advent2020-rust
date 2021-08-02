use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let grid = read_input_from_file().expect("Error reading file.");
    part_one(&grid);
    part_two(&grid);
}

// Number of trees on a single slope
fn part_one(grid: &Vec<Vec<bool>>) {
    let num_trees = count_trees(&grid, 3, 1);
    println!("Number of trees encountered: {}", num_trees);
}

// Product of number of trees on multiple slopes
fn part_two(grid: &Vec<Vec<bool>>) {
    let a = count_trees(&grid, 1, 1) as u32; // Type handles overflow
    let b = count_trees(&grid, 3, 1) as u32;
    let c = count_trees(&grid, 5, 1) as u32;
    let d = count_trees(&grid, 7, 1) as u32;
    let e = count_trees(&grid, 1, 2) as u32;

    println!(
        "{} * {} * {} * {} * {} = {}",
        a,
        b,
        c,
        d,
        e,
        a * b * c * d * e
    )
}

// Count the number of trees encountered on the given slope
fn count_trees(grid: &Vec<Vec<bool>>, right: i32, down: i32) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut num_trees = 0;
    let mut curr_col = 0;
    let mut curr_row = 0;
    while curr_row < rows {
        if grid[curr_row as usize][curr_col as usize] {
            num_trees += 1;
        }

        curr_col += right;
        curr_row += down;

        if curr_col >= cols {
            curr_col -= cols;
        }
    }
    num_trees
}

fn read_input_from_file() -> Result<Vec<Vec<bool>>, io::Error> {
    let path = "data/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut grid = Vec::new();
    for line in buffered.lines() {
        let line = line?;
        let bytes = line.trim().as_bytes();

        let mut row = Vec::new();
        for byte in bytes {
            match byte {
                b'#' => {
                    row.push(true);
                }
                b'.' => {
                    row.push(false);
                }
                _ => {
                    panic!("Something wrong with the file...")
                }
            }
        }
        grid.push(row);
    }

    Ok(grid)
}

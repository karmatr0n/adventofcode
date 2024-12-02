// https://adventofcode.com/2024/day/1

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() == 2 {
            let left_number: i32 = columns[0].parse().unwrap_or_else(|_| {
                panic!("Failed to parse '{}' as integer", columns[0]);
            });
            left_numbers.push(left_number);

            let right_number: i32 = columns[1].parse().unwrap_or_else(|_| {
                panic!("Failed to parse '{}' as integer", columns[1]);
            });
            right_numbers.push(right_number);
        } else {
            eprintln!("Line does not contain exactly two columns: {}", line);
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let max: usize = left_numbers.len() - 1;
    let mut i: usize = 0;
    let mut total: i32 = 0;
    while i <= max {
        total += (right_numbers[i] - left_numbers[i]).abs();
        i += 1;
    }
    println!("Total: {}", total);
    Ok(())
}

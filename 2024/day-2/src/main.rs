// https://adventofcode.com/2024/day/2
use std::fs::read_to_string;
use std::iter::Iterator;
use std::path::Path;

pub fn is_safe(levels: &Vec<u32>) -> bool {
    levels
        .windows(2)
        .all(|w| 1 <= w[0].abs_diff(w[1]) && w[0].abs_diff(w[1]) <= 3)
        && (levels.is_sorted() || levels.iter().rev().is_sorted())
}

pub fn safe_reports_part1<P>(path: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(path).unwrap();
    input
        .lines()
        .map(|levels| {
            levels
                .split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(is_safe)
        .count() as u32
}

pub fn safe_reports_part2<P>(path: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(path).unwrap();
    input
        .lines()
        .map(|levels| {
            levels
                .split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|levels| {
            is_safe(levels)
                || (0..levels.len()).any(|skipped| {
                    let new_levels = levels
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, val)| if idx != skipped { Some(*val) } else { None })
                        .collect::<Vec<u32>>();
                    is_safe(&new_levels)
                })
        })
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let path = Path::new("./input.txt");
    let safe_reports1 = safe_reports_part1(path);
    println!("Safe reports1: {}", safe_reports1);
    let safe_reports2 = safe_reports_part2(path);
    println!("Safe reports2: {}", safe_reports2);
    Ok(())
}

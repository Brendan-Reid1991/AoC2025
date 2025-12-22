use itertools::Itertools;

/// This file collects individual parsers for each day.
///
/// Data will always be provided as a text file with each
/// input on a separate line; the parsers will correctly
/// unpack the data for the required logic.
use crate::reader::{read_puzzle_input, PuzzleInput, RawData};

/// The output data type for Day 1.
pub type Day1T = PuzzleInput<i32>;

/// Parse the day 1 file into useable data.
///
/// Reads the ../puzzle_input/day1.txt file and returns a vector of
/// signed integers. These are positive if the char at the beginning
/// is 'L' (turn dial left) and negative if the char at the beginning
/// is 'R' (turn dial right).
///
/// # Returns
///
/// Vec<i32>
///
/// # Panics
///
/// If the puzzle input file could not be read.
pub fn parse_day_1() -> Day1T {
    let to_be_parsed: RawData =
        read_puzzle_input(1).expect("Could not read the day 1 file");
    to_be_parsed
        .iter()
        .map(|line: &String| {
            let direction: char = line.chars().next().unwrap();
            let number: i32 = line[1..].parse::<i32>().unwrap();
            if direction == 'R' {
                -number
            } else {
                number
            }
        })
        .collect()
}

/// The output data type for Day 2.
pub type Day2T = PuzzleInput<(String, String)>;

pub fn parse_day_2() -> Day2T {
    let to_be_parsed: RawData = read_puzzle_input(2).expect("Could not read Day 2 file.");
    let raw_data: &String = to_be_parsed.iter().next().expect("Day 2 file is empty.");
    raw_data
        .split(',')
        .map(|element| {
            element
                .split('-')
                .collect_tuple()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .expect("Expected exactly two elements!")
        })
        .collect()
}

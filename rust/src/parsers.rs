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

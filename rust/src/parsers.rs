use crate::reader::{read_puzzle_input, PuzzleInput, RawData};

pub type Day1T = PuzzleInput<i32>;

pub fn parse_day_1() -> Day1T {
    let to_be_parsed: RawData = read_puzzle_input(1).expect("Could not read the day 1 file");
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

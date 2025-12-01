use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type RawData = Vec<String>;

const PUZZLE_FOLDER: &str = "../puzzle_input/";

fn pathway(day: u8) -> String {
    format!("{PUZZLE_FOLDER}day{day}.txt")
}

pub fn read_puzzle_input(day: u8) -> io::Result<RawData> {
    let path: String = pathway(day);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines().collect()
}

pub type PuzzleInput<T> = Vec<T>;

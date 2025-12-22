/// This file provides a function to read in .txt files from the
/// ../puzzle_input/ folder.
///
/// It is assumed that each file will have each input on a separate line.
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// This is the return type after parsing puzzle text files.
pub type RawData = Vec<String>;

/// Where the txt files are stored.
const PUZZLE_FOLDER: &str = "../puzzle_input/";

/// This function returns a formatted string, giving a relative
/// pathway to a text file.
///
/// # Arguments
///
/// * `day` - Which day of Advent of Code we are on.
///
/// # Returns
///
/// A relative pathway as a formatted string.
fn pathway(day: u8) -> String {
    format!("{PUZZLE_FOLDER}day{day}.txt")
}

/// This function reads in a txt file and returns a vector of
/// the entries.
///
/// # Arguments
///
/// * `day` - Which day of Advent  of Code we are on.
///
/// # Returns
///
/// An io::Result of RawData
pub fn read_puzzle_input(day: u8) -> io::Result<RawData> {
    let path: String = pathway(day);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines().collect()
}

/// A generic type for individual puzzle inputs.
pub type PuzzleInput<T> = Vec<T>;

use std::collections::HashSet;

use aoc2025::parsers::{parse_day_2, Day2T};

fn main() {
    println!("Outcome from part 1...");
    part1();
    println!("\nOutcome from part 2...");
    part2();
}

fn part1() {
    let parsed: Day2T = parse_day_2();
    let mut rangers = parsed
        .iter()
        .filter_map(|ele| {
            let mut ranger = InRange::from(ele.clone());
            if ranger.tighten_range() {
                Some(ranger)
            } else {
                None
            }
        })
        .collect::<Vec<InRange>>();

    let total_invalids = rangers
        .iter_mut()
        .map(|in_range| in_range.find_all_invalids())
        .sum::<u64>();

    println!("{:?}", total_invalids)
}

fn part2() {
    let parsed: Day2T = parse_day_2();
    let mut rangers = parsed
        .iter()
        .map(|ele: &(String, String)| InRange::from(ele.clone()))
        .collect::<Vec<InRange>>();

    let total_invalids = rangers
        .iter_mut()
        .map(|in_range| in_range.find_all_invalids_part_2())
        .sum::<u64>();

    println!("{:?}", total_invalids)
}

#[derive(Debug, Clone)]
struct InRange {
    pub low: u64,
    pub high: u64,
}

impl InRange {
    fn new(low: u64, high: u64) -> Self {
        InRange { low, high }
    }

    fn find_all_invalids(&mut self) -> u64 {
        if !self.tighten_range() {
            return 0;
        }

        let s = self.low.to_string();
        let length = s.len();
        let mut construct = s[..length / 2].parse::<u64>().unwrap();
        let mut total_invalid = 0;

        loop {
            let new_id = Self::repeat_pattern(construct, 2);
            if new_id > self.high {
                break;
            }
            if self.is_in_range(new_id) {
                total_invalid += new_id;
            }
            construct += 1;
        }
        total_invalid
    }

    fn find_all_invalids_part_2(&mut self) -> u64 {
        let low_num_digits = Self::num_digits(self.low);
        let high_num_digits = Self::num_digits(self.high);
        let mut seen = HashSet::new();

        for num_digits in low_num_digits..=high_num_digits {
            let divisors = Self::divisors(num_digits);
            for &div in &divisors {
                let repetitions = num_digits / div;
                let (min_pattern, max_pattern) = Self::pattern_range(div);

                for pattern in min_pattern..=max_pattern {
                    let repeated_id = Self::repeat_pattern(pattern, repetitions);
                    if self.is_in_range(repeated_id) {
                        seen.insert(repeated_id);
                    }
                }
            }
        }
        seen.iter().sum()
    }

    fn repeat_pattern(pattern: u64, repetitions: u8) -> u64 {
        let s = pattern.to_string();
        let repeated = s.repeat(repetitions as usize);
        repeated.parse().unwrap_or(u64::MAX)
    }

    fn is_in_range(&self, id: u64) -> bool {
        id >= self.low && id <= self.high
    }

    fn pattern_range(pattern_length: u8) -> (u64, u64) {
        let min = 10_u64.pow((pattern_length - 1) as u32);
        let max = 10_u64.pow(pattern_length as u32) - 1;
        (min, max)
    }

    fn num_digits(value: u64) -> u8 {
        value.checked_ilog10().map(|n| n as u8 + 1).unwrap_or(1)
    }

    fn divisors(value: u8) -> Vec<u8> {
        (1..=value / 2).filter(|i| value % i == 0).collect()
    }

    fn raise_lower(&mut self) {
        let num_digits = Self::num_digits(self.low);
        if num_digits % 2 == 1 {
            self.low = 10_u64.pow(num_digits as u32)
        }
    }

    fn deflate_higher(&mut self) {
        let num_digits = Self::num_digits(self.high);
        if num_digits % 2 == 1 {
            self.high = 10_u64.pow((num_digits - 1) as u32) - 1
        }
    }

    fn tighten_range(&mut self) -> bool {
        self.raise_lower();
        if self.low > self.high {
            return false;
        }
        self.deflate_higher();
        if self.high < self.low {
            return false;
        }
        true
    }
}
impl From<(String, String)> for InRange {
    fn from(tuple: (String, String)) -> Self {
        InRange::new(
            tuple
                .0
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Can't parse this string! '{}'", tuple.0)),
            tuple
                .1
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Can't parse this string! '{}'", tuple.1)),
        )
    }
}

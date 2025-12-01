use aoc2025::parsers::{parse_day_1, Day1T};

fn main() {
    let rotations: Day1T = parse_day_1();
    let mut position: i32 = 50;
    let mut hits_zero: i32 = 0;
    let mut new_position: i32 = 0;
    rotations.iter().for_each(|value: &i32| {
        let full_rotations: i32 = value.abs() / 100;
        hits_zero += full_rotations;
        new_position = ((position + value) % 100 + 100) % 100;
        if position != 0
            && ((*value > 0 && new_position < position)
                || (*value < 0 && new_position > position)
                || new_position == 0)
        {
            hits_zero += 1;
        }
        position = new_position;
    });
    println!("{}", hits_zero)
}

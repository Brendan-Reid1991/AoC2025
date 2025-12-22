/// The file implements the logic for day 1.
use aoc2025::parsers::{parse_day_1, Day1T};

/// Iterate through the input for day 1.
///
/// For each value, determine how mnay full rotations it
/// encurs and add that to the `hits_zero` integer.
///
/// Then, add on the remaining ticks (i.e. R131 is equivalent
/// to R31, plus one full rotation).
///
/// After determining the final position of the dial, determine if that
/// final movement cause one more crossing of zero, or if the dial
/// landed on zero.
///
/// If the dial was already on zero and then lands on zero, this is caught
/// in the full rotations.
///
/// The result is printed to std::out.
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

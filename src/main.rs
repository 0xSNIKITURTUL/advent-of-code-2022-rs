use std::fs;

mod day_one;

fn main() {
    println!("Advent of Code 2022 Results:");

    // day 01: calorie counting
    let day_one_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_one_result = day_one::find_max_calories(day_one_input);
    println!("day 01: {}", day_one_result);
}

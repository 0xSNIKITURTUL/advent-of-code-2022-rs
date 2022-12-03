use std::fs;

mod day_one;
mod day_two;

fn main() {
    println!("Advent of Code 2022 Results:");

    // day 01: calorie counting part 1
    let day_one_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_one_result = day_one::find_max_calories(&day_one_input);
    println!("day 01 part 1: {}", day_one_result);

    // day 01: calorie counting part 2
    let day_one_result: i32 = day_one::find_top_three_calories(day_one_input);
    println!("day 01 part 2: {}", day_one_result);
}

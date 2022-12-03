use advent_of_code::{solutions::*, solver::solve};
use std::fs;

fn main() {
    println!("Advent of Code 2022 Results:");

    // day 01: calorie counting part 1
    let day_one_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_one_result = solve::<day_01::Day01>(&day_one_input, 1);
    println!("day 01 part 1: {}", day_one_result);

    // day 01: calorie counting part 2
    let day_one_result = solve::<day_01::Day01>(day_one_input, 2);
    println!("day 01 part 2: {}", day_one_result);

    // day 02: rock paper scissors part 1
    let day_two_input = fs::read_to_string("inputs/2.txt").unwrap();
    let day_two_result = solve::<day_02::Day02>(&day_two_input, 1);
    println!("day 02 part 1: {}", day_two_result);

    // day 02: rock paper scissors part 2
    let day_two_result = solve::<day_02::Day02>(day_two_input, 2);
    println!("day 02 part 2: {}", day_two_result);

    // day 03: rucksack reorganization part 1
    let day_three_input = fs::read_to_string("inputs/3.txt").unwrap();
    let day_three_result = solve::<day_03::Day03>(&day_three_input, 1);
    println!("day 03 part 1: {}", day_three_result);

    // day 03: rucksack reorganization part 2
    let day_three_result = solve::<day_03::Day03>(day_three_input, 2);
    println!("day 03 part 2: {}", day_three_result);
}

use advent_of_code::{solutions::*, solver::solve};
use std::fs;

fn main() {
    println!("Advent of Code 2022 Answers:");

    println!("==============");

    // day 01: calorie counting part 1
    let day_one_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_one_result = solve::<i32, day_01::Day01>(&day_one_input, 1);
    println!("day 01 part 1: {}", day_one_result);

    // day 01: calorie counting part 2
    let day_one_result = solve::<i32, day_01::Day01>(day_one_input, 2);
    println!("day 01 part 2: {}", day_one_result);

    println!("==============");

    // day 02: rock paper scissors part 1
    let day_two_input = fs::read_to_string("inputs/2.txt").unwrap();
    let day_two_result = solve::<i32, day_02::Day02>(&day_two_input, 1);
    println!("day 02 part 1: {}", day_two_result);

    // day 02: rock paper scissors part 2
    let day_two_result = solve::<i32, day_02::Day02>(day_two_input, 2);
    println!("day 02 part 2: {}", day_two_result);

    println!("==============");

    // day 03: rucksack reorganization part 1
    let day_three_input = fs::read_to_string("inputs/3.txt").unwrap();
    let day_three_result = solve::<i32, day_03::Day03>(&day_three_input, 1);
    println!("day 03 part 1: {}", day_three_result);

    // day 03: rucksack reorganization part 2
    let day_three_result = solve::<i32, day_03::Day03>(day_three_input, 2);
    println!("day 03 part 2: {}", day_three_result);

    println!("==============");

    // day 04: camp cleanup part 1
    let day_four_input = fs::read_to_string("inputs/4.txt").unwrap();
    let day_four_result = solve::<i32, day_04::Day04>(&day_four_input, 1);
    println!("day 04 part 1: {}", day_four_result);

    // day 04: camp cleanup part 2
    let day_four_result = solve::<i32, day_04::Day04>(day_four_input, 2);
    println!("day 04 part 2: {}", day_four_result);

    println!("==============");

    // day 05: crate reorganization part 1
    let day_five_input = fs::read_to_string("inputs/5.txt").unwrap();
    let day_five_result = solve::<String, day_05::Day05>(&day_five_input, 1);
    println!("day 05 part 1: {}", day_five_result);

    // day 05: crate reorganization part 2
    let day_five_result = solve::<String, day_05::Day05>(day_five_input, 2);
    println!("day 05 part 2: {}", day_five_result);
}

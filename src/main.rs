use advent_of_code::{
    solutions::*,
    solver::{solve, Part},
};
use std::fs;

fn main() {
    println!("Advent of Code 2022 Answers:");

    println!("==============");

    // day 01: calorie counting part 1
    let day_01_input = fs::read_to_string("inputs/1.txt").unwrap();
    let day_01_part_one_result = solve::<i32, day_01::Day01>(&day_01_input, Part::One);
    println!("day 01 part 1: {}", day_01_part_one_result);

    // day 01: calorie counting part 2
    let day_01_part_two_result = solve::<i32, day_01::Day01>(day_01_input, Part::Two);
    println!("day 01 part 2: {}", day_01_part_two_result);

    println!("==============");

    // day 02: rock paper scissors part 1
    let day_02_input = fs::read_to_string("inputs/2.txt").unwrap();
    let day_02_part_one_result = solve::<i32, day_02::Day02>(&day_02_input, Part::One);
    println!("day 02 part 1: {}", day_02_part_one_result);

    // day 02: rock paper scissors part 2
    let day_02_part_two_result = solve::<i32, day_02::Day02>(day_02_input, Part::Two);
    println!("day 02 part 2: {}", day_02_part_two_result);

    println!("==============");

    // day 03: rucksack reorganization part 1
    let day_03_input = fs::read_to_string("inputs/3.txt").unwrap();
    let day_03_part_one_result = solve::<i32, day_03::Day03>(&day_03_input, Part::One);
    println!("day 03 part 1: {}", day_03_part_one_result);

    // day 03: rucksack reorganization part 2
    let day_03_part_two_result = solve::<i32, day_03::Day03>(day_03_input, Part::Two);
    println!("day 03 part 2: {}", day_03_part_two_result);

    println!("==============");

    // day 04: camp cleanup part 1
    let day_04_input = fs::read_to_string("inputs/4.txt").unwrap();
    let day_04_part_one_result = solve::<i32, day_04::Day04>(&day_04_input, Part::One);
    println!("day 04 part 1: {}", day_04_part_one_result);

    // day 04: camp cleanup part 2
    let day_04_part_two_result = solve::<i32, day_04::Day04>(day_04_input, Part::Two);
    println!("day 04 part 2: {}", day_04_part_two_result);

    println!("==============");

    // day 05: supply stacks part 1
    let day_05_input = fs::read_to_string("inputs/5.txt").unwrap();
    let day_05_part_one_result = solve::<String, day_05::Day05>(&day_05_input, Part::One);
    println!("day 05 part 1: {}", day_05_part_one_result);

    // day 05: supply stacks part 2
    let day_05_part_two_result = solve::<String, day_05::Day05>(day_05_input, Part::Two);
    println!("day 05 part 2: {}", day_05_part_two_result);

    println!("==============");

    // day 06: tuning trouble part 1
    let day_06_input = fs::read_to_string("inputs/6.txt").unwrap();
    let day_06_part_one_result = solve::<usize, day_06::Day06>(&day_06_input, Part::One);
    println!("day 06 part 1: {}", day_06_part_one_result);

    // day 06: tuning trouble part 2
    let day_06_part_two_result = solve::<usize, day_06::Day06>(day_06_input, Part::Two);
    println!("day 06 part 2: {}", day_06_part_two_result);

    println!("==============");

    // day 07: no space left on device part 1
    let day_07_input = fs::read_to_string("inputs/7.txt").unwrap();
    let day_07_part_one_result = solve::<usize, day_07::Day07>(&day_07_input, Part::One);
    println!("day 07 part 1: {}", day_07_part_one_result);

    // day 07: no space left on device part 2
    let day_07_part_two_result = solve::<usize, day_07::Day07>(day_07_input, Part::Two);
    println!("day 07 part 2: {}", day_07_part_two_result);

    println!("==============");

    // day 08: treetop tree house part 1
    let day_08_input = fs::read_to_string("inputs/8.txt").unwrap();
    let day_08_part_one_result = solve::<usize, day_08::Day08>(&day_08_input, Part::One);
    println!("day 08 part 1: {}", day_08_part_one_result);

    // day 08: treetop tree house part 2
    let day_08_part_two_result = solve::<usize, day_08::Day08>(day_08_input, Part::Two);
    println!("day 08 part 2: {}", day_08_part_two_result);
}

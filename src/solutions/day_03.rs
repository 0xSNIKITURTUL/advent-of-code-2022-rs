use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day03;

impl Solver<i32> for Day03 {
    fn part_one(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .map(|rucksack| {
                let (comp_1, comp_2) = rucksack.split_at(rucksack.len() / 2);
                let compartment_1 = comp_1.chars().collect::<HashSet<char>>();
                let compartment_2 = comp_2.chars().collect::<HashSet<char>>();
                let character = compartment_1.intersection(&compartment_2).next().unwrap();
                if character.is_ascii_lowercase() {
                    return (*character as i32) - 96;
                }
                (*character as i32) - 38
            })
            .sum::<i32>() as i32
    }

    fn part_two(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|group| {
                if group.len() < 3 {
                    return 0;
                }
                let set1 = group[0].chars().collect::<HashSet<char>>();
                let set2 = group[1].chars().collect::<HashSet<char>>();
                let set3 = group[2].chars().collect::<HashSet<char>>();
                let intersection = set1.intersection(&set2).cloned().collect::<HashSet<char>>();
                let character = intersection.intersection(&set3).next().unwrap();
                if character.is_ascii_lowercase() {
                    return (*character as i32) - 96;
                }
                (*character as i32) - 38
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day03;

    static INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_day_03_part_one() {
        test_solution::<i32, Day03>(INPUT, 157, Part::One);
    }

    #[test]
    fn test_day_03_part_two() {
        test_solution::<i32, Day03>(INPUT, 70, Part::Two);
    }
}

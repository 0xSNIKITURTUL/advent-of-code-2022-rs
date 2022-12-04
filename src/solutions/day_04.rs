use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day04;

fn find_common_elements(set_01: HashSet<i32>, set_02: HashSet<i32>) -> Vec<i32> {
    set_01.intersection(&set_02).cloned().collect::<Vec<i32>>()
}

fn split_pairs(input: &str) -> (HashSet<i32>, HashSet<i32>) {
    let split = input.split(',').collect::<Vec<&str>>();
    let set_1 = split[0].split('-').collect::<Vec<&str>>();
    let set_2 = split[1].split('-').collect::<Vec<&str>>();
    let set_1 = (set_1[0].parse::<i32>().unwrap()..=set_1[1].parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();
    let set_2 = (set_2[0].parse::<i32>().unwrap()..=set_2[1].parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();
    (set_1, set_2)
}

#[allow(unused_variables, unused_must_use)]
impl Solver for Day04 {
    fn part_one(input: impl Into<String>) -> i32 {
        let input: String = input.into();
        input
            .lines()
            .map(|pair| {
                let (elf_1_range, elf_2_range) = split_pairs(pair);
                let common = find_common_elements(elf_1_range.clone(), elf_2_range.clone());
                if common.len() != elf_1_range.len() && common.len() != elf_2_range.len() {
                    return 0;
                }
                1
            })
            .sum::<i32>()
    }

    fn part_two(input: impl Into<String>) -> i32 {
        let input: String = input.into();
        input
            .lines()
            .map(|pair| {
                let (elf_1_range, elf_2_range) = split_pairs(pair);
                let common = find_common_elements(elf_1_range, elf_2_range);
                if common.len() == 0 {
                    return 0;
                }
                1
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use crate::solver::test_solution;

    use super::Day04;

    static INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day_04() {
        test_solution::<Day04>(INPUT, 2, 4);
    }
}

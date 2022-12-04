use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day04;

#[allow(unused_variables, unused_must_use)]
impl Solver for Day04 {
    fn part_one(input: impl Into<String>) -> i32 {
        let input: String = input.into();
        input
            .lines()
            .map(|pair| {
                let mut split = pair.split(',');
                let elf_1 = split.nth(0).unwrap();
                let elf_2 = split.nth(0).unwrap();

                let mut elf_1_range = elf_1.split('-');
                let elf_1_start = elf_1_range.nth(0).unwrap().parse::<i32>().unwrap();
                let elf_1_end = elf_1_range.nth(0).unwrap().parse::<i32>().unwrap();

                let mut elf_2_range = elf_2.split('-');
                let elf_2_start = elf_2_range.nth(0).unwrap().parse::<i32>().unwrap();
                let elf_2_end = elf_2_range.nth(0).unwrap().parse::<i32>().unwrap();

                let elf_1_range = (elf_1_start..=elf_1_end).collect::<HashSet<i32>>();
                let elf_2_range = (elf_2_start..=elf_2_end).collect::<HashSet<i32>>();

                let common = elf_1_range
                    .intersection(&elf_2_range)
                    .cloned()
                    .collect::<HashSet<i32>>();

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
                let mut split = pair.split(',');
                let elf_1 = split.nth(0).unwrap();
                let elf_2 = split.nth(0).unwrap();

                let mut elf_1_range = elf_1.split('-');
                let elf_1_start = elf_1_range.nth(0).unwrap().parse::<i32>().unwrap();
                let elf_1_end = elf_1_range.nth(0).unwrap().parse::<i32>().unwrap();

                let mut elf_2_range = elf_2.split('-');
                let elf_2_start = elf_2_range.nth(0).unwrap().parse::<i32>().unwrap();
                let elf_2_end = elf_2_range.nth(0).unwrap().parse::<i32>().unwrap();

                let elf_1_range = (elf_1_start..=elf_1_end).collect::<HashSet<i32>>();
                let elf_2_range = (elf_2_start..=elf_2_end).collect::<HashSet<i32>>();

                let common = elf_1_range
                    .intersection(&elf_2_range)
                    .cloned()
                    .collect::<HashSet<i32>>();
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

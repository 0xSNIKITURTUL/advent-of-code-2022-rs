use crate::solver::Solver;

pub struct Day01;

impl Solver for Day01 {
    fn part_one(input: impl Into<String>) -> i32 {
        input
            .into()
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|line| line.parse::<i32>().map(|value| value).ok())
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    }

    fn part_two(input: impl Into<String>) -> i32 {
        let mut result = input
            .into()
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|line| line.parse::<i32>().map(|value| value).ok())
                    .sum::<i32>()
            })
            .collect::<Vec<i32>>();
        result.sort();
        result.iter().rev().take(3).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day01;

    static INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day_01_part_one() {
        test_solution::<Day01>(INPUT, 24000, Part::One);
    }

    #[test]
    fn test_day_01_part_two() {
        test_solution::<Day01>(INPUT, 45000, Part::Two);
    }
}

use crate::solver::Solver;

pub struct Day02;

impl Solver<i32> for Day02 {
    fn part_one(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .map(|scores| {
                let mut choices = scores.split(' ');
                match (choices.nth(0), choices.nth(0)) {
                    (Some("A"), Some("X")) => 4,
                    (Some("B"), Some("X")) => 1,
                    (Some("C"), Some("X")) => 7,
                    (Some("A"), Some("Y")) => 8,
                    (Some("B"), Some("Y")) => 5,
                    (Some("C"), Some("Y")) => 2,
                    (Some("A"), Some("Z")) => 3,
                    (Some("B"), Some("Z")) => 9,
                    (Some("C"), Some("Z")) => 6,
                    _ => panic!("invalid inputs"),
                }
            })
            .sum::<i32>()
    }

    fn part_two(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .map(|scores| {
                let mut choices = scores.split(' ');
                match (choices.nth(0), choices.nth(0)) {
                    (Some("A"), Some("X")) => 3,
                    (Some("B"), Some("X")) => 1,
                    (Some("C"), Some("X")) => 2,
                    (Some("A"), Some("Y")) => 4,
                    (Some("B"), Some("Y")) => 5,
                    (Some("C"), Some("Y")) => 6,
                    (Some("A"), Some("Z")) => 8,
                    (Some("B"), Some("Z")) => 9,
                    (Some("C"), Some("Z")) => 7,
                    _ => panic!("invalid inputs"),
                }
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day02;

    static INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_day_02_part_one() {
        test_solution::<i32, Day02>(INPUT, 15, Part::One);
    }

    #[test]
    fn test_day_02_part_two() {
        test_solution::<i32, Day02>(INPUT, 12, Part::Two);
    }
}

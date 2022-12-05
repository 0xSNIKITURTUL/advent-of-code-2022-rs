use crate::solver::Solver;

pub struct Day02;

impl Solver<i32> for Day02 {
    fn part_one(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .map(|scores| {
                let mut choices = scores.split(' ');
                let opp_choice = choices.nth(0).unwrap();
                let your_choice = choices.nth(0).unwrap();

                return if your_choice == "X" {
                    if opp_choice == "A" {
                        3 + 1
                    } else if opp_choice == "B" {
                        0 + 1
                    } else {
                        6 + 1
                    }
                } else if your_choice == "Y" {
                    if opp_choice == "A" {
                        6 + 2
                    } else if opp_choice == "B" {
                        3 + 2
                    } else {
                        0 + 2
                    }
                } else {
                    if opp_choice == "A" {
                        0 + 3
                    } else if opp_choice == "B" {
                        6 + 3
                    } else {
                        3 + 3
                    }
                };
            })
            .sum::<i32>()
    }

    fn part_two(input: impl Into<String>) -> i32 {
        input
            .into()
            .lines()
            .map(|scores| {
                let mut choices = scores.split(' ');
                let opp_choice = choices.nth(0).unwrap();
                let strategy = choices.nth(0).unwrap();

                return if strategy == "X" {
                    if opp_choice == "A" {
                        0 + 3
                    } else if opp_choice == "B" {
                        0 + 1
                    } else {
                        0 + 2
                    }
                } else if strategy == "Y" {
                    if opp_choice == "A" {
                        3 + 1
                    } else if opp_choice == "B" {
                        3 + 2
                    } else {
                        3 + 3
                    }
                } else {
                    if opp_choice == "A" {
                        6 + 2
                    } else if opp_choice == "B" {
                        6 + 3
                    } else {
                        6 + 1
                    }
                };
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

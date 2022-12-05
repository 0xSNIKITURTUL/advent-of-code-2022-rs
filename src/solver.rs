use std::fmt::Debug;

pub trait Solver<T: PartialEq + Eq> {
    fn part_one(input: impl Into<String>) -> T;
    fn part_two(input: impl Into<String>) -> T;
}

pub fn solve<T: PartialEq + Eq, S: Solver<T>>(input: impl Into<String>, part: u8) -> T {
    if part == 1 {
        return S::part_one(input);
    }
    S::part_two(input)
}

pub enum Part {
    One,
    Two,
}

pub fn test_solution<T: Eq + PartialEq + Debug, S: Solver<T>>(
    input: impl Into<String> + Clone + Copy,
    expected_solution: T,
    part: Part,
) {
    match part {
        Part::One => {
            assert_eq!(S::part_one(input), expected_solution);
        }
        Part::Two => {
            assert_eq!(S::part_two(input), expected_solution);
        }
    }
}

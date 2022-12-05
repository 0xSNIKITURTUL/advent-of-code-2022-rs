pub trait Solver {
    fn part_one(input: impl Into<String>) -> i32;
    fn part_two(input: impl Into<String>) -> i32;
}

pub fn solve<S: Solver>(input: impl Into<String>, part: u8) -> i32 {
    if part == 1 {
        return S::part_one(input);
    }
    S::part_two(input)
}

pub enum Part {
    One,
    Two,
}

pub fn test_solution<S: Solver>(
    input: impl Into<String> + Clone + Copy,
    expected_solution: i32,
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

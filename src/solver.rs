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

pub fn test_solution<S: Solver>(
    input: impl Into<String> + Clone + Copy,
    part_one_solution: i32,
    part_two_solution: i32,
) {
    assert_eq!(S::part_one(input), part_one_solution);
    assert_eq!(S::part_two(input), part_two_solution);
}

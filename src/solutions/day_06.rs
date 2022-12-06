use std::{collections::HashSet, hash::Hash};

use crate::solver::Solver;

pub struct Day06;

fn fill_chunk<I: Iterator<Item = impl PartialEq + Eq + Copy + Clone>>(
    iter: &mut I,
    size: usize,
) -> Vec<I::Item> {
    (0..size).map(|_| iter.nth(0).unwrap()).collect()
}

fn no_duplicates<T: IntoIterator<Item = impl PartialEq + Eq + Hash> + Clone + Copy>(
    iter: T,
) -> bool {
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

fn cycle<T: Clone + Copy>(chunk: &mut Vec<T>, val: T) {
    (0..chunk.len()).for_each(|curr| {
        chunk[curr] = if curr == chunk.len() - 1 {
            val
        } else {
            chunk[curr + 1]
        };
    })
}

impl Solver<usize> for Day06 {
    fn part_one(input: impl Into<String>) -> usize {
        const SIZE: usize = 4;
        let input: String = input.into();
        let mut characters = input.chars();
        let mut cycling_chunk = fill_chunk(&mut characters, SIZE);
        return if no_duplicates(&cycling_chunk) {
            SIZE
        } else {
            for (x, ch) in characters.enumerate() {
                cycle(&mut cycling_chunk, ch);
                if no_duplicates(&cycling_chunk) {
                    return x + SIZE + 1;
                }
            }
            0
        };
    }

    fn part_two(input: impl Into<String>) -> usize {
        const SIZE: usize = 14;
        let input: String = input.into();
        let mut characters = input.chars();
        let mut cycling_chunk = fill_chunk(&mut characters, SIZE);
        return if no_duplicates(&cycling_chunk) {
            SIZE
        } else {
            for (x, ch) in characters.enumerate() {
                cycle(&mut cycling_chunk, ch);
                if no_duplicates(&cycling_chunk) {
                    return x + SIZE + 1;
                }
            }
            0
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day06;

    static INPUTS: [&'static str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    static PART_ONE_SOLUTIONS: [usize; 5] = [7, 5, 6, 10, 11];

    static PART_TWO_SOLUTIONS: [usize; 5] = [19, 23, 23, 29, 26];

    #[test]
    fn test_day_06_part_one() {
        for (x, input) in INPUTS.iter().enumerate() {
            test_solution::<usize, Day06>(*input, PART_ONE_SOLUTIONS[x], Part::One);
        }
    }

    #[test]
    fn test_day_06_part_two() {
        for (x, input) in INPUTS.iter().enumerate() {
            test_solution::<usize, Day06>(*input, PART_TWO_SOLUTIONS[x], Part::Two);
        }
    }
}

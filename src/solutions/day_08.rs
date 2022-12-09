use crate::solver::Solver;

pub struct Day08;

fn build_grid(input: impl Into<String>) -> Vec<Vec<char>> {
    input
        .into()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

impl Solver<usize> for Day08 {
    fn part_one(input: impl Into<String>) -> usize {
        let grid = build_grid(input);
        grid.iter()
            .enumerate()
            .map(|(x, row)| {
                if x == 0 || x == grid.len() - 1 {
                    row.len()
                } else {
                    row.iter()
                        .enumerate()
                        .map(|(y, tree)| {
                            if y == 0
                                || y == grid[x].len() - 1
                                || row[0..y].iter().all(|z| z < tree)
                                || row[y + 1..row.len()].iter().all(|z| z < tree)
                                || grid[0..x].iter().all(|i| i[y] < *tree)
                                || grid[x + 1..grid.len()].iter().all(|i| i[y] < *tree)
                            {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<usize>()
                }
            })
            .sum()
    }

    fn part_two(input: impl Into<String>) -> usize {
        let grid = build_grid(input);
        grid.iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, tree)| {
                        let up_score = (0..x)
                            .rev()
                            .find_map(|i| {
                                if grid[i][y] >= *tree {
                                    Some(x - i)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(x);
                        let down_score = (x + 1..grid.len())
                            .find_map(|i| {
                                if grid[i][y] >= *tree {
                                    Some(i - x)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(grid.len() - x - 1);
                        let left_score = (0..y)
                            .rev()
                            .find_map(|i| if row[i] >= *tree { Some(y - i) } else { None })
                            .unwrap_or(y);
                        let right_score = (y + 1..row.len())
                            .find_map(|i| if row[i] >= *tree { Some(i - y) } else { None })
                            .unwrap_or(row.len() - y - 1);
                        up_score * down_score * left_score * right_score
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day08;

    static INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_day_08_part_one() {
        test_solution::<usize, Day08>(INPUT, 21, Part::One);
    }

    #[test]
    fn test_day_08_part_two() {
        test_solution::<usize, Day08>(INPUT, 8, Part::Two);
    }
}

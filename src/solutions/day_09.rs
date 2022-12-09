use std::collections::HashSet;

use crate::solver::Solver;

pub struct Day09;

fn is_touching(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1
}

fn get_displacement(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let (x_dist, y_dist) = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
    (
        x_dist / x_dist.abs().clamp(1, i32::MAX),
        y_dist / y_dist.abs().clamp(1, i32::MAX),
    )
}

impl Solver<usize> for Day09 {
    fn part_one(input: impl Into<String>) -> usize {
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        let mut places_visited = HashSet::from([(0, 0)]);
        input.into().lines().for_each(|line| {
            let (cmd, spaces) = line.split_at(1);
            let spaces = spaces.trim().parse::<i32>().unwrap();
            (0..spaces).for_each(|_| {
                match cmd {
                    "U" => head_pos.1 += 1,
                    "D" => head_pos.1 -= 1,
                    "R" => head_pos.0 += 1,
                    "L" => head_pos.0 -= 1,
                    _ => panic!("invalid command"),
                };
                if !is_touching(head_pos, tail_pos) {
                    let (x_disp, y_disp) = get_displacement(head_pos, tail_pos);
                    tail_pos.0 += x_disp;
                    tail_pos.1 += y_disp;
                }
                places_visited.insert(tail_pos);
            });
        });
        places_visited.len()
    }

    fn part_two(input: impl Into<String>) -> usize {
        let mut head_pos = (0, 0);
        let mut knot_poss = [(0, 0); 9];
        let mut places_visited = HashSet::from([(0, 0)]);
        input.into().lines().for_each(|line| {
            let (cmd, spaces) = line.split_at(1);
            let spaces = spaces.trim().parse::<i32>().unwrap();
            (0..spaces).for_each(|_| {
                match cmd {
                    "U" => head_pos.1 += 1,
                    "D" => head_pos.1 -= 1,
                    "R" => head_pos.0 += 1,
                    "L" => head_pos.0 -= 1,
                    _ => panic!("invalid command"),
                };
                let mut last_knot = (0, 0);
                knot_poss.iter_mut().enumerate().for_each(|(x, knot_pos)| {
                    let pred_pos = if x == 0 { head_pos } else { last_knot };
                    if !is_touching(pred_pos, *knot_pos) {
                        let (x_disp, y_disp) = get_displacement(pred_pos, *knot_pos);
                        knot_pos.0 += x_disp;
                        knot_pos.1 += y_disp;
                    }
                    if x == 8 {
                        places_visited.insert(*knot_pos);
                    } else {
                        last_knot = *knot_pos;
                    }
                });
            });
        });
        places_visited.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day09;

    static INPUT_1: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static INPUT_2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_day_08_part_one() {
        test_solution::<usize, Day09>(INPUT_1, 13, Part::One);
    }

    #[test]
    fn test_day_08_part_two() {
        test_solution::<usize, Day09>(INPUT_2, 36, Part::Two);
    }
}

use crate::solver::Solver;

pub struct Day10;

impl Solver<String> for Day10 {
    fn part_one(input: impl Into<String>) -> String {
        let mut register = 1;
        let mut cycle = 1;
        let mut special_signals = vec![];
        input.into().lines().for_each(|line| {
            let (cmd, val) = line.split_at(4);
            let val = if val.is_empty() {
                0
            } else {
                val.trim().parse::<i32>().unwrap()
            };
            cycle += 1;
            if cycle <= 220 && cycle % 40 == 20 {
                special_signals.push(cycle * register);
            }
            if cmd == "addx" {
                cycle += 1;
                register += val;
                if cycle <= 220 && cycle % 40 == 20 {
                    special_signals.push(cycle * register);
                }
            }
        });
        special_signals.iter().sum::<i32>().to_string()
    }

    fn part_two(input: impl Into<String>) -> String {
        let mut monitor = [["."; 40]; 6];
        let mut sprite = [0i32, 1i32, 2i32];
        let mut cycle = 1usize;
        let mut monitor_line = 0;
        input.into().lines().for_each(|line| {
            let (cmd, val) = line.split_at(4);
            let val = if val.is_empty() {
                0
            } else {
                val.trim().parse::<i32>().unwrap()
            };
            if sprite.iter().any(|pixel| *pixel == cycle as i32 - 1) {
                monitor[monitor_line][cycle - 1] = "#";
            }
            cycle += 1;
            if cycle % 40 == 0 {
                monitor_line += 1;
                cycle = 0;
            }
            if cmd == "addx" {
                if sprite.iter().any(|pixel| *pixel == cycle as i32 - 1) {
                    monitor[monitor_line][cycle - 1] = "#";
                }
                cycle += 1;
                sprite.iter_mut().for_each(|pixel| *pixel += val);

                if cycle % 40 == 0 {
                    monitor_line += 1;
                    cycle = 0;
                }
            }
        });
        monitor.map(|line| line.concat()).join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day10;

    static INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_day_10_part_one() {
        test_solution::<String, Day10>(INPUT, "13140".into(), Part::One);
    }

    static RESULT: &'static str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....";

    #[test]
    fn test_day_10_part_two() {
        test_solution::<String, Day10>(INPUT, RESULT.into(), Part::Two);
    }
}

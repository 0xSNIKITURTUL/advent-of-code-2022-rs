use crate::solver::Solver;

fn read_diagram(diagram: &str) -> Vec<Vec<char>> {
    let matrix = diagram
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| chunk[1])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    (0..matrix[0].len())
        .map(|index| matrix.iter().map(|inner| inner[index]).collect())
        .collect::<Vec<Vec<char>>>()
        .iter()
        .map(|row| {
            let mut borrowed = row.clone();
            borrowed.reverse();
            borrowed[1..]
                .iter()
                .filter(|x| **x != ' ')
                .cloned()
                .collect()
        })
        .collect()
}

pub struct Day05;

#[allow(unused, unused_must_use)]
impl Solver<String> for Day05 {
    fn part_one(input: impl Into<String>) -> String {
        let input: String = input.into();
        let mut parts = input.split("\n\n");
        let mut diagram = read_diagram(parts.nth(0).unwrap());
        let instructions = parts.nth(0).unwrap();

        instructions.lines().for_each(|instruction| {
            let instruction = instruction.split(' ').collect::<Vec<&str>>();
            let moved = instruction[1].parse::<usize>().unwrap();
            let from = instruction[3].parse::<usize>().unwrap() - 1;
            let to = instruction[5].parse::<usize>().unwrap() - 1;
            (0..moved).for_each(|_| {
                let crane = diagram[from].pop().unwrap();
                diagram[to].push(crane);
            });
        });

        let solution = diagram
            .iter()
            .map(|stack| stack[stack.len() - 1] as u8)
            .collect::<Vec<u8>>();

        String::from_utf8(solution).unwrap()
    }

    fn part_two(input: impl Into<String>) -> String {
        let input: String = input.into();
        let mut parts = input.split("\n\n");
        let mut diagram = read_diagram(parts.nth(0).unwrap());
        let instructions = parts.nth(0).unwrap();

        instructions.lines().for_each(|instruction| {
            let instruction = instruction.split(' ').collect::<Vec<&str>>();
            let moved = instruction[1].parse::<usize>().unwrap();
            let from = instruction[3].parse::<usize>().unwrap() - 1;
            let to = instruction[5].parse::<usize>().unwrap() - 1;
            let mut crane = (0..moved)
                .map(|_| diagram[from].pop().unwrap())
                .collect::<Vec<char>>();
            crane.reverse();
            diagram[to].append(&mut crane);
        });

        let sol = diagram
            .iter()
            .map(|stack| stack[stack.len() - 1] as u8)
            .collect::<Vec<u8>>();

        String::from_utf8(sol).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day05;

    static INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day_05_part_one() {
        test_solution::<String, Day05>(INPUT, "CMZ".into(), Part::One);
    }

    #[test]
    fn test_day_05_part_two() {
        test_solution::<String, Day05>(INPUT, "MCD".into(), Part::Two);
    }
}

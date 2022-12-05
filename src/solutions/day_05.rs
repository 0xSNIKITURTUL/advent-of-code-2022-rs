use crate::solver::Solver;

fn rotate90(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if v.is_empty() {
        return v;
    }

    // transpose code adapted from @Netwave on stackoverflow: https://stackoverflow.com/a/64499219/19020697
    let transposed = (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut diagram = vec![];
    for mut stack in transposed {
        stack.reverse();
        let filtered = stack
            .iter()
            .filter(|x| **x != ' ')
            .cloned()
            .collect::<Vec<char>>();
        diagram.push(filtered);
    }
    diagram
}

fn build_vector2d_from_diagram(diagram: &str) -> Vec<Vec<char>> {
    let mut vector2d: Vec<Vec<char>> = vec![];

    diagram.lines().for_each(|line| {
        let characters = line.chars().collect::<Vec<char>>();
        let mut row = vec![];
        for chunk in characters.chunks(4) {
            row.push(chunk[1]);
        }
        vector2d.push(row);
    });

    vector2d
}

pub struct Day05;

#[allow(unused, unused_must_use)]
impl Solver<String> for Day05 {
    fn part_one(input: impl Into<String>) -> String {
        let input: String = input.into();
        let mut parts = input.split("\n\n");
        let diagram = build_vector2d_from_diagram(parts.nth(0).unwrap());
        let mut diagram = rotate90(diagram);
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
        let diagram = build_vector2d_from_diagram(parts.nth(0).unwrap());
        let mut diagram = rotate90(diagram);
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

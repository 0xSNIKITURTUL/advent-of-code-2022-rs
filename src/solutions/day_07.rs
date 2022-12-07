use crate::solver::Solver;

pub struct Day07;

fn collect_directory_sizes(input: impl Into<String>) -> Vec<usize> {
    let mut tracker = vec![];
    let mut exited = vec![];
    input.into().lines().for_each(|line| {
        if line.starts_with("$ cd") {
            match line.contains("..") {
                true => exited.push(tracker.pop().unwrap()),
                false => tracker.push(0),
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            tracker
                .iter_mut()
                .for_each(|x| *x += line.split(' ').next().unwrap().parse::<usize>().unwrap());
        }
    });
    [tracker, exited].concat()
}

impl Solver<usize> for Day07 {
    fn part_one(input: impl Into<String>) -> usize {
        collect_directory_sizes(input)
            .iter()
            .filter_map(|x| if *x > 100_000 { None } else { Some(x) })
            .sum()
    }

    fn part_two(input: impl Into<String>) -> usize {
        let sizes = collect_directory_sizes(input);
        let required = 30000000 - (70000000 - sizes.iter().max().unwrap());
        sizes
            .iter()
            .filter_map(|x| if *x < required { None } else { Some(*x) })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{test_solution, Part};

    use super::Day07;

    static INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_day_07_part_one() {
        test_solution::<usize, Day07>(INPUT, 95437, Part::One);
    }

    #[test]
    fn test_day_07_part_two() {
        test_solution::<usize, Day07>(INPUT, 24933642, Part::Two);
    }
}

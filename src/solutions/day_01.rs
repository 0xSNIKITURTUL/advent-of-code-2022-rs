use crate::solver::Solver;

pub struct Day01;

impl Solver for Day01 {
    fn part_one(input: impl Into<String>) -> i32 {
        input
            .into()
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|item| match item.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => 0,
                    })
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    }

    fn part_two(input: impl Into<String>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        input
            .into()
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|item| match item.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => 0,
                    })
                    .sum::<i32>()
            })
            .for_each(|calories| {
                if calories > first {
                    third = second;
                    second = first;
                    first = calories;
                } else if calories > second {
                    third = second;
                    second = calories;
                } else if calories > third {
                    third = calories;
                }
            });
        first + second + third
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::test_solution;

    use super::Day01;

    static INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day_01() {
        test_solution::<Day01>(INPUT, 24000, 45000);
    }
}
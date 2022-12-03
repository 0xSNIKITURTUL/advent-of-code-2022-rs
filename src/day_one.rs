pub fn find_max_calories(input: impl Into<String>) -> i32 {
    input
        .into()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| match item.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => 0,
                })
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

pub fn find_top_three_calories(input: impl Into<String>) -> i32 {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    input
        .into()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
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

#[cfg(test)]
mod tests {
    use crate::day_one::{find_max_calories, find_top_three_calories};

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
    fn day_one_part_one() {
        assert!(find_max_calories(INPUT) == 24000);
    }

    #[test]
    fn day_one_part_two() {
        assert!(find_top_three_calories(INPUT) == 45000);
    }
}

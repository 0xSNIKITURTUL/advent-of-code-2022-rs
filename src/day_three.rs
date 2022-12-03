use std::collections::HashSet;

pub fn find_sum_of_priorities_of_common_items(input: impl Into<String>) -> i32 {
    input
        .into()
        .lines()
        .map(|rucksack| {
            if rucksack == "" {
                return 0;
            }
            let (comp_1, comp_2) = rucksack.split_at(rucksack.len() / 2);
            let compartment_1 = comp_1.chars().collect::<HashSet<char>>();
            let compartment_2 = comp_2.chars().collect::<HashSet<char>>();
            let character = compartment_1.intersection(&compartment_2).next().unwrap();
            if character.is_ascii_lowercase() {
                return (*character as i32) - 96;
            }
            (*character as i32) - 38
        })
        .sum::<i32>() as i32
}

pub fn find_sum_of_priorities(input: impl Into<String>) -> i32 {
    input
        .into()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            if group.len() < 3 {
                return 0;
            }
            let set1 = group[0].chars().collect::<HashSet<char>>();
            let set2 = group[1].chars().collect::<HashSet<char>>();
            let set3 = group[2].chars().collect::<HashSet<char>>();
            let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
            let character = intersection.intersection(&set3).next().unwrap();
            if character.is_ascii_lowercase() {
                return (*character as i32) - 96;
            }
            (*character as i32) - 38
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::{find_sum_of_priorities, find_sum_of_priorities_of_common_items};

    static INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day_three_part_one() {
        assert!(find_sum_of_priorities_of_common_items(INPUT) == 157);
    }

    #[test]
    fn day_three_part_two() {
        assert!(find_sum_of_priorities(INPUT) == 70);
    }
}

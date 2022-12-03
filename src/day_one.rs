pub fn find_max_calories(input: String) -> i32 {
    let mut max_calories = 0;
    input.split("\n\n").for_each(|elf| {
        let mut calories = 0;
        elf.split('\n').for_each(|item| {
            calories += match item.parse::<i32>() {
                Ok(num) => num,
                Err(_) => 0,
            };
        });
        if calories > max_calories {
            max_calories = calories;
        }
    });
    max_calories
}

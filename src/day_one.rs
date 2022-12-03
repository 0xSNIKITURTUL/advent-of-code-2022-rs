pub fn find_max_calories(input: String) -> i32 {
    input
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

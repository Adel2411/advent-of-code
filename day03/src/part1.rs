pub fn solve(input: &str) {
    let mut total_joltage: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let mut max_joltage = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        total_joltage += max_joltage;
    }

    println!("Part 1 Total Output Joltage: {}", total_joltage);
}

pub fn solve(input: &str) {
    let mut total_joltage: u64 = 0;
    let k = 12;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() < k {
            continue; // Should not happen based on problem description
        }

        let mut stack: Vec<u32> = Vec::new();
        let mut rem = digits.len() - k;

        for &digit in &digits {
            while rem > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                rem -= 1;
            }
            stack.push(digit);
        }

        stack.truncate(k);

        let mut joltage: u64 = 0;
        for &d in &stack {
            joltage = joltage * 10 + d as u64;
        }

        total_joltage += joltage;
    }

    println!("Part 2 Total Output Joltage: {}", total_joltage);
}

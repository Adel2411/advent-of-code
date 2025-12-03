pub fn solve(input: &str) {
    let mut invalid_sum: u64 = 0;

    let ranges: Vec<&str> = input.trim().split(',').collect();

    for range_str in ranges {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().expect("Invalid start number");
        let end: u64 = parts[1].parse().expect("Invalid end number");

        for num in start..=end {
            if is_invalid_id(num) {
                invalid_sum += num;
            }
        }
    }

    println!("Part 1 Sum of Invalid IDs: {}", invalid_sum);
}

fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[0..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

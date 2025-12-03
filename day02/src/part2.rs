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

    println!("Part 2 Sum of Invalid IDs: {}", invalid_sum);
}

fn is_invalid_id(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();

    for sub_len in 1..=len / 2 {
        if len % sub_len == 0 {
            let sub = &s[0..sub_len];
            let repetitions = len / sub_len;
            
            let expected = sub.repeat(repetitions);
            
            if s == expected {
                return true;
            }
        }
    }
    
    false
}

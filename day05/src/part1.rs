pub fn solve(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        panic!("Invalid input format");
    }

    let range_lines = parts[0];
    let id_lines = parts[1];

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in range_lines.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let bounds: Vec<&str> = line.split('-').collect();
        let start: u64 = bounds[0].parse().expect("Invalid range start");
        let end: u64 = bounds[1].parse().expect("Invalid range end");
        ranges.push((start, end));
    }

    let mut fresh_count = 0;
    for line in id_lines.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let id: u64 = line.parse().expect("Invalid ID");

        let mut is_fresh = false;
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}

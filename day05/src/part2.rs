pub fn solve(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 1 {
        panic!("Invalid input format");
    }

    let range_lines = parts[0];

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

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    if ranges.is_empty() {
        println!("Part 2 Total Fresh Ingredients: 0");
        return;
    }

    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for i in 1..ranges.len() {
        let next_start = ranges[i].0;
        let next_end = ranges[i].1;

        if next_start <= current_end + 1 {
            if next_end > current_end {
                current_end = next_end;
            }
        } else {
            merged_ranges.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }
    merged_ranges.push((current_start, current_end));

    let mut total_fresh: u64 = 0;
    for (start, end) in merged_ranges {
        total_fresh += end - start + 1;
    }

    println!("Part 2 Total Fresh Ingredients: {}", total_fresh);
}

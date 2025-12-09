pub fn solve(input: &str) -> i32 {
    let mut current_pos = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = match line.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            Some(c) => panic!("Invalid direction '{}'", c),
            None => panic!("Empty line"),
        };
        let amount: i32 = line[1..].parse().expect("Should be a number");

        current_pos = (current_pos + amount * direction).rem_euclid(100);

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

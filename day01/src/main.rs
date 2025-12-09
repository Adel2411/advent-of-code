mod part1;
mod part2;

use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let start = Instant::now();
    let r1 = part1::solve(&input);
    let duration1 = start.elapsed();
    println!("Part 1 Result: {} (took {:?})", r1, duration1);

    let start = Instant::now();
    let r2 = part2::solve(&input);
    let duration2 = start.elapsed();
    println!("Part 2 Result: {} (took {:?})", r2, duration2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1::solve(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2::solve(input), 6);
    }
}

mod part1;
mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    
    part1::solve(&input);
    part2::solve(&input);
}
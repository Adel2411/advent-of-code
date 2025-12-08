mod part1;
mod part2;
mod unionfind;

fn main() {
    let input =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1::solve(&input);
    part2::solve(&input);
}

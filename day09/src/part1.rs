pub fn solve(input: &str) {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect();

    let n = tiles.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    println!("Part 1: {}", max_area);
}
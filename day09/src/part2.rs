use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn solve(input: &str) -> i64 {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let mut xs: BTreeSet<i64> = tiles.iter().map(|&(x, _)| x).collect();
    xs.insert(i64::MIN);
    xs.insert(i64::MAX);
    let shrink_x: BTreeMap<i64, usize> = xs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();

    let mut ys: BTreeSet<i64> = tiles.iter().map(|&(_, y)| y).collect();
    ys.insert(i64::MIN);
    ys.insert(i64::MAX);
    let shrink_y: BTreeMap<i64, usize> = ys.into_iter().enumerate().map(|(i, y)| (y, i)).collect();

    let width = shrink_x.len();
    let height = shrink_y.len();

    let mut grid = vec![vec!['X'; height]; width];

    let n = tiles.len();
    for i in 0..n {
        let (x1, y1) = tiles[i];
        let (x2, y2) = tiles[(i + 1) % n];

        let cx1 = shrink_x[&x1];
        let cx2 = shrink_x[&x2];
        let cy1 = shrink_y[&y1];
        let cy2 = shrink_y[&y2];

        for x in cx1.min(cx2)..=cx1.max(cx2) {
            for y in cy1.min(cy2)..=cy1.max(cy2) {
                grid[x][y] = '#';
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    grid[0][0] = '.';

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == 'X' {
                    grid[nx][ny] = '.';
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    let mut max_area = 0;
    for i in 0..n {
        'outer: for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let cx1 = shrink_x[&x1];
            let cx2 = shrink_x[&x2];
            let cy1 = shrink_y[&y1];
            let cy2 = shrink_y[&y2];

            for x in cx1.min(cx2)..=cx1.max(cx2) {
                for y in cy1.min(cy2)..=cy1.max(cy2) {
                    if grid[x][y] == '.' {
                        continue 'outer;
                    }
                }
            }

            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area
}


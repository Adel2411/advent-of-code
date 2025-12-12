use crate::unionfind::UnionFind;

pub fn solve(input: &str) -> i64 {
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();

    let n = points.len();

    let mut distances: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            let dz = points[i].2 - points[j].2;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            distances.push((dist_sq, i, j));
        }
    }

    distances.sort_by_key(|&(d, _, _)| d);

    let mut uf = UnionFind::new(n);

    let mut num_circuits = n;

    let mut last_pair: (usize, usize) = (0, 0);

    for (_, i, j) in &distances {
        if uf.union(*i, *j) {
            num_circuits -= 1;
            last_pair = (*i, *j);

            if num_circuits == 1 {
                break;
            }
        }
    }

    let x1 = points[last_pair.0].0;
    let x2 = points[last_pair.1].0;
    let result = x1 * x2;

    result
}

use crate::unionfind::UnionFind;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
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

    let mut connections_made = 0;
    for (_, i, j) in &distances {
        if connections_made >= 1000 {
            break;
        }
        uf.union(*i, *j);
        connections_made += 1;
    }

    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let root = uf.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = circuit_sizes.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    let result: usize = sizes.iter().take(3).product();

    result
}

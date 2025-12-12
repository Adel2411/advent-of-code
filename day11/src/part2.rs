use std::collections::HashMap;

pub fn solve(input: &str) -> i64 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].trim();
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();

        graph.insert(device, outputs);
    }

    let svr_dac = count_paths(&graph, "svr", "dac");
    let dac_fft = count_paths(&graph, "dac", "fft");
    let fft_out = count_paths(&graph, "fft", "out");

    let svr_fft = count_paths(&graph, "svr", "fft");
    let fft_dac = count_paths(&graph, "fft", "dac");
    let dac_out = count_paths(&graph, "dac", "out");

    let path1 = svr_dac * dac_fft * fft_out;
    let path2 = svr_fft * fft_dac * dac_out;

    path1 + path2
}

fn count_paths<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &str) -> i64 {
    let mut memo: HashMap<&str, i64> = HashMap::new();
    let mut stack: Vec<(&str, bool)> = vec![(start, false)];

    while let Some((node, expanded)) = stack.pop() {
        if expanded {
            // Time to compute and memoize this node
            if node == end {
                memo.insert(node, 1);
            } else {
                let mut total = 0;
                if let Some(neighbors) = graph.get(node) {
                    for &n in neighbors {
                        if let Some(&c) = memo.get(n) {
                            total += c;
                        }
                    }
                }
                memo.insert(node, total);
            }
            continue;
        }

        // If memoized, skip
        if memo.contains_key(node) {
            continue;
        }

        // Post-order: first mark this node to be processed after children
        stack.push((node, true));

        // Then push children (unexpanded)
        if let Some(neighbors) = graph.get(node) {
            for &n in neighbors {
                if !memo.contains_key(n) {
                    stack.push((n, false));
                }
            }
        }
    }

    *memo.get(start).unwrap_or(&0)
}

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
        let outputs: Vec<&str> = parts[1]
            .split_whitespace()
            .collect();
        
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

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &str,
) -> i64 {
    let mut memo = HashMap::new();
    count_paths_recursive(graph, start, end, &mut memo)
}

fn count_paths_recursive<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    memo: &mut HashMap<&'a str, i64>,
) -> i64 {
    if current == target {
        return 1;
    }
    
    if let Some(&count) = memo.get(current) {
        return count;
    }
    
    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(current) {
        for &neighbor in neighbors {
            total_paths += count_paths_recursive(graph, neighbor, target, memo);
        }
    }
    
    memo.insert(current, total_paths);
    total_paths
}

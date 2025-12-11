use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
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
    
    count_paths(&graph, "you", "out")
}

fn count_paths(graph: &HashMap<&str, Vec<&str>>, current: &str, target: &str) -> i32 {
    if current == target {
        return 1;
    }
    
    if let Some(neighbors) = graph.get(current) {
        let mut total_paths = 0;
        
        for &neighbor in neighbors {
            total_paths += count_paths(graph, neighbor, target);
        }
        
        total_paths
    } else {
        0
    }
}

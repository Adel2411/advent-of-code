pub fn solve(input: &str) -> i32 {
    let (shapes, regions) = parse_input(input);
    
    let mut count = 0;
    for region in &regions {
        if can_fit(region, &shapes) {
            count += 1;
        }
    }
    
    count
}

type Shape = Vec<Vec<bool>>;

fn parse_input(input: &str) -> (Vec<Shape>, Vec<((i32, i32), Vec<i32>)>) {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<((i32, i32), Vec<i32>)> = Vec::new();
    let mut curr_shape: Vec<Vec<bool>> = Vec::new();
    
    for line in input.lines() {
        let line = line.trim();
        
        if line.contains('x') && line.contains(':') {
            if !curr_shape.is_empty() {
                shapes.push(curr_shape);
                curr_shape = Vec::new();
            }
            
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let size_part = tokens[0].trim_end_matches(':');
            let size_parts: Vec<&str> = size_part.split('x').collect();
            let width: i32 = size_parts[0].parse().unwrap();
            let height: i32 = size_parts[1].parse().unwrap();
            
            let counts: Vec<i32> = tokens[1..]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();
            
            regions.push(((width, height), counts));
        } else if line.contains(':') && !line.contains('#') && !line.contains('.') {
            if !curr_shape.is_empty() {
                shapes.push(curr_shape);
            }
            curr_shape = Vec::new();
        } else if line.contains('#') || line.contains('.') {
            let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
            curr_shape.push(row);
        }
    }
    
    if !curr_shape.is_empty() {
        shapes.push(curr_shape);
    }
    
    (shapes, regions)
}

fn can_fit(region: &((i32, i32), Vec<i32>), shapes: &[Shape]) -> bool {
    let ((width, height), counts) = region;
    let region_area = width * height;
    
    let mut total_shape_area = 0;
    
    for (i, count) in counts.iter().enumerate() {
        if i >= shapes.len() {
            continue;
        }
        
        let shape_size: i32 = shapes[i]
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count() as i32)
            .sum();
        
        total_shape_area += shape_size * count;
    }
    
    region_area >= total_shape_area
}

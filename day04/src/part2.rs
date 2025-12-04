pub fn solve(input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() {
        return;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut to_remove = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let mut neighbor_count = 0;

                    for &(dr, dc) in &DIRECTIONS {
                        let nr = row as i32 + dr;
                        let nc = col as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }

                    if neighbor_count < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len();
        for (r, c) in to_remove {
            grid[r][c] = '.';
        }
    }

    println!("Part 2: {}", total_removed);
}

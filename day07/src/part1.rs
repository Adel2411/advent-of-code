use std::collections::HashSet;

pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut beams: HashSet<usize> = HashSet::new();

    for c in 0..cols {
        if grid[0][c] == 'S' {
            beams.insert(c);
            break;
        }
    }

    let mut split_count = 0;

    for r in 0..rows {
        if beams.is_empty() {
            break;
        }

        let mut next_beams: HashSet<usize> = HashSet::new();

        for &c in &beams {
            let cell = grid[r][c];

            match cell {
                'S' | '.' => {
                    if r + 1 < rows {
                        next_beams.insert(c);
                    }
                }
                '^' => {
                    split_count += 1;

                    if r + 1 < rows {
                        if c > 0 {
                            next_beams.insert(c - 1);
                        }
                        if c + 1 < cols {
                            next_beams.insert(c + 1);
                        }
                    }
                }
                _ => {
                    if r + 1 < rows {
                        next_beams.insert(c);
                    }
                }
            }
        }
        beams = next_beams;
    }

    split_count
}


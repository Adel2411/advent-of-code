use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut beams: HashMap<usize, u64> = HashMap::new();

    for c in 0..cols {
        if grid[0][c] == 'S' {
            beams.insert(c, 1);
            break;
        }
    }

    let mut finished_timelines: u64 = 0;

    for r in 0..rows {
        if beams.is_empty() {
            break;
        }

        let mut next_beams: HashMap<usize, u64> = HashMap::new();

        for (&c, &count) in &beams {
            let cell = grid[r][c];

            match cell {
                'S' | '.' => {
                    if r + 1 < rows {
                        *next_beams.entry(c).or_insert(0) += count;
                    } else {
                        finished_timelines += count;
                    }
                }
                '^' => {
                    if c > 0 {
                        if r + 1 < rows {
                            *next_beams.entry(c - 1).or_insert(0) += count;
                        } else {
                            finished_timelines += count;
                        }
                    } else {
                        finished_timelines += count;
                    }

                    if c + 1 < cols {
                        if r + 1 < rows {
                            *next_beams.entry(c + 1).or_insert(0) += count;
                        } else {
                            finished_timelines += count;
                        }
                    } else {
                        finished_timelines += count;
                    }
                }
                _ => {
                    if r + 1 < rows {
                        *next_beams.entry(c).or_insert(0) += count;
                    } else {
                        finished_timelines += count;
                    }
                }
            }
        }
        beams = next_beams;
    }

    finished_timelines
}


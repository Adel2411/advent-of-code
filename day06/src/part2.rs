pub fn solve(input: &str) {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return;
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut columns_are_separators = vec![true; max_width];

    for col_idx in 0..max_width {
        for line in &lines {
            if col_idx < line.len() && line.as_bytes()[col_idx] != b' ' {
                columns_are_separators[col_idx] = false;
                break;
            }
        }
    }

    let mut problems = Vec::new();
    let mut block_start_index = None;

    for col_idx in 0..max_width {
        if !columns_are_separators[col_idx] {
            if block_start_index.is_none() {
                block_start_index = Some(col_idx);
            }
        } else if let Some(start) = block_start_index {
            problems.push(extract_vertical_problem(&lines, start, col_idx));
            block_start_index = None;
        }
    }
    if let Some(start) = block_start_index {
        problems.push(extract_vertical_problem(&lines, start, max_width));
    }

    let mut total_sum: u64 = 0;
    for (operator, values) in problems {
        let computation = match operator {
            '+' => values.iter().sum::<u64>(),
            '*' => values.iter().product::<u64>(),
            _ => 0,
        };
        total_sum += computation;
    }

    println!("Part 2 Grand Total: {}", total_sum);
}

fn extract_vertical_problem(lines: &[&str], start_col: usize, end_col: usize) -> (char, Vec<u64>) {
    let mut numeric_values = Vec::new();
    let mut operation_symbol = '+';

    let last_line_index = lines.len() - 1;
    let operator_row = lines[last_line_index];
    
    let safe_start = start_col.min(operator_row.len());
    let safe_end = end_col.min(operator_row.len());
    
    if safe_start < safe_end {
        let segment = &operator_row[safe_start..safe_end];
        if let Some(symbol) = segment.chars().find(|c| !c.is_whitespace()) {
            operation_symbol = symbol;
        }
    }

    for col_idx in (start_col..end_col).rev() {
        let mut number_string = String::new();
        for line in &lines[0..last_line_index] {
            if col_idx < line.len() {
                let character = line.as_bytes()[col_idx] as char;
                if !character.is_whitespace() {
                    number_string.push(character);
                }
            }
        }
        
        if !number_string.is_empty() {
            if let Ok(parsed_number) = number_string.parse::<u64>() {
                numeric_values.push(parsed_number);
            }
        }
    }

    (operation_symbol, numeric_values)
}

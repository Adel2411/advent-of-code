pub fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut columns_are_separators = vec![true; max_width];

    for col_indices in 0..max_width {
        for line in &lines {
            if col_indices < line.len() && line.as_bytes()[col_indices] != b' ' {
                columns_are_separators[col_indices] = false;
                break;
            }
        }
    }

    let mut problems = Vec::new();
    let mut block_start_index = None;

    for col_index in 0..max_width {
        if !columns_are_separators[col_index] {
            if block_start_index.is_none() {
                block_start_index = Some(col_index);
            }
        } else if let Some(start) = block_start_index {
            problems.push(extract_problem_data(&lines, start, col_index));
            block_start_index = None;
        }
    }
    if let Some(start) = block_start_index {
        problems.push(extract_problem_data(&lines, start, max_width));
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

    total_sum
}

fn extract_problem_data(lines: &[&str], start_col: usize, end_col: usize) -> (char, Vec<u64>) {
    let mut numeric_values = Vec::new();
    let mut operation_symbol = '+';

    for (index, line) in lines.iter().enumerate() {
        let safe_start = start_col.min(line.len());
        let safe_end = end_col.min(line.len());
        let line_segment = &line[safe_start..safe_end];

        if line_segment.trim().is_empty() {
            continue;
        }

        if index == lines.len() - 1 {
            if let Some(symbol) = line_segment.chars().find(|c| !c.is_whitespace()) {
                operation_symbol = symbol;
            }
        } else if let Ok(parsed_number) = line_segment.trim().parse::<u64>() {
            numeric_values.push(parsed_number);
        }
    }

    (operation_symbol, numeric_values)
}

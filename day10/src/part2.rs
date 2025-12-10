use good_lp::*;

pub fn solve(input: &str) -> i64 {
    let mut total_presses = 0i64;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let curly_start = line.find('{').unwrap();
        let curly_end = line.find('}').unwrap();
        let joltage_str = &line[curly_start + 1..curly_end];

        let jolts: Vec<usize> = joltage_str
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        let bracket_end = line.find(']').unwrap();
        let buttons_section = &line[bracket_end + 1..curly_start];

        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = buttons_section.chars().collect();

        while i < chars.len() {
            if chars[i] == '(' {
                let start = i + 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                let paren_content: String = chars[start..i].iter().collect();

                let mut button_affects = Vec::new();
                for num_str in paren_content.split(',') {
                    let num_str = num_str.trim();
                    if !num_str.is_empty() {
                        button_affects.push(num_str.parse::<usize>().unwrap());
                    }
                }
                buttons.push(button_affects);
            }
            i += 1;
        }

        let mut vars = variables!();

        let press_vars: Vec<Variable> = (0..buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));

        let mut exprs = vec![0.into_expression(); jolts.len()];
        for i in 0..buttons.len() {
            for &x in &buttons[i] {
                exprs[x] += press_vars[i];
            }
        }

        for (e, &j) in exprs.into_iter().zip(jolts.iter()) {
            problem = problem.with(e.eq(j as f64));
        }

        let sol = problem.solve().unwrap();
        let min_presses = press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as usize;

        total_presses += min_presses as i64;
    }

    total_presses
}
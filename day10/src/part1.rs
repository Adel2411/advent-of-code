pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let s = line.trim();

            let b0 = s.find('[').unwrap();
            let b1 = s.find(']').unwrap();
            let mut target = 0u64;
            let mut n_lights = 0usize;
            for (i, c) in s[b0 + 1..b1].chars().enumerate() {
                if c == '#' {
                    target |= 1u64 << i;
                }
                n_lights += 1;
            }

            let curly = s.find('{').unwrap_or(s.len());
            let chars: Vec<char> = s[b1 + 1..curly].chars().collect();
            let mut buttons: Vec<u64> = Vec::new();
            let mut i = 0usize;
            while i < chars.len() {
                if chars[i] == '(' {
                    let start = i + 1;
                    while chars[i] != ')' {
                        i += 1;
                    }
                    let content: String = chars[start..i].iter().collect();
                    let mut mask = 0u64;
                    for num in content.split(',').map(str::trim).filter(|x| !x.is_empty()) {
                        let idx: usize = num.parse().unwrap();
                        mask |= 1u64 << idx;
                    }
                    buttons.push(mask);
                }
                i += 1;
            }

            let m = buttons.len();
            if m == 0 {
                return if target == 0 { 0 } else { 0 };
            }

            let mut rows = vec![0u64; n_lights];
            let mut rhs = vec![0u8; n_lights];
            for (col, &btn_mask) in buttons.iter().enumerate() {
                for r in 0..n_lights {
                    if (btn_mask >> r) & 1 == 1 {
                        rows[r] |= 1u64 << col;
                    }
                }
            }
            for r in 0..n_lights {
                rhs[r] = ((target >> r) & 1) as u8;
            }

            let mut pivot_row = vec![None; m];
            let mut row = 0usize;
            for col in 0..m {
                if let Some(r) = (row..n_lights).find(|&r| ((rows[r] >> col) & 1) == 1) {
                    rows.swap(row, r);
                    rhs.swap(row, r);
                    for r2 in 0..n_lights {
                        if r2 != row && ((rows[r2] >> col) & 1) == 1 {
                            rows[r2] ^= rows[row];
                            rhs[r2] ^= rhs[row];
                        }
                    }
                    pivot_row[col] = Some(row);
                    row += 1;
                    if row == n_lights {
                        break;
                    }
                }
            }
            for r in row..n_lights {
                if rows[r] == 0 && rhs[r] == 1 {
                    return 0;
                }
            }

            let mut x0 = 0u64;
            for col in 0..m {
                if let Some(r) = pivot_row[col] {
                    if rhs[r] == 1 {
                        x0 |= 1u64 << col;
                    }
                }
            }

            let free_cols: Vec<usize> = (0..m).filter(|&c| pivot_row[c].is_none()).collect();
            let mut null_basis = Vec::<u64>::new();
            for &f in &free_cols {
                let mut v = 1u64 << f;
                for p in 0..m {
                    if let Some(r) = pivot_row[p] {
                        if ((rows[r] >> f) & 1) == 1 {
                            v |= 1u64 << p;
                        }
                    }
                }
                null_basis.push(v);
            }

            let k = null_basis.len();
            if k <= 26 {
                let mut best = usize::MAX;
                let limit = 1usize << k;
                for mask in 0..limit {
                    let mut comb = 0u64;
                    for (i, v) in null_basis.iter().enumerate() {
                        if ((mask >> i) & 1) == 1 {
                            comb ^= v;
                        }
                    }
                    let cand = x0 ^ comb;
                    best = best.min(cand.count_ones() as usize);
                }
                return best as i32;
            }

            let mut best = x0;
            let mut best_w = best.count_ones() as u32;
            for _pass in 0..3 {
                let mut improved = false;
                for &v in &null_basis {
                    let c = best ^ v;
                    let w = c.count_ones();
                    if w < best_w {
                        best = c;
                        best_w = w;
                        improved = true;
                    }
                }
                if !improved {
                    break;
                }
            }
            best_w as i32
        })
        .sum()
}

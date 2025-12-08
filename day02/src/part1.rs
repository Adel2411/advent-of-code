use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

pub fn solve(input: &str) {
    let time = Instant::now();

    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .filter_map(|r| {
            let mut it = r.split('-');
            let start = it.next()?.parse::<u64>().ok()?;
            let end = it.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();

    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let global_sum = Arc::new(AtomicU64::new(0));

    // Split work for threads
    let mut chunks = vec![Vec::new(); threads];
    for (i, r) in ranges.into_iter().enumerate() {
        chunks[i % threads].push(r);
    }

    let mut handles = Vec::with_capacity(threads);

    for chunk in chunks {
        let global_ref = Arc::clone(&global_sum);

        handles.push(thread::spawn(move || {
            let mut local_sum = 0u64;

            for (start, end) in chunk {
                for num in start..=end {
                    if is_invalid_id(num) {
                        local_sum += num;
                    }
                }
            }

            global_ref.fetch_add(local_sum, Ordering::Relaxed);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "Part 1 Sum of Invalid IDs: {}, time: {:.2?}",
        global_sum.load(Ordering::Relaxed),
        time.elapsed()
    );
}

#[inline(always)]
fn is_invalid_id(num: u64) -> bool {
    let mut tmp = num;
    let mut digits = [0u8; 20];
    let mut len = 0;

    while tmp > 0 {
        digits[len] = (tmp % 10) as u8;
        tmp /= 10;
        len += 1;
    }

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;

    for i in 0..half {
        if digits[i] != digits[i + half] {
            return false;
        }
    }

    true
}

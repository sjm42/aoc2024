// bin/day02.rs

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut n_safe1: u64 = 0;
    let mut n_safe2: u64 = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();

        let reports = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>();
        debug!("{reports:?}");

        if is_safe(&reports) {
            n_safe1 += 1;
            n_safe2 += 1;
            continue;
        }

        for i in 0..reports.len() {
            let mut try_rep = reports.clone();
            try_rep.remove(i);
            if is_safe(&try_rep) {
                n_safe2 += 1;
                break;
            }
        }
    }

    println!("Safe1: {n_safe1}");
    println!("Safe2: {n_safe2}");
    Ok(())
}

fn is_safe(rep: &[i64]) -> bool {
    let sign = (rep[1] - rep[0]).signum();
    if sign == 0 {
        return false;
    }

    let mut prev = rep[0];
    for &n in rep.iter().skip(1) {
        let diff = n - prev;
        if diff.signum() != sign {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
        prev = n;
    }

    true
}

// EOF

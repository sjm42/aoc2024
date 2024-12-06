// bin/day05.rs

use aoc2024::*;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut orders = HashMap::new();
    let mut updates = Vec::new();

    // read input data
    let mut do_order = true;
    for (line_n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line_t = line.trim().to_owned();

        if do_order {
            if line_t.is_empty() {
                do_order = false;
                continue;
            }
            if let Some((xs, ys)) = line_t.split_once('|') {
                let (x, y) = (xs.parse::<i64>()?, ys.parse::<i64>()?);
                orders
                    .entry(x)
                    .and_modify(|v: &mut Vec<i64>| v.push(y))
                    .or_insert_with(|| vec![y]);
            } else {
                error!("Invalid line #{line_n}: {line_t:?}");
                bail!("Failed to parse input.");
            }
        } else {
            let pages = line_t
                .split(',')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>();
            updates.push(pages);
        }
    }
    debug!("Got orders: {orders:?}");
    debug!("Got updates: {updates:?}");

    let mut sum1 = 0;
    let mut sum2 = 0;
    for update in updates {
        if check_order(&orders, &update) {
            debug!("--- OK");
            let middle = update[update.len() / 2];
            debug!("--- middle: {middle}");
            sum1 += middle;
        } else {
            debug!("--- FAIL");
            let sorted = sort_order(&orders, &update);
            debug!("--- SORTED: {sorted:?}");
            let middle = sorted[sorted.len() / 2];
            debug!("--- middle: {middle}");
            sum2 += middle;
        }
    }
    println!("Sum1: {sum1}");
    println!("Sum1: {sum2}");
    Ok(())
}

fn check_order(orders: &HashMap<i64, Vec<i64>>, update: &[i64]) -> bool {
    debug!("### Checking update {update:?}");
    for (i, &page) in update.iter().enumerate().rev() {
        debug!("Checking page[{i}] {page}");
        if let Some(rules) = orders.get(&page) {
            debug!("[{i}] found rules {rules:?}");
            for &rule in rules {
                for j in (0..i).rev() {
                    if update[j] == rule {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn sort_order(orders: &HashMap<i64, Vec<i64>>, pages: &[i64]) -> Vec<i64> {
    debug!("### Sorting pages {pages:?}");
    let mut sorted = pages.to_vec();
    sorted.sort_by(|a, b| {
        if let Some(rules) = orders.get(a) {
            for &rule in rules {
                if rule == *b {
                    return Ordering::Less;
                }
            }
        }
        if let Some(rules) = orders.get(b) {
            for &rule in rules {
                if rule == *a {
                    return Ordering::Greater;
                }
            }
        }
        Ordering::Equal
    });
    sorted
}
// EOF

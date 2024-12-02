// bin/day01.rs

use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::anyhow;
use itertools::Itertools;

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();

        let (a, b) = line
            .split_whitespace()
            .take(2)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect_tuple()
            .ok_or(anyhow!("Invalid input on line {n}"))?;
        debug!("{a} {b}");
        list_a.push(a);
        list_b.push(b);
    }

    let mut freq_hash = HashMap::new();
    list_b.iter().for_each(|&n| {
        freq_hash.entry(n).and_modify(|e| *e += 1).or_insert(1);
    });
    let similarity = list_a
        .iter()
        .map(|&n| *(freq_hash.get(&n).unwrap_or(&0)) * n)
        .sum::<u64>();

    list_a.sort();
    list_b.sort();
    let sum = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u64>();
    println!("Sum: {sum}");
    println!("Similarity: {similarity}");

    Ok(())
}
// EOF

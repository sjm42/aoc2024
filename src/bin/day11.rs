// bin/day11.rs

use aoc2024::*;
use num::bigint::BigUint;
use num::Zero;
use std::collections::HashMap;
use std::ops::Mul;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let stones = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok().map(BigUint::from))
        .collect::<Vec<BigUint>>();

    debug!("Got stones: {stones:?}");

    let mut cache = HashMap::new();
    let c1 = stones
        .iter()
        .map(|i| count_stones(&mut cache, i, 6))
        .sum::<u64>();
    println!("After 6 iterations, got {c1} stones");

    let c2 = stones
        .iter()
        .map(|i| count_stones(&mut cache, i, 25))
        .sum::<u64>();
    println!("After 25 iterations, got {c2} stones");

    let c3 = stones
        .iter()
        .map(|i| count_stones(&mut cache, i, 75))
        .sum::<u64>();
    println!("After 75 iterations, got {c3} stones");

    Ok(())
}

#[allow(dead_code)]
fn count_stones(cache: &mut HashMap<(BigUint, usize), u64>, stone: &BigUint, mut n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    n -= 1;
    if stone.is_zero() {
        return count_stones(cache, &BigUint::from(1u8), n);
    }

    let s = stone.to_string();
    let len = s.len();
    // s has even number of digits
    if len % 2 == 0 {
        if cache.contains_key(&(stone.clone(), n)) {
            return *cache.get(&(stone.clone(), n)).unwrap();
        }
        let middle = len / 2;
        let count = count_stones(cache, &s[0..middle].parse::<BigUint>().unwrap(), n)
            + count_stones(cache, &s[middle..len].parse::<BigUint>().unwrap(), n);
        cache.insert((stone.clone(), n), count);
        return count;
    }
    debug!("At level {n}");
    count_stones(cache, &stone.mul(2024u32), n)
}
// EOF

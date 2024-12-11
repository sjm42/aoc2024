// bin/day11.rs

use aoc2024::*;
use num::bigint::BigUint;
use num::Zero;
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

    let c1 = stones.iter().map(|i| count_stones(i, 6)).sum::<u64>();
    println!("After 6 iterations, got {c1} stones");

    let c2 = stones.iter().map(|i| count_stones(i, 25)).sum::<u64>();
    println!("After 25 iterations, got {c2} stones");

    let c3 = stones.iter().map(|i| count_stones(i, 75)).sum::<u64>();
    println!("After 75 iterations, got {c3} stones");

    Ok(())
}

#[allow(dead_code)]
fn count_stones(stone: &BigUint, mut n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    n -= 1;

    if stone.is_zero() {
        return count_stones(&BigUint::from(1u8), n);
    }



    let s = stone.to_string();
    let len = s.len();
    // s has even number of digits
    if len % 2 == 0 {
        let middle = len / 2;
        return count_stones(&s[0..middle].parse::<BigUint>().unwrap(), n)
            + count_stones(&s[middle..len].parse::<BigUint>().unwrap(), n);
    }
    debug!("At level {n}");
    count_stones(&stone.mul(2024u32), n)
}
// EOF

// bin/day11.rs

use std::collections::HashMap;

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let stones = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u128>().ok())
        .collect::<Vec<u128>>();

    debug!("Got stones: {stones:?}");

    let mut cache = HashMap::new();
    let mut max_len = 0;
    let c1 = stones
        .iter()
        .map(|&i| count_stones(&mut cache, i, 6, &mut max_len))
        .sum::<u128>();
    println!("After 6 iterations, got {c1} stones");

    let c2 = stones
        .iter()
        .map(|&i| count_stones(&mut cache, i, 25, &mut max_len))
        .sum::<u128>();
    println!("After 25 iterations, got {c2} stones");

    let c3 = stones
        .iter()
        .map(|&i| count_stones(&mut cache, i, 75, &mut max_len))
        .sum::<u128>();
    println!("After 75 iterations, got {c3} stones");

    info!("Label max len: {max_len}");

    Ok(())
}

#[allow(dead_code)]
fn count_stones(
    cache: &mut HashMap<(u128, usize), u128>,
    stone: u128,
    mut n: usize,
    max_len: &mut usize,
) -> u128 {
    if n == 0 {
        return 1;
    }
    n -= 1;
    if stone == 0 {
        return count_stones(cache, 1, n, max_len);
    }

    let s = stone.to_string();
    let len = s.len();
    if len > *max_len {
        *max_len = len;
    }
    // s has even number of digits
    if len % 2 == 0 {
        if cache.contains_key(&(stone, n)) {
            return *cache.get(&(stone, n)).unwrap();
        }
        let middle = len / 2;
        let num1 = s[..middle].parse::<u128>().unwrap();
        let num2 = s[middle..].parse::<u128>().unwrap();
        let count = count_stones(cache, num1, n, max_len) + count_stones(cache, num2, n, max_len);
        cache.insert((stone, n), count);
        return count;
    }
    debug!("At level {n}");
    count_stones(cache, stone * 2024, n, max_len)
}
// EOF

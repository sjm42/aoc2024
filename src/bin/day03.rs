// bin/day03.rs

use regex::Regex;

use aoc2024::*;

// const MUL_RE: &str = r"mul\((\d+),(\d+)\)";
const OP_RE: &str = r"(mul|do|don\'t)\(([,\d]*)\)";
const MUL_RE: &str = r"^(\d+),(\d+)$";

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let op_re = Regex::new(OP_RE)?;
    let mul_re = Regex::new(MUL_RE)?;

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut do_mul = true;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();

        for op in op_re.captures_iter(line) {
            if &op[1] == "do" && op[2].is_empty() {
                debug!("do: {op:?}");
                do_mul = true;
                continue;
            }

            if &op[1] == "don't" && op[2].is_empty() {
                debug!("dont: {op:?}");
                do_mul = false;
                continue;
            }

            if &op[1] == "mul" && !op[2].is_empty() {
                if let Some(cap) = mul_re.captures(&op[2]) {
                    let x = cap[1].to_string().parse::<i64>().unwrap_or(0);
                    let y = cap[2].to_string().parse::<i64>().unwrap_or(0);
                    println!("mul({x},{y})");
                    sum1 += x * y;
                    if do_mul {
                        sum2 += x * y;
                    }
                }
            }
        }
    }
    println!("sum1: {sum1}");
    println!("sum2: {sum2}");
    Ok(())
}
// EOF

// bin/day07.rs

use aoc2024::*;

#[derive(Copy, Clone, Debug)]
enum CalcOp {
    Add,
    Mul,
    Cat,
}

impl CalcOp {
    pub fn op(self, a: i64, b: i64) -> i64 {
        match self {
            CalcOp::Add => a + b,
            CalcOp::Mul => a * b,
            CalcOp::Cat => (a * (10_i64.pow(1 + b.checked_ilog10().unwrap_or_default()))) + b,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut tasks = Vec::new();
    // read input data
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        if let Some((result, list_s)) = line.split_once(':') {
            let list = list_s
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>();
            tasks.push((result.parse::<i64>()?, list));
        } else {
            bail!("Invalid line #{line_num}: {line:?}");
        }
    }
    debug!("Got tasks: {tasks:?}");

    let mut total1 = 0;
    for (result, nums) in &tasks {
        debug!("### testing: {result} with {nums:?}");
        if try_out(*result, nums, &[CalcOp::Add, CalcOp::Mul]) {
            total1 += *result;
        }
    }
    println!("Total1: {total1}");

    let mut total2 = 0;
    for (result, nums) in &tasks {
        debug!("### testing: {result} with {nums:?}");
        if try_out(*result, nums, &[CalcOp::Add, CalcOp::Mul, CalcOp::Cat]) {
            total2 += *result;
        }
    }
    println!("Total2: {total2}");

    Ok(())
}

fn try_out(result: i64, nums: &[i64], ops: &[CalcOp]) -> bool {
    let ops_n = ops.len().pow(nums.len() as u32 - 1);
    let mut op_list = Vec::with_capacity(ops_n);
    for &op in ops {
        op_list.push(vec![op]);
    }

    for _n in nums.iter().skip(2) {
        let o_len = op_list.len();
        let mut new_list = Vec::with_capacity(o_len * ops.len());
        for olist in op_list.iter() {
            for &op in ops {
                let mut new_olist = Vec::with_capacity(o_len + 1);
                new_olist.extend_from_slice(olist);
                new_olist.push(op);
                new_list.push(new_olist);
            }
        }
        op_list.clear();
        op_list.append(&mut new_list);
    }
    debug!("try_out {op_list:?}");

    for ops in op_list.iter() {
        let mut calc = nums[0];
        nums.iter().skip(1).enumerate().for_each(|(i, &num)| {
            calc = ops[i].op(calc, num);
        });
        if calc == result {
            debug!("Found result with {ops:?}");
            return true;
        }
    }

    false
}
// EOF

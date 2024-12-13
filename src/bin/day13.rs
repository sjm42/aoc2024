// bin/day13.rs

use std::cmp;

use aoc2024::*;

const MAX_BUTTON: i64 = 100;

enum DataState {
    ButtonA,
    ButtonB,
    Prize,
}

const STR_BUTTON_A: &str = "Button A:";
const RE_BUTTON_A: &str = r"^Button\s+A:\s+X\+(\d+),\s+Y\+(\d+)$";
const STR_BUTTON_B: &str = "Button B:";
const RE_BUTTON_B: &str = r"^Button\s+B:\s+X\+(\d+),\s+Y\+(\d+)$";
const STR_PRIZE: &str = "Prize:";
const RE_PRIZE: &str = r"^Prize:\s+X\=(\d+),\s+Y\=(\d+)$";

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let re_button_a = regex::Regex::new(RE_BUTTON_A)?;
    let re_button_b = regex::Regex::new(RE_BUTTON_B)?;
    let re_prize = regex::Regex::new(RE_PRIZE)?;

    // read input data
    let mut tasks = Vec::new();
    let mut read_state = DataState::ButtonA;
    let (mut a_x, mut a_y, mut b_x, mut b_y) = (0, 0, 0, 0);
    let (mut p_x, mut p_y);

    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // we expect incoming data to be in predefined order
        match read_state {
            DataState::ButtonA if line.starts_with(STR_BUTTON_A) => {
                (a_x, a_y) = get_nums(&re_button_a, line, line_num)?;
                debug!("Got ButtonA: ({a_x},{a_y})");
                read_state = DataState::ButtonB;
            }
            DataState::ButtonB if line.starts_with(STR_BUTTON_B) => {
                (b_x, b_y) = get_nums(&re_button_b, line, line_num)?;
                debug!("Got ButtonB: ({b_x},{b_y})");
                read_state = DataState::Prize;
            }
            DataState::Prize if line.starts_with(STR_PRIZE) => {
                (p_x, p_y) = get_nums(&re_prize, line, line_num)?;
                debug!("Got Prize: ({p_x},{p_y})");
                tasks.push((a_x, a_y, b_x, b_y, p_x, p_y));

                read_state = DataState::ButtonA;
            }
            _ => {
                bail!("Invalid input on line #{line_num}: \"{line}\"");
            }
        }
    }
    debug!("Tasks: {:?}", tasks);

    let cost: i64 = tasks
        .iter()
        .filter_map(|t| find_ab_stupid((t.0, t.1), (t.2, t.3), (t.4, t.5)))
        .map(|(a, b)| a * 3 + b)
        .sum();
    println!("Part 1 (stupid) cost: {cost}");

    let cost1 = tasks
        .iter()
        .map(|t| solve_ab((t.0, t.1), (t.2, t.3), (t.4, t.5), 0))
        .sum::<i64>();
    println!("Part 1 (solve) cost: {cost1}");

    Ok(())
}

fn get_nums(re: &regex::Regex, input: &str, line_num: usize) -> anyhow::Result<(i64, i64)> {
    if let Some(caps) = re.captures(input) {
        let nums = caps
            .iter()
            .skip(1)
            .filter(|c| c.is_some())
            .map(|s| s.unwrap().as_str().parse::<i64>().unwrap_or_default())
            .collect::<Vec<_>>();
        // debug!("Found {nums:?}");
        if nums.len() == 2 {
            return Ok((nums[0], nums[1]));
        }
    }
    Err(anyhow!("Invalid input on line #{line_num}: \"{input}\""))
}

fn find_ab_stupid(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> Option<(i64, i64)> {
    info!("*** Trying to solve {p:?} from A{a:?} + B{b:?}");
    for n_b in (0..cmp::min(MAX_BUTTON, p.0 / b.0)).rev() {
        let rem_x = p.0 - n_b * b.0;
        if rem_x % a.0 == 0 {
            let n_a = rem_x / a.0;
            debug!("* found x, {n_a}*{} + {n_b}*{} = {}", a.0, b.0, p.0);
            if n_a > MAX_BUTTON {
                debug!("* uh false alarm, A is too big.");
                break;
            }
            if n_a * a.1 + n_b * b.1 == p.1 {
                info!("* OK A={n_a} + B={n_b} is a match! ***");
                info!(
                    "* {n_a}*{} + {n_b}*{} = {} and {n_a}*{} + {n_b}*{} = {}",
                    a.0, b.0, p.0, a.1, b.1, p.1
                );
                return Some((n_a, n_b));
            }
        }
    }
    None
}

fn solve_ab(a: (i64, i64), b: (i64, i64), p: (i64, i64), offset: i64) -> i64 {
    let prize = (p.0 + offset, p.1 + offset);
    let det = a.0 * b.1 - a.1 * b.0;
    let n_a = (prize.0 * b.1 - prize.1 * b.0) / det;
    let n_b = (a.0 * prize.1 - a.1 * prize.0) / det;
    if (a.0 * n_a + b.0 * n_b, a.1 * n_a + b.1 * n_b) == (prize.0, prize.1) {
        n_a * 3 + n_b
    } else {
        0
    }
}

// EOF

// bin/day06.rs
#![allow(clippy::type_complexity)]

use aoc2024::*;
use std::collections::{hash_map::Entry, HashMap};

const START_POS: char = '^';

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut map = Vec::new();
    let mut len_l = 0;
    let mut pos_x = None;
    let mut pos_y = None;
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;

        // all lines must be equal length, and we are being Unicode/UTF8 safe
        let len = line.chars().count();
        if len_l == 0 {
            len_l = line.len();
        } else if len != len_l {
            bail!("Line #{line_num} length {len_l} differs from others. Exit.");
        }

        let row = line.chars().collect::<Vec<char>>();
        if let Some((pos, _)) = row.iter().enumerate().find(|(_, &c)| c == START_POS) {
            if pos_x.is_none() && pos_y.is_none() {
                info!("Found start position ({line_num},{pos})");
                pos_y = Some(line_num as i32);
                pos_x = Some(pos as i32);
            } else {
                bail!("Duplicate start position ({line_num},{pos})");
            }
        }
        map.push(row.clone());
    }

    if pos_x.is_none() || pos_y.is_none() {
        bail!("Start position does not exist");
    }
    info!("Read map of {} lines, len {len_l}", map.len());
    debug!("Got map:");
    print_map(&map);

    // brute force
    // let n_block = block_guard(&map_walk1, start, direction);
    // println!("Method 2 found blocking options: {n_block}");

    let start = (pos_x.unwrap(), pos_y.unwrap());
    let direction = (0, -1);
    let mut walked = HashMap::new();
    let (infinite, i, b, pos, dir) =
        guard_walk(&mut map, &mut walked, start, direction, false);
    println!(
        "Walk1: Guard walked {i} steps at {pos:?} heading to {dir:?}. Infinite loop: {infinite}"
    );
    println!(
        "Walk1: visited positions# {} possible block positions# {b}",
        count_visited(&map)
    );
    debug!("Map after walk1:");
    print_map(&map);

    Ok(())
}

fn guard_walk(
    map: &mut [Vec<char>],
    walked: &mut HashMap<((i32, i32), (i32, i32)), i32>,
    start: (i32, i32),
    direction: (i32, i32),
    check_only: bool,
) -> (bool, i32, i32, (i32, i32), (i32, i32)) {
    let size_x = map[0].len() as i32;
    let size_y = map.len() as i32;
    let mut pos = start;
    let mut dir = direction;

    let mut blocks = 0;
    let mut i = 0;
    loop {
        if !check_only {
            map[pos.1 as usize][pos.0 as usize] = 'X';
        }

        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if new_pos.0 < 0 || new_pos.0 >= size_x || new_pos.1 < 0 || new_pos.1 >= size_y {
            // just walked out of map
            return (false, i, blocks, pos, dir);
        }

        let try_pos = &mut map[new_pos.1 as usize][new_pos.0 as usize];

        if *try_pos == '#' {
            dir = turn_right(dir);
            continue;
        }

        let key = (new_pos, dir);
        if let Entry::Vacant(e) = walked.entry(key) {
            e.insert(i);
        } else {
            // I have been there, walking to same direction!
            return (true, i, blocks, pos, dir);
        }

        if !check_only && *try_pos != '#' && *try_pos != 'X' {
            let save = *try_pos;
            // try to insert new block
            *try_pos = '#';
            debug!("Check blocking at {new_pos:?}");
            let mut test_walked = walked.clone();

            debug!("Walked size: {}", test_walked.len());
            let (success, _, _, _, _) = guard_walk(map, &mut test_walked, pos, dir, true);
            map[new_pos.1 as usize][new_pos.0 as usize] = save;
            if success {
                blocks += 1;
                debug!("Found block position: {new_pos:?}");
            }
        }

        pos = new_pos;
        i += 1;
    }
}

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    (-dir.1, dir.0)
}

fn count_visited(map: &[Vec<char>]) -> usize {
    let mut visited = 0;
    map.iter().for_each(|row| {
        visited += row.iter().filter(|&&c| c == 'X').count();
    });
    visited
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        debug!("{}", row.iter().collect::<String>());
    }
}

// brute force method, not used anymore
#[allow(dead_code)]
fn block_guard(map: &[Vec<char>], start: (i32, i32), direction: (i32, i32)) -> usize {
    let size_x = map[0].len() as i32;
    let size_y = map.len() as i32;

    let mut n_inf = 0;
    for y in 0..size_y {
        for x in 0..size_x {
            if map[y as usize][x as usize] != 'X' {
                continue;
            }
            let mut try_map = clone_map(map);
            try_map[y as usize][x as usize] = '#';
            let mut walked = HashMap::new();
            let (infinite, _, _, _, _) =
                guard_walk(&mut try_map, &mut walked, start, direction, true);
            if infinite {
                n_inf += 1;
            }
        }
    }
    n_inf
}

fn clone_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = Vec::with_capacity(map.len());
    map.iter().for_each(|row| {
        new_map.push(row.clone());
    });
    new_map
}
// EOF

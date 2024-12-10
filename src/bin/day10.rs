// bin/day10.rs

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut map = Vec::new();
    let mut trailheads = Vec::new();
    let mut len_l = 0;
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;

        // all lines must be equal length, and we are being Unicode/UTF8 safe
        let len = line.chars().count();
        if len_l == 0 {
            len_l = line.len();
        } else if len != len_l {
            bail!("Line #{line_num} length {len_l} differs from others. Exit.");
        }

        let mut row = Vec::with_capacity(len);
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                trailheads.push((i as i32, line_num as i32));
            }
            row.push((c.to_digit(10).unwrap_or_default() as u8, 0usize));
        }
        map.push(row);
    }

    debug!("Got map:");
    map_print(&map);
    debug!("Got trailheads: {trailheads:?}");

    let mut total1 = 0;
    for (i, &t) in trailheads.iter().enumerate() {
        let score = trailhead_score(&mut map, t, i + 1);
        debug!("Trailhead #{i} {t:?} score: {score}");
        total1 += score;
    }
    println!("Total1: {}", total1);

    Ok(())
}

#[allow(dead_code)]
fn map_print(map: &[Vec<(u8, usize)>]) {
    for row in map {
        debug!(
            "{}",
            row.iter()
                .flat_map(|p| [char::from_digit(p.0 as u32, 10).unwrap_or_default(), ' '])
                .collect::<String>()
        );
    }
}

#[allow(dead_code)]
fn map_visited(map: &[Vec<(u8, usize)>], trail: usize) {
    for row in map {
        debug!(
            "{}",
            row.iter()
                .flat_map(|p| [if p.1 == trail { '#' } else { '.' }, ' '])
                .collect::<String>()
        );
    }
}

fn trailhead_score(map: &mut [Vec<(u8, usize)>], loc: (i32, i32), trail_id: usize) -> u64 {
    if !on_map(map, loc) {
        return 0;
    }
    let next_val = map[loc.1 as usize][loc.0 as usize].0 + 1;
    map[loc.1 as usize][loc.0 as usize].1 = trail_id;
    if map[loc.1 as usize][loc.0 as usize].0 == 9 {
        return 1;
    }

    let mut score = 0;
    for testloc in [
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
    ] {
        if !on_map(map, testloc) {
            continue;
        }
        let point = map[testloc.1 as usize][testloc.0 as usize];
        if point.0 == next_val && point.1 != trail_id {
            score += trailhead_score(map, testloc, trail_id);
        }
    }
    score
}

#[inline(always)]
fn on_map(map: &[Vec<(u8, usize)>], loc: (i32, i32)) -> bool {
    loc.0 >= 0 && loc.1 >= 0 && loc.0 < map[0].len() as i32 && loc.1 < map.len() as i32
}
// EOF

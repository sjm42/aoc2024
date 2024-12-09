// bin/day08.rs

use std::collections::HashMap;

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut len_l = 0;
    let mut map = Vec::new();
    let mut antennas = HashMap::new();
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;

        // all lines must be equal length, and we are being Unicode/UTF8 safe
        let len = line.chars().count();
        if len_l == 0 {
            len_l = line.len();
        } else if len != len_l {
            bail!("Line {line_num} length {len_l} differs from others. Exit.");
        }

        let line_c = line.chars().collect::<Vec<char>>();
        for (i, &c) in line_c.iter().enumerate() {
            if c.is_alphanumeric() {
                let pos = (i as i64, line_num as i64);
                antennas
                    .entry(c)
                    .and_modify(|e: &mut Vec<(i64, i64)>| e.push(pos))
                    .or_insert_with(|| vec![pos]);
            }
        }
        map.push(line_c);
    }

    debug!("Got map:");
    print_map(&map);
    debug!("Antennas found: {antennas:?}");

    let mut map1 = deepcopy_map(&map);
    mark_antinodes1(&mut map1, &antennas);

    debug!("Map1 with antinodes:");
    print_map(&map1);

    println!("#antinodes1: {}", count_antinodes(&map1));

    let mut map2 = deepcopy_map(&map);
    mark_antinodes2(&mut map2, &antennas);

    debug!("Map2 with antinodes:");
    print_map(&map2);

    println!("#antinodes2: {}", count_antinodes(&map2));

    Ok(())
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        debug!("{}", row.iter().flat_map(|c| [c, &' ']).collect::<String>());
    }
}

fn deepcopy_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut newmap = Vec::with_capacity(map.len());
    for v in map.iter() {
        newmap.push(v.clone());
    }
    newmap
}

fn mark_antinodes1(map: &mut [Vec<char>], antennas: &HashMap<char, Vec<(i64, i64)>>) {
    let limits = (map[0].len() as i64, map.len() as i64);

    for (freq, locs) in antennas.iter() {
        for loc1 in locs.iter() {
            for loc2 in locs.iter() {
                if *loc1 == *loc2 {
                    continue;
                }
                debug!("Vetting {freq} at {loc1:?}-{loc2:?}");

                let (x_d, y_d) = (2 * (loc2.0 - loc1.0), 2 * (loc2.1 - loc1.1));
                let loc = (loc1.0 + x_d, loc1.1 + y_d);
                if valid_loc(limits, loc) {
                    mark_node(map, loc);
                }

                if x_d % 3 == 0 && y_d % 3 == 0 {
                    let loc = (loc1.0 + x_d / 3, loc1.1 + y_d / 3);
                    if valid_loc(limits, loc) {
                        mark_node(map, loc);
                    }
                }
            }
        }
    }
}

fn mark_antinodes2(map: &mut [Vec<char>], antennas: &HashMap<char, Vec<(i64, i64)>>) {
    let limits = (map[0].len() as i64, map.len() as i64);

    for (freq, locs) in antennas.iter() {
        for loc1 in locs.iter() {
            for loc2 in locs.iter() {
                if *loc1 == *loc2 {
                    continue;
                }
                debug!("Vetting {freq} at {loc1:?}-{loc2:?}");

                let mut i = 0;
                loop {
                    let (x_d, y_d) = reduce((loc2.0 - loc1.0, loc2.1 - loc1.1));
                    let loc = (loc1.0 + x_d * i, loc1.1 + y_d * i);
                    if valid_loc(limits, loc) {
                        mark_node(map, loc);
                        i += 1;
                        continue;
                    }
                    break;
                }
            }
        }
    }
}

fn reduce(loc: (i64, i64)) -> (i64, i64) {
    let (mut x, mut y) = loc;
    for i in 2..=x.abs() {
        while x % i == 0 && y % i == 0 {
            x /= i;
            y /= i;
        }
    }
    (x, y)
}

fn valid_loc(limits: (i64, i64), location: (i64, i64)) -> bool {
    location.0 >= 0 && location.0 < limits.0 && location.1 >= 0 && location.1 < limits.1
}

fn mark_node(map: &mut [Vec<char>], location: (i64, i64)) {
    map[location.1 as usize][location.0 as usize] = '#';
}

fn count_antinodes(map: &[Vec<char>]) -> usize {
    let mut cnt = 0;
    map.iter().for_each(|row| {
        cnt += row.iter().filter(|&&c| c == '#').count();
    });
    cnt
}
// EOF

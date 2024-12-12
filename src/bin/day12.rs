// bin/day12.rs

use aoc2024::*;

#[derive(Copy, Clone, Debug)]
struct Point {
    value: char,
    visited: bool,
}

#[derive(Copy, Clone, Debug)]
struct Loc {
    x: i64,
    y: i64,
}

impl Loc {
    fn on_map(&self, map: &[Vec<Point>]) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map[0].len() as i64 && self.y < map.len() as i64
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Neighbor {
    loc: Loc,
    value: char,
}

#[derive(Clone, Debug)]
struct AreaInfo {
    area: u64,
    perimeter: u64,
    neighbors: Vec<Neighbor>,
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut map = Vec::new();
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

        let line_c = line
            .chars()
            .map(|c| Point {
                value: c,
                visited: false,
            })
            .collect::<Vec<_>>();
        map.push(line_c);
    }

    let mut check_neighbors = vec![{
        Neighbor {
            loc: Loc { x: 0, y: 0 },
            value: map[0][0].value,
        }
    }];
    let mut cost = 0;
    while !check_neighbors.is_empty() {
        let mut new_neighbors = Vec::new();

        for n in check_neighbors.iter() {
            if map[n.loc.y as usize][n.loc.x as usize].visited {
                continue;
            }
            let mut info = area_info(&mut map, n.loc);
            cost += info.perimeter * info.area;

            debug!("Check {n:?} found new neighbors: {new_neighbors:?}");
            new_neighbors.append(&mut info.neighbors);
        }
        check_neighbors.clear();

        debug!("Found new neighbors: {new_neighbors:?}");
        check_neighbors.append(&mut new_neighbors);
    }
    println!("Part 1: {cost}");

    Ok(())
}

fn area_info(map: &mut [Vec<Point>], loc: Loc) -> AreaInfo {
    let mut area = 1;
    let mut perimeter = 0;
    let mut neighbors = Vec::new();

    map[loc.y as usize][loc.x as usize].visited = true;

    for test_loc in [
        Loc {
            x: loc.x - 1,
            y: loc.y,
        },
        Loc {
            x: loc.x + 1,
            y: loc.y,
        },
        Loc {
            x: loc.x,
            y: loc.y - 1,
        },
        Loc {
            x: loc.x,
            y: loc.y + 1,
        },
    ] {
        if !test_loc.on_map(map) {
            perimeter += 1;
            continue;
        }

        let test_point = map[test_loc.y as usize][test_loc.x as usize];
        if test_point.value == map[loc.y as usize][loc.x as usize].value {
            if test_point.visited {
                continue;
            }
            let mut more_info = area_info(map, test_loc);
            neighbors.append(more_info.neighbors.as_mut());
            area += more_info.area;
            perimeter += more_info.perimeter;
        } else {
            perimeter += 1;
            if !test_point.visited {
                neighbors.push(Neighbor {
                    loc: test_loc,
                    value: test_point.value,
                });
            }
        }
    }

    debug!("Point at {loc:?} found neighbors: {neighbors:?}");
    AreaInfo {
        area,
        perimeter,
        neighbors,
    }
}
// EOF

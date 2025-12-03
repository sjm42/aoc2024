// bin/day15.rs

use aoc2024::*;

#[derive(Clone, Debug)]
struct Map {
    size_x: i64,
    size_y: i64,
    map: Vec<Vec<char>>,
    robot_loc: (i64, i64),
}

impl Map {
    fn on_map(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.size_x && pos.1 < self.size_y
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let robot_loc = self.robot_loc;
        // Admittedly, it's a bit evil to make here a temporary copy of the whole thing.
        let mut map_tmp = self.map.clone();
        map_tmp[robot_loc.1 as usize][robot_loc.0 as usize] = '@';
        writeln!(f, "MAP size ({},{})", self.size_x, self.size_y)?;
        writeln!(f, "robot at ({},{})", self.robot_loc.0, self.robot_loc.1)?;
        for row in map_tmp.iter() {
            for c in row.iter() {
                write!(f, "{c} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Mov {
    Up,
    Right,
    Down,
    Left,
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut map_data = Vec::new();
    let mut len_l = 0;
    let mut read_map = true;
    let mut robot_pos = None;
    let mut map = None;
    let mut moves = Vec::new();

    // read input data
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if robot_pos.is_none() {
                bail!("Robot location not found");
            }
            let size_x = len_l as i64;
            let size_y = map_data.len() as i64;
            map = Some(Map {
                size_x,
                size_y,
                map: map_data.clone(),
                robot_loc: robot_pos.unwrap(),
            });

            read_map = false;
            continue;
        }

        if read_map {
            // all lines must be equal length, and we are being Unicode/UTF8 safe
            let len = line.chars().count();
            if len_l == 0 {
                len_l = line.len();
            } else if len != len_l {
                bail!("Line #{line_num} length {len_l} differs from others. Exit.");
            }
            let mut line_c = line.chars().collect::<Vec<_>>();
            if let Some((i, _)) = line_c.iter().enumerate().find(|(_, c)| **c == '@') {
                if robot_pos.is_some() {
                    bail!("Robot position already found. Exit.");
                }

                debug!("Found robot at ({i},{line_num})");
                line_c[i] = '.';
                robot_pos = Some((i as i64, line_num as i64));
            }
            map_data.push(line_c);
        } else {
            // read robot moves
            for (i, c) in line.chars().enumerate() {
                let mov = match c {
                    '^' => Mov::Up,
                    '>' => Mov::Right,
                    'v' => Mov::Down,
                    '<' => Mov::Left,
                    _ => bail!("Invalid move on line# {line_num} at position {i} char: {c}"),
                };
                moves.push(mov);
            }
        }
    }
    drop(map_data);

    if map.is_none() {
        bail!("No map. Cannot continue.");
    }
    let map = map.unwrap();

    debug!("Got map: {map}");
    debug!("Got moves: {moves:?}");

    Ok(())
}
// EOF

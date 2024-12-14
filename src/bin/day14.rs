// bin/day14.rs

use aoc2024::*;

const RE_INPUT: &str = r"^p=(\d+),\s*(\d+)\s+v=(-?\d+),\s*(-?\d+)$";

// const SIZE_X: i64 = 11;
// const SIZE_Y: i64 = 7;

const SIZE_X: i64 = 101;
const SIZE_Y: i64 = 103;

const STEPS: usize = 100;

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn sum(&self, pos2: &Self) -> Self {
        Self {
            x: self.x + pos2.x,
            y: self.y + pos2.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

impl Robot {
    fn new(v: &[i64]) -> anyhow::Result<Self> {
        if v.len() < 4 {
            anyhow::bail!("Robot::new(): invalid vec length");
        }
        Ok(Self {
            pos: Pos { x: v[0], y: v[1] },
            vel: Pos { x: v[2], y: v[3] },
        })
    }

    fn step(&mut self) {
        let mut pos_new = self.pos.sum(&self.vel);
        if pos_new.x < 0 {
            pos_new.x += SIZE_X;
        } else if pos_new.x >= SIZE_X {
            pos_new.x %= SIZE_X;
        }
        if pos_new.y < 0 {
            pos_new.y += SIZE_Y;
        } else if pos_new.y >= SIZE_Y {
            pos_new.y %= SIZE_Y;
        }
        self.pos = pos_new;
    }
}

#[derive(Clone, Debug)]
struct Map {
    size_x: i64,
    size_y: i64,
    map: Vec<Vec<i64>>,
}
impl Map {
    fn new(size_x: i64, size_y: i64) -> Self {
        let map = vec![vec![0; size_x as usize]; size_y as usize];
        Self {
            size_x,
            size_y,
            map,
        }
    }

    fn on_map(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size_x && pos.y < self.size_y
    }

    fn with_robots(mut self, robots: &[Robot]) -> Self {
        for r in robots {
            if !self.on_map(r.pos) {
                continue;
            }
            self.map[r.pos.y as usize][r.pos.x as usize] += 1;
        }
        self
    }

    fn safety_factor(&self) -> i64 {
        let (szx, szy) = (self.size_x as usize, self.size_y as usize);
        if self.size_x % 2 == 0 || self.size_y % 2 == 0 {
            // we only deal with odd sizes here, sorry
            return 0;
        }
        let (xhalf, yhalf) = (szx / 2, szy / 2);
        let q1 = self.map[0..yhalf]
            .iter()
            .map(|r| r[xhalf + (szx % 2)..szx].iter().sum::<i64>())
            .sum::<i64>();
        let q2 = self.map[0..yhalf]
            .iter()
            .map(|r| r[0..xhalf].iter().sum::<i64>())
            .sum::<i64>();
        let q3 = self.map[yhalf + (szy % 2)..szy]
            .iter()
            .map(|r| r[0..xhalf].iter().sum::<i64>())
            .sum::<i64>();
        let q4 = self.map[yhalf + (szy % 2)..szy]
            .iter()
            .map(|r| r[xhalf + (szx % 2)..szx].iter().sum::<i64>())
            .sum::<i64>();
        q1 * q2 * q3 * q4
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size_y as usize {
            for x in 0..self.size_x as usize {
                let v = self.map[y][x] % 10;
                if v == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{v} ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let re_input = regex::Regex::new(RE_INPUT)?;
    let mut robots = Vec::new();
    for (line_num, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = re_input.captures(line) {
            let nums = caps
                .iter()
                .skip(1)
                .filter(|c| c.is_some())
                .map(|s| s.unwrap().as_str().parse::<i64>().unwrap_or_default())
                .collect::<Vec<_>>();
            // debug!("Found {nums:?}");
            if nums.len() == 4 {
                debug!("nums: {nums:?}");
                robots.push(Robot::new(&nums)?);
                continue;
            }
        }
        bail!("Invalid input on line #{line_num}: \"{line}\"");
    }
    debug!("Got robots: {robots:?}");

    let map_empty = Map::new(SIZE_X, SIZE_Y);
    debug!("Empty map:\n{map_empty}");

    let mut robots_test = robots.clone();
    for i in 0..=2 {
        let map = Map::new(SIZE_X, SIZE_Y).with_robots(&robots_test);
        debug!("Robots after {i} steps:\n{map}");
        for r in robots_test.iter_mut() {
            r.step();
        }
    }

    for _ in 0..STEPS {
        for r in robots.iter_mut() {
            r.step();
        }
    }
    let map = Map::new(SIZE_X, SIZE_Y).with_robots(&robots);
    debug!("Robots after {STEPS} steps:\n{map}");
    println!("Safety factor: {}", map.safety_factor());

    Ok(())
}
// EOF

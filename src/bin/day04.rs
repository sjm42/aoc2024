// bin/day04.rs

use aoc2024::*;

const WORDS: [&str; 1] = ["XMAS"];
const WORDS2: [&str; 1] = ["MAS"];

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    let mut words = Vec::new();
    for w in WORDS {
        words.push(w.to_owned());
        words.push(w.chars().rev().collect::<String>());
    }

    // read input data
    let mut lines_s = Vec::new();
    let mut lines_c = Vec::new();
    let mut len_l = 0;
    for (line_n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let line_t = line.trim().to_owned();

        // all lines must be equal length, and we are being Unicode/UTF8 safe
        let len = line_t.chars().count();
        if len_l == 0 {
            len_l = line_t.len();
        } else if len != len_l {
            bail!("Line {line_n} length {len_l} differs from others. Exit.");
        }

        lines_c.push(line_t.chars().collect::<Vec<char>>());
        lines_s.push(line_t);
    }
    let lines_n = lines_s.len();
    info!("Read {lines_n} lines, length {len_l}");

    // find matches, part one
    let mut n_found: usize = 0;
    for word in words.iter() {
        info!("*** Checking word {word}");
        let len_w = word.chars().count();

        // find horizontal matches
        for (line_n, line) in lines_s.iter().enumerate() {
            let cnt = line.match_indices(word).count();
            if cnt > 0 {
                debug!("Line #{line_n} matches ({word}): {cnt}");
            }
            n_found += cnt;
        }

        // find vertical matches
        for line_idx in 0..(lines_n + 1 - len_w) {
            debug!("Checking vertical line #{line_idx}");

            for x in 0..len_l {
                let mut found = true;
                for (i, c) in word.chars().enumerate() {
                    if lines_c[line_idx + i][x] != c {
                        found = false;
                        break;
                    }
                }
                if found {
                    debug!("Found vertical word {word} line {line_idx} pos {x}");
                    n_found += 1;
                }
            }
        }

        // find diagonal matches
        for line_idx in 0..(lines_n + 1 - len_w) {
            debug!("Checking (+)diagonal line #{line_idx}");

            for x in 0..(len_l + 1 - len_w) {
                let mut found = true;
                for (i, c) in word.chars().enumerate() {
                    if lines_c[line_idx + i][x + i] != c {
                        found = false;
                        break;
                    }
                }
                if found {
                    debug!("Found (+)diagonal word {word} line {line_idx} pos {x}");
                    n_found += 1;
                }
            }
        }
        for line_idx in 0..(lines_n + 1 - len_w) {
            debug!("Checking (-)diagonal line #{line_idx}");

            for x in (len_w - 1)..len_l {
                let mut found = true;
                for (i, c) in word.chars().enumerate() {
                    if lines_c[line_idx + i][x - i] != c {
                        found = false;
                        break;
                    }
                }
                if found {
                    debug!("Found (-)diagonal word {word} line {line_idx} pos {x}");
                    n_found += 1;
                }
            }
        }
    }
    println!("Found1: {n_found}");

    // find matches, part two
    n_found = 0;
    for word in WORDS2 {
        if word.len() % 2 == 0 {
            error!("Invalid word \"{word}\": length must be odd!");
            continue;
        }
        let word_rs = word.chars().rev().collect::<String>();
        let word_r = word_rs.as_str();

        let wrad = word.len() / 2;
        debug!("Checking for X-word {word}");

        // let mut rev = None;
        for line_idx in (wrad)..(lines_n + 1 - wrad) {
            let mut found = true;
            // TODO
        }
    }
    Ok(())
}
// EOF

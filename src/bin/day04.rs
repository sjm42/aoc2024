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
    let mut n_found1: usize = 0;
    for word in words.iter() {
        info!("*** Checking word {word}");
        let len_w = word.chars().count();

        // find horizontal matches
        for (line_n, line) in lines_s.iter().enumerate() {
            let cnt = line.match_indices(word).count();
            if cnt > 0 {
                debug!("Line #{line_n} matches ({word}): {cnt}");
            }
            n_found1 += cnt;
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
                    debug!("Found vertical word {word} ({line_idx},{x})");
                    n_found1 += 1;
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
                    debug!("Found (+)diagonal word {word} ({line_idx},{x})");
                    n_found1 += 1;
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
                    debug!("Found (-)diagonal word {word} ({line_idx},{x})");
                    n_found1 += 1;
                }
            }
        }
    }
    println!("Found1: {n_found1}");

    // find matches, part two
    let mut n_found2: usize = 0;
    for word in WORDS2 {
        let len_w = word.len();
        if len_w % 2 == 0 {
            error!("Invalid word \"{word}\": length must be odd!");
            continue;
        }
        let word_c = word.chars().collect::<Vec<char>>();
        let word_rc = word.chars().rev().collect::<Vec<char>>();

        // let wrad = word.len() / 2;
        info!("Checking for X-word {word} -- {word_c:?} rev {word_rc:?}");

        for line_idx in 0..(lines_n + 1 - len_w) {
            for x in 0..(len_l + 1 - len_w) {
                let mut found = true;
                let mut rev_a = None;
                let mut rev_b = None;

                for i in 0..len_w {
                    // detect first diagonal
                    debug!("Check#{i}");
                    let (i1, i2) = (line_idx + i, x + i);
                    match rev_a {
                        None => {
                            debug!("Diagonal(+) start checking at ({i1},{i2})");
                            if lines_c[i1][i2] == word_c[i] {
                                debug!("found(+) fwd {} at ({i1},{i2})", word_c[i],);
                                rev_a = Some(false);
                            } else if lines_c[i1][i2] == word_rc[i] {
                                debug!("found(+) rev {} at ({i1},{i2})", word_rc[i],);
                                rev_a = Some(true);
                            } else {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                        }
                        Some(false) => {
                            if lines_c[i1][i2] != word_c[i] {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                            debug!("fwd(+)[{}] match {} at ({i1},{i2})", i, word_c[i],);
                        }
                        Some(true) => {
                            if lines_c[i1][i2] != word_rc[i] {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                            debug!("rev(+)[{}] match {} at ({i1},{i2})", i, word_rc[i],);
                        }
                    }

                    // detect second diagonal
                    let (i1, i2) = (line_idx + i, x + (len_w - 1) - i);
                    match rev_b {
                        None => {
                            debug!("Diagonal(-) start checking at ({i1},{i2})");
                            if lines_c[i1][i2] == word_c[i] {
                                debug!("found(-) fwd {} at ({i1},{i2})", word_c[i],);
                                rev_b = Some(false);
                            } else if lines_c[i1][i2] == word_rc[i] {
                                debug!("found(-) rev {} at ({i1},{i2})", word_rc[i],);
                                rev_b = Some(true);
                            } else {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                        }
                        Some(false) => {
                            if lines_c[i1][i2] != word_c[i] {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                            debug!("fwd(-)[{}] match {} at ({i1},{i2})", i, word_c[i],);
                        }
                        Some(true) => {
                            if lines_c[i1][i2] != word_rc[i] {
                                debug!("check#{i} fail at ({i1},{i2})");
                                found = false;
                                break;
                            }
                            debug!("rev(-)[{}] match {} at ({i1},{i2})", i, word_rc[i],);
                        }
                    }
                }
                if found {
                    debug!("Found X at ({line_idx},{x})");
                    n_found2 += 1;
                }
            }
        }
    }
    println!("Found2: {n_found2}");
    Ok(())
}
// EOF

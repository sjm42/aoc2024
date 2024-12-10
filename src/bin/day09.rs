// bin/day09.rs

use aoc2024::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.finalize()?;
    opts.start_pgm(env!("CARGO_BIN_NAME"));

    // read input data
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let data = line.chars().collect::<Vec<char>>();
    let len = data
        .iter()
        .map(|&i| i.to_digit(10).unwrap_or_default() as usize)
        .sum();

    info!("Got data ({} chars) blocks: {len})", data.len());
    // debug!("Data: {data:?}");

    let mut blocks1 = Vec::with_capacity(len);
    let mut blocks2: Vec<(u32, Option<usize>)> = Vec::with_capacity(data.len());
    for (i, &c) in data.iter().enumerate() {
        let n = c.to_digit(10).unwrap_or_default();
        if n == 0 {
            // not appending zero-size blocks
            continue;
        }

        let value = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in 0..n {
            blocks1.push(value);
        }
        if value.is_none() && !blocks2.is_empty() && blocks2.last().unwrap().1.is_none() {
            blocks2.last_mut().unwrap().0 += n;
        } else {
            blocks2.push((n, value));
        }
    }
    debug!("Blocks1[{}]:", blocks1.len());
    // disk_print1(&blocks1);
    disk_defrag1(&mut blocks1);
    // disk_print1(&blocks1);
    println!("Defrag1 checksum: {}", disk_checksum1(&blocks1));

    debug!("Blocks2[{}]", blocks2.len());
    // disk_print2(&blocks2);
    disk_defrag2(&mut blocks2);
    // disk_print2(&blocks2);
    println!("Defrag2 checksum: {}", disk_checksum2(&blocks2));

    Ok(())
}

#[allow(dead_code)]
fn disk_print1(disk: &[Option<usize>]) {
    let dump = disk
        .iter()
        .map(|r| r.unwrap_or(usize::MAX))
        .map(|n| {
            if n < usize::MAX {
                (0x30 + (n % 10) as u8) as char
            } else {
                '.'
            }
        })
        .collect::<String>();
    println!("{dump}");
}

#[allow(dead_code)]
fn disk_print2(disk: &[(u32, Option<usize>)]) {
    let dump = disk
        .iter()
        .map(|b| match b.1 {
            None => '.'.to_string(),
            Some(id) => format!("[{}x{:x}]", b.0, id),
        })
        .collect::<String>();
    println!("{dump}");
}

fn disk_defrag1(disk: &mut [Option<usize>]) {
    let len = disk.len();
    let mut last_nonempty = len;
    for i in 0..len {
        if i >= last_nonempty {
            break;
        }

        if disk[i].is_none() {
            for j in (i..last_nonempty).rev() {
                if disk[j].is_some() {
                    disk[i] = disk[j].take();
                    last_nonempty = j;
                    break;
                }
            }
            // disk_print1(disk);
        }
    }
}

fn disk_defrag2(disk: &mut Vec<(u32, Option<usize>)>) {
    let len = disk.len();
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        loop {
            let mut free = 0;
            if disk[i].1.is_none() {
                for j in (i..len).rev() {
                    if disk[j].1.is_some() && disk[i].0 >= disk[j].0 {
                        result.push(disk[j]);
                        free = disk[i].0 - disk[j].0;
                        disk[i].0 = free;
                        disk[j].1 = None;
                        break;
                    }
                }
            } else {
                result.push(disk[i]);
            }
            if free == 0 {
                break;
            }
        }
        if disk[i].0 > 0 && disk[i].1.is_none() {
            result.push(disk[i]);
        }
    }
    disk.clear();
    disk.append(&mut result);
}

fn disk_checksum1(disk: &[Option<usize>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|x| (x.0 * x.1.unwrap()) as u64)
        .sum()
}

fn disk_checksum2(disk: &[(u32, Option<usize>)]) -> u64 {
    let mut idx = 0;
    let mut sum = 0;
    for b in disk.iter() {
        if let Some(id) = b.1 {
            sum += (0..b.0).map(|j| id as u32 * (idx + j)).sum::<u32>() as u64;
        }
        idx += b.0;
    }
    sum
}
// EOF

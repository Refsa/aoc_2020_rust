use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn aoc_6(reader: BufReader<File>) -> String {
    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    let sum: u32 = lines
        .split(|l| l.is_empty())
        .map(|g| {
            let mut group_count: [u32; 255] = [0; 255];
            g.iter()
                .map(|s| s.bytes())
                .for_each(|bs| bs.for_each(|b| group_count[b as usize] += 1));
            (g.len() as u32, group_count)
        })
        .map(|d| d.1.iter().filter(|v| **v == d.0).count() as u32)
        .sum();

    format!("{}", sum)
}

use std::fs::File;
use std::io::prelude::*;
use std::str::Chars;
use std::{io::BufReader};

pub fn aoc_5(reader: BufReader<File>) -> String {
    let mut seat_ids: Vec<u32> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (row, col) = l.split_at(7);
            let row = bsp_search(row.chars(), (0, 127));
            let col = bsp_search(col.chars(), (0, 7));
            row * 8 + col
        })
        .collect();

    seat_ids.sort();

    let max = seat_ids.last().unwrap();
    let my_seat = seat_ids.windows(2).find(|c| c[1] - c[0] != 1).unwrap()[0] + 1;

    format!("{}\n\t{:?}", max, my_seat)
}

fn bsp_search(mut seq: Chars, mut range: (u32, u32)) -> u32 {
    match seq.nth(0) {
        Some(c) => {
            let mid = (range.0 + range.1) / 2;
            match c {
                'F' | 'L' => range.1 = mid,
                'B' | 'R' => range.0 = mid,
                _ => (),
            }
            bsp_search(seq, range)
        }
        None => range.1,
    }
}

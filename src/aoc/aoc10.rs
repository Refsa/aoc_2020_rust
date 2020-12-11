use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    thread::current,
};

const WINDOW_SIZE: usize = 25;
const RANGE: RangeInclusive<i16> = 1..=3;

pub fn aoc_10(reader: BufReader<File>) -> String {
    let mut lines: Vec<i16> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i16>().unwrap())
        .collect();

    lines.sort();

    let ones_threes = lines
        .windows(2)
        .map(|v| v[1] - v[0])
        .fold(add_counter(lines[0], (0, 1)), |a, v| add_counter(v, a));

    // Part 2
    // lines.reverse();
    let mut paths = Vec::new();
    for i in 0..lines.len() {
        let current = lines[i];
        let mut vec = Vec::new();

        for j in i + 1..lines.len() {
            let next = lines[j] - current;
            if next >= 1 && next <= 3 {
                vec.push(j);
            }
        }
        println!("{} : {:?}", i, vec);
        paths.push(vec);
    }

    // paths.reverse();
    // let mut permutes = 1usize;
    // for i in 0..lines.len() - 1 {
        // if paths[i].len() > 1 {
            // permutes += paths[i].len();
        // }
// 
        // println!("{} - {:?} - {}", i, paths[i], permutes);
    // }

    let permutes = count_paths(&paths, 0);

    format!("P1: {}\n\t P2: {}", ones_threes.0 * ones_threes.1, permutes)
}

fn count_paths(paths: &[Vec<usize>], index: usize) -> usize {
    if index == paths.len() - 1 {
        return 1;
    }

    let mut sum = 0;
    for path in &paths[index] {
        sum += count_paths(paths, *path);
    }
    sum
}

#[inline]
fn add_counter(diff: i16, mut counter: (u16, u16)) -> (u16, u16) {
    match diff {
        1 => counter.0 += 1,
        3 => counter.1 += 1,
        _ => (),
    }
    counter
}
